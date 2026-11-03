use std::{process::ExitCode, time::Duration};

use moonbyte::{
    Data, Error, achievement::AchievementStore, atlas::AtlasStore, commands, config::Config,
    consent::ConsentStore, database::Database, game::GameRuntime,
    guild_settings::GuildSettingsStore, profile::ProfileStore, wallet::WalletStore,
};
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Moonbyte startup/runtime error: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Error> {
    let config = Config::from_environment()?;
    let database = Database::connect(config.database_url()).await?;
    let games = GameRuntime::new(database.pool());
    let profiles = ProfileStore::new(database.pool());
    let wallets = WalletStore::new(database.pool());
    let achievements = AchievementStore::new(database.pool());
    let atlas = AtlasStore::new(database.pool());
    let guild_settings = GuildSettingsStore::new(database.pool()).await?;
    let consents = ConsentStore::new(database.pool());
    let prefix = config.default_prefix().to_owned();

    // One central cleanup task enforces the 24-hour inactive-session retention
    // window without polling each individual session.
    let cleanup_database = database.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(15 * 60));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            interval.tick().await;
            if cleanup_database.remove_expired_sessions().await.is_err() {
                eprintln!("Moonbyte: PostgreSQL expired-session cleanup failed.");
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
                    if let serenity::FullEvent::GuildDelete { incomplete, .. } = event {
                        // Temporary outages also emit GuildDelete; retain guild data for those.
                        if !incomplete.unavailable {
                            let cleanup = framework
                                .user_data
                                .guild_settings()
                                .remove_guild_data(&incomplete.id.to_string())
                                .await?;
                            eprintln!(
                                "Moonbyte: removed {} command settings and {} active sessions from a departed guild.",
                                cleanup.command_settings_removed,
                                cleanup.sessions_removed
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
            let games = games.clone();
            let profiles = profiles.clone();
            let wallets = wallets.clone();
            let achievements = achievements.clone();
            let atlas = atlas.clone();
            let guild_settings = guild_settings.clone();
            let consents = consents.clone();
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::new(
                    prefix,
                    games,
                    profiles,
                    wallets,
                    achievements,
                    atlas,
                    guild_settings,
                    consents,
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

    client.start().await?;
    Ok(())
}
