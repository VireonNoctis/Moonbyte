//! Discord-native commands. Handlers adapt user input to application services;
//! the game module remains independent of Serenity and Poise.

use crate::{
    Data, Error,
    game::{GameKind, GamePhase},
    profile::BoardTheme,
};

type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(help_message(ctx.data().prefix())).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let message = format!(
        "Moonbyte v{} — a colourful world of games inside Discord.\n\n\
         This build includes four playable games, guild-scoped multiplayer lobbies, \
         turn validation, private session reads and explicit leave controls. Sessions \
         are persisted in PostgreSQL and expire after 24 hours of inactivity. One cosmetic \
         player-profile preference is stored; profile statistics, progression, Vaults \
         and the Bazaar are not connected yet.",
        env!("CARGO_PKG_VERSION")
    );
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!(
        "Moonbyte is online. Process uptime: {} seconds.",
        ctx.data().uptime().as_secs()
    ))
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn privacy(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(PRIVACY_OVERVIEW).await?;
    Ok(())
}


#[poise::command(prefix_command, slash_command)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "Optional theme: classic, forest, sunset, or lavender"] theme: Option<String>,
) -> Result<(), Error> {
    let user_id = ctx.author().id.to_string();
    let repository = ctx.data().profiles();

    let saved_theme = match theme {
        Some(value) if value.trim().eq_ignore_ascii_case("delete") => {
            ctx.say(
                "This removes only your saved cosmetic theme preference. To confirm, run `.profile delete confirm`. Your active and completed session records are not deleted.",
            )
            .await?;
            return Ok(());
        }
        Some(value) if value.trim().eq_ignore_ascii_case("delete confirm") => {
            let deleted = repository.delete_profile(&user_id).await?;
            let result = if deleted {
                "Your saved cosmetic profile preference has been deleted. Session records are not affected."
            } else {
                "You did not have a saved profile preference to delete."
            };
            ctx.say(result).await?;
            return Ok(());
        }
        Some(value) => {
            let Some(theme) = BoardTheme::parse(&value) else {
                ctx.say("Unknown board theme. Choose: classic, forest, sunset, or lavender. Use `.profile delete` to view the preference-deletion confirmation.")
                    .await?;
                return Ok(());
            };
            repository.set_theme(&user_id, theme).await?
        }
        None => repository.get_or_create_theme(&user_id).await?,
    };

    ctx.say(format!(
        "**Your Moonbyte profile**\nBoard theme: **{}**\n\nChange it with `.profile <theme>` or `/profile theme`. Use `.profile delete confirm` to remove this preference only. It is cosmetic and does not change game rules or outcomes.",
        saved_theme.label()
    ))
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn games(ctx: Context<'_>) -> Result<(), Error> {
    let entries = GameKind::ALL
        .iter()
        .map(|kind| {
            let definition = kind.definition();
            format!(
                "**{}** ({}) · {}\n{}\nControls: {}",
                definition.name,
                definition.id,
                definition.players,
                definition.description,
                definition.controls
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    ctx.say(format!("**Moonbyte Game Library**\n\n{entries}"))
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "tic-tac-toe, connect-four, memory-match, or word-scramble"] game: String,
) -> Result<(), Error> {
    let game = GameKind::parse(&game)?;
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let snapshot = ctx
        .data()
        .games()
        .start(game, &player_id, guild_id.as_deref())
        .await?;
    let intro = if snapshot.phase == GamePhase::WaitingForPlayers {
        format!(
            "Lobby created. Another player in this server can join with .join {}.",
            snapshot.id
        )
    } else {
        String::from("Your solo session is ready.")
    };
    ctx.say(format!("{intro}\n\n{}", snapshot.render())).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "The session ID shown in the lobby"] session_id: String,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let snapshot = ctx
        .data()
        .games()
        .join(&session_id, &player_id, guild_id.as_deref())
        .await?;
    ctx.say(format!("You joined the game.\n\n{}", snapshot.render()))
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn action(
    ctx: Context<'_>,
    #[description = "The session ID"] session_id: String,
    #[description = "Tic-tac-toe: row,column; Connect Four: column; Memory: position; Word Scramble: answer"]
    input: String,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let snapshot = ctx
        .data()
        .games()
        .act(&session_id, &player_id, guild_id.as_deref(), &input)
        .await?;
    ctx.say(snapshot.render()).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn session(
    ctx: Context<'_>,
    #[description = "The session ID"] session_id: String,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let snapshot = ctx
        .data()
        .games()
        .get(&session_id, &player_id, guild_id.as_deref())
        .await?;
    ctx.say(snapshot.render()).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn leave(
    ctx: Context<'_>,
    #[description = "The session ID"] session_id: String,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let snapshot = ctx
        .data()
        .games()
        .leave(&session_id, &player_id, guild_id.as_deref())
        .await?;
    ctx.say(format!("Session ended.\n\n{}", snapshot.render()))
        .await?;
    Ok(())
}

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![
        help(),
        about(),
        ping(),
        privacy(),
        profile(),
        games(),
        play(),
        join(),
        action(),
        session(),
        leave(),
    ]
}

fn help_message(prefix: &str) -> String {
    let safe_prefix = prefix
        .replace('@', "@\u{200B}")
        .replace(char::from(96), "ˋ")
        .replace(char::from(92), "＼");
    format!(
        "**Moonbyte commands**\n\
         Configured prefix: {safe_prefix}\n\n\
         {safe_prefix}help / /help — list commands\n\
         {safe_prefix}about / /about — describe this build\n\
         {safe_prefix}ping / /ping — check responsiveness and uptime\n\
         {safe_prefix}privacy / /privacy — read the current privacy overview\n\
         {safe_prefix}profile [theme] / /profile — view/change your cosmetic theme; delete confirm removes only that preference\n\
         {safe_prefix}games / /games — browse the playable library\n\
         {safe_prefix}play <game> / /play — start a solo game or multiplayer lobby\n\
         {safe_prefix}join <session> / /join — join an open lobby in this server\n\
         {safe_prefix}action <session> <input> / /action — submit a legal game action\n\
         {safe_prefix}session <session> / /session — inspect your own session\n\
         {safe_prefix}leave <session> / /leave — end a session you joined\n\n\
         Multiplayer sessions are scoped to their originating server. The current \
         session state is stored in PostgreSQL and expires after 24 hours of inactivity."
    )
}

const PRIVACY_OVERVIEW: &str = "\
**Moonbyte privacy — current build**\n\n\
Moonbyte processes Discord interactions to run commands and games. Active game sessions \
are stored in PostgreSQL, including session state, Discord participant IDs, optional \
guild scope, phase, and action counts. Sessions are scoped to their originating server \
and expire after 24 hours without activity. Raw message histories are not stored as \
analytics. A minimal profile row stores only the Discord user ID and a validated cosmetic \
board-theme preference. Inventories and game-history features are not connected.\n\n\
Discord independently processes interactions under its own policies. Moonbyte cannot \
control Discord's retention. PostgreSQL does not by itself prove encryption at rest, and \
this project does not claim end-to-end encryption.\n\n\
**Not implemented yet:** a complete stored-data inventory, private data export, and \
confirmed account deletion. Do not post private personal data in public channels. This \
overview is not a deletion request.";

#[cfg(test)]
mod tests {
    use super::help_message;

    #[test]
    fn help_lists_only_commands_registered_in_this_build() {
        let message = help_message(".");
        for command in [
            ".help / /help",
            ".about / /about",
            ".ping / /ping",
            ".privacy / /privacy",
            ".profile [theme] / /profile",
            ".games / /games",
            ".play <game> / /play",
            ".join <session> / /join",
            ".action <session> <input> / /action",
            ".session <session> / /session",
            ".leave <session> / /leave",
        ] {
            assert!(message.contains(command), "missing help entry: {command}");
        }
        assert!(message.contains("expires after 24 hours of inactivity"));
    }

    #[test]
    fn help_safely_displays_custom_prefixes() {
        let prefix = format!("@everyone{}", char::from(96));
        let message = help_message(&prefix);
        assert!(message.contains("@\u{200B}everyoneˋ"));
        assert!(!message.contains("@everyone"));
    }
}
