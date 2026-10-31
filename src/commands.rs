//! Discord-native commands available in the initial Moonbyte runtime.

use crate::{Data, Error};

type Context<'a> = poise::Context<'a, Data, Error>;

/// Show the commands currently implemented by Moonbyte.
#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(help_message(ctx.data().prefix())).await?;
    Ok(())
}

/// Describe the current Moonbyte build and its implementation boundary.
#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let message = format!(
        "Moonbyte v{} — a little world of games, right inside Discord.\n\n\
         This early runtime provides help, about, ping, and a privacy overview. \
         Playable games, sessions, profiles, and SQLite persistence are not connected yet.",
        env!("CARGO_PKG_VERSION")
    );
    ctx.say(message).await?;
    Ok(())
}

/// Report that the command runtime is responding and show its process uptime.
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!(
        "Moonbyte is online. Process uptime: {} seconds.",
        ctx.data().uptime().as_secs()
    ))
    .await?;
    Ok(())
}

/// Explain the current runtime's data handling and unavailable privacy controls.
#[poise::command(prefix_command, slash_command)]
pub async fn privacy(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(PRIVACY_OVERVIEW).await?;
    Ok(())
}

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![help(), about(), ping(), privacy()]
}

fn help_message(prefix: &str) -> String {
    let safe_prefix = prefix
        .replace('@', "@\u{200B}")
        .replace('\x60', "ˋ")
        .replace('\\', "＼");

    format!(
        "**Moonbyte commands**\n\
         Configured prefix: {safe_prefix}\n\n\
         {safe_prefix}help / /help — list implemented commands\n\
         {safe_prefix}about / /about — describe this build\n\
         {safe_prefix}ping / /ping — check command responsiveness and process uptime\n\
         {safe_prefix}privacy / /privacy — read the current privacy overview\n\n\
         Game, profile, economy, and data-control commands are not available yet."
    )
}

const PRIVACY_OVERVIEW: &str = "\
**Moonbyte privacy — current build**\n\n\
This runtime receives Discord command interactions so it can answer commands. Prefix \
commands are parsed in memory. The current build does not connect to SQLite or persist \
player profiles, sessions, or game records. No advertising analytics or AI integration \
is installed.\n\n\
Discord independently processes interactions under its own policies. Moonbyte cannot \
control Discord's retention. This project does not claim end-to-end encryption or \
database encryption.\n\n\
**Not implemented yet:** personal data inventory, profile export, and confirmed \
deletion. This overview is not a deletion request. Do not post private personal data \
in public channels.";

#[cfg(test)]
mod tests {
    use super::help_message;

    #[test]
    fn help_lists_only_the_current_command_surface() {
        let message = help_message(".");

        for command in ["help", "about", "ping", "privacy"] {
            assert!(message.contains(&format!(".{command} / /{command}")));
        }
        assert!(
            message.contains(
                "Game, profile, economy, and data-control commands are not available yet."
            )
        );
    }

    #[test]
    fn help_safely_displays_custom_prefixes() {
        let message = help_message("@everyone\x60");

        assert!(message.contains("@\u{200B}everyoneˋ"));
        assert!(!message.contains("@everyone"));
    }
}
