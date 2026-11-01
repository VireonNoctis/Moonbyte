use std::{process::ExitCode, time::Duration};

use moonbyte::{Data, Error, commands, config::Config, database::Database, game::GameRuntime};
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
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(prefix.clone().into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            let prefix = prefix.clone();
            let games = games.clone();
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::new(prefix, games))
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
