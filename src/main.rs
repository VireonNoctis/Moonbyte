use std::{process::ExitCode, time::Duration};

use moonbyte::{
    Data, DataServices, Error, achievement::AchievementStore, atlas::AtlasStore,
    chronicle::ChronicleStore, commands, config::Config, consent::ConsentStore, database::Database,
    expedition::ExpeditionStore, game::GameRuntime, guild_settings::GuildSettingsStore,
    inventory::InventoryStore, profile::ProfileStore, wallet::WalletStore, wayfarer::WayfarerStore,
};
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> ExitCode {
    if init_tracing().is_err() {
        eprintln!("Moonbyte: structured logging could not be initialized.");
        return ExitCode::FAILURE;
    }

    tracing::info!(event = "process_starting");
    match run().await {
        Ok(()) => {
            tracing::info!(event = "process_stopped");
            ExitCode::SUCCESS
        }
        Err(_) => {
            // Avoid printing raw errors: some dependency errors can include connection
            // details or other values that should not reach operational logs.
            tracing::error!(
                event = "runtime_failed",
                "Moonbyte startup or runtime failed."
            );
            ExitCode::FAILURE
        }
    }
}

fn init_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("moonbyte=info,serenity=warn,poise=info,sqlx=warn"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .try_init()?;

    Ok(())
}

async fn run() -> Result<(), Error> {
    tracing::info!(event = "configuration_loading");
    let config = Config::from_environment()?;
    let database = Database::connect(config.database_url()).await?;
    tracing::info!(event = "database_ready");
    let games = GameRuntime::new(database.pool());
    let profiles = ProfileStore::new(database.pool());
    let wallets = WalletStore::new(database.pool());
    let achievements = AchievementStore::new(database.pool());
    let atlas = AtlasStore::new(database.pool());
    let chronicle = ChronicleStore::new(database.pool());
    let guild_settings = GuildSettingsStore::new(database.pool()).await?;
    let consents = ConsentStore::new(database.pool());
    let wayfarer = WayfarerStore::new(database.pool());
    let expeditions = ExpeditionStore::new(database.pool());
    let inventory = InventoryStore::new(database.pool());
    let prefix = config.default_prefix().to_owned();

    // One central cleanup task enforces the 24-hour inactive-session retention
    // window without polling each individual session.
    let cleanup_database = database.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(15 * 60));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            interval.tick().await;
            match cleanup_database.remove_expired_sessions().await {
                Ok(removed) if removed > 0 => {
                    tracing::info!(event = "expired_sessions_removed", rows = removed);
                }
                Ok(_) => {}
                Err(_) => {
                    tracing::warn!(event = "expired_session_cleanup_failed");
                }
            }
        }
    });

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            command_check: Some(|ctx| {
                Box::pin(async move {
                    let command_name = ctx.command().name.as_ref();
                    let legal_or_privacy_command = matches!(
                        command_name,
                        "terms" | "privacy" | "privacy_data" | "privacy_export"
                            | "privacy_delete" | "consent"
                    );

                    // Legal documents and privacy controls remain available before acceptance.
                    if !legal_or_privacy_command {
                        let player_id = ctx.author().id.to_string();
                        if !ctx.data().consents().is_current(&player_id).await? {
                            let prefix = ctx
                                .data()
                                .command_prefix(ctx.guild_id())
                                .replace('@', "@\u{200B}")
                                .replace(char::from(96), "ˋ")
                                .replace(char::from(92), "＼");
                            ctx.send(
                                poise::CreateReply::new()
                                    .content(format!(
                                        "Before you continue, please review the Terms of Service and Privacy Policy.\n\n{prefix}terms — read the Terms of Service\n{prefix}privacy — read the full Privacy Policy\n\nAfter reviewing both, run {prefix}consent accept to agree.\nAcceptance is saved against these document versions. Material policy updates require acceptance again.\n\nYour privacy commands remain available without accepting."
                                    ))
                                    .ephemeral(true),
                            )
                            .await?;
                            return Ok(false);
                        }
                    }

                    // Legal/privacy access and prefix configuration remain recoverable from any channel.
                    if legal_or_privacy_command || command_name == "prefix" {
                        return Ok(true);
                    }

                    let Some(guild_id) = ctx.guild_id() else {
                        return Ok(true);
                    };
                    let Some(settings) = ctx.data().guild_settings().get(&guild_id.to_string()) else {
                        return Ok(true);
                    };
                    let Some(channel_id) = settings.command_channel_id else {
                        return Ok(true);
                    };

                    if channel_id == ctx.channel_id().to_string() {
                        return Ok(true);
                    }

                    ctx.say(format!(
                        "Commands for this server are configured in <#{channel_id}>. Use /prefix there or from any channel to update the setting."
                    ))
                    .await?;
                    Ok(false)
                })
            }),
            event_handler: |framework, event| {
                Box::pin(async move {
                    if let serenity::FullEvent::InteractionCreate {
                        interaction: serenity::Interaction::Component(component),
                    } = event
                    {
                        commands::handle_game_component(
                            framework.serenity_context,
                            framework.user_data,
                            component,
                        )
                        .await?;
                    }

                    if let serenity::FullEvent::GuildDelete { incomplete, .. } = event {
                        // Temporary outages also emit GuildDelete; retain guild data for those.
                        if !incomplete.unavailable {
                            let cleanup = framework
                                .user_data
                                .guild_settings()
                                .remove_guild_data(&incomplete.id.to_string())
                                .await?;
                            tracing::info!(
                                event = "guild_data_cleaned",
                                command_settings_removed = cleanup.command_settings_removed,
                                active_sessions_removed = cleanup.sessions_removed
                            );
                        }
                    }
                    Ok(())
                })
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: None,
                dynamic_prefix: Some(|partial| {
                    Box::pin(async move {
                        let data = partial.framework.user_data;
                        let settings = partial
                            .guild_id
                            .and_then(|guild_id| data.guild_settings().get(&guild_id.to_string()));
                        let prefix = settings
                            .map(|settings| settings.prefix)
                            .unwrap_or_else(|| data.prefix().to_owned());
                        Ok(Some(prefix.into()))
                    })
                }),
                mention_as_prefix: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            let prefix = prefix.clone();
            let database = database.clone();
            let games = games.clone();
            let profiles = profiles.clone();
            let wallets = wallets.clone();
            let achievements = achievements.clone();
            let atlas = atlas.clone();
            let guild_settings = guild_settings.clone();
            let consents = consents.clone();
            let wayfarer = wayfarer.clone();
            let expeditions = expeditions.clone();
            let inventory = inventory.clone();
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::new(
                    prefix,
                    DataServices {
                        database,
                        games,
                        profiles,
                        wallets,
                        achievements,
                        atlas,
                        chronicle,
                        guild_settings,
                        consents,
                        wayfarer,
                        expeditions,
                        inventory,
                    },
                ))
            })
        })
        .build();

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(config.discord_token(), intents)
        .framework(framework)
        .await?;

    tracing::info!(event = "discord_gateway_starting");
    let shard_manager = client.shard_manager.clone();
    tokio::select! {
        result = client.start() => {
            result?;
            tracing::info!(event = "discord_gateway_stopped");
        }
        signal = shutdown_signal() => {
            signal?;
            tracing::info!(event = "shutdown_signal_received");
            shard_manager.shutdown_all().await;
            tracing::info!(event = "discord_shards_closed");
        }
    }

    Ok(())
}

/// Wait for the platform's normal process-termination signal.
async fn shutdown_signal() -> Result<(), std::io::Error> {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => result,
            _ = terminate.recv() => Ok(()),
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await
    }
}
