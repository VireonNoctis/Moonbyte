//! Discord-native commands. Handlers adapt user input to application services;
//! the game module remains independent of Serenity and Poise.

use crate::{Data, Error, game::{GameKind, GamePhase}};

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
         currently live in process memory and reset when the bot restarts; persistent \
         profiles, SQLite saves, Vaults and the Bazaar are not connected yet.",
        env!("CARGO_PKG_VERSION")
    );
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("Moonbyte is online. Process uptime: {} seconds.", ctx.data().uptime().as_secs())).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn privacy(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(PRIVACY_OVERVIEW).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn games(ctx: Context<'_>) -> Result<(), Error> {
    let entries = GameKind::ALL.iter().map(|kind| {
        let definition = kind.definition();
        format!(
            "**{}** ({}) · {}\n{}\nControls: {}",
            definition.name, definition.id, definition.players, definition.description, definition.controls
        )
    }).collect::<Vec<_>>().join("\n\n");
    ctx.say(format!("**Moonbyte Game Library**\n\n{entries}")).await?;
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
    let snapshot = ctx.data().games().start(game, &player_id, guild_id.as_deref())?;
    let intro = if snapshot.phase == GamePhase::WaitingForPlayers {
        format!("Lobby created. Another player in this server can join with .join {}.", snapshot.id)
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
    let snapshot = ctx.data().games().join(&session_id, &player_id, guild_id.as_deref())?;
    ctx.say(format!("You joined the game.\n\n{}", snapshot.render())).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn action(
    ctx: Context<'_>,
    #[description = "The session ID"] session_id: String,
    #[description = "Tic-tac-toe: row,column; Connect Four: column; Memory: position; Word Scramble: answer"] input: String,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let snapshot = ctx.data().games().act(&session_id, &player_id, guild_id.as_deref(), &input)?;
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
    let snapshot = ctx.data().games().get(&session_id, &player_id, guild_id.as_deref())?;
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
    let snapshot = ctx.data().games().leave(&session_id, &player_id, guild_id.as_deref())?;
    ctx.say(format!("Session ended.\n\n{}", snapshot.render())).await?;
    Ok(())
}

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![help(), about(), ping(), privacy(), games(), play(), join(), action(), session(), leave()]
}

fn help_message(prefix: &str) -> String {
    let safe_prefix = prefix
        .replace('@', "@​")
        .replace(char::from(96), "ˋ")
        .replace(char::from(92), "＼");
    format!(
        "**Moonbyte commands**\n\
         Configured prefix: {safe_prefix}\n\n\
         {safe_prefix}help / /help — list commands\n\
         {safe_prefix}about / /about — describe this build\n\
         {safe_prefix}ping / /ping — check responsiveness and uptime\n\
         {safe_prefix}privacy / /privacy — read the current privacy overview\n\
         {safe_prefix}games / /games — browse the playable library\n\
         {safe_prefix}play <game> / /play — start a solo game or multiplayer lobby\n\
         {safe_prefix}join <session> / /join — join an open lobby in this server\n\
         {safe_prefix}action <session> <input> / /action — submit a legal game action\n\
         {safe_prefix}session <session> / /session — inspect your own session\n\
         {safe_prefix}leave <session> / /leave — end a session you joined\n\n\
         Multiplayer sessions are scoped to their originating server. The current \
         runtime is in-memory; progress resets when the bot process restarts."
    )
}

const PRIVACY_OVERVIEW: &str = "\
**Moonbyte privacy — current build**\n\n\
Moonbyte processes Discord interactions to run commands and games. Game sessions are \
kept in process memory only and are not written to a database. The current runtime does \
not persist player profiles, inventories, message histories or game records. Sessions \
are scoped to their originating Discord server, and session reads are limited to their \
participants.\n\n\
Discord independently processes interactions under its own policies. Moonbyte cannot \
control Discord's retention. This project does not claim end-to-end encryption or \
database encryption.\n\n\
**Not implemented yet:** SQLite persistence, a stored-data inventory, private data \
export and confirmed account deletion. Do not post private personal data in public \
channels. This overview is not a deletion request.";

#[cfg(test)]
mod tests {
    use super::help_message;

    #[test]
    fn help_lists_only_commands_registered_in_this_build() {
        let message = help_message(".");
        for command in ["help", "about", "ping", "privacy", "games", "play", "join", "action", "session", "leave"] {
            assert!(message.contains(&format!(".{command} / /{command}")));
        }
        assert!(message.contains("progress resets when the bot process restarts"));
    }

    #[test]
    fn help_safely_displays_custom_prefixes() {
        let prefix = format!("@everyone{}", char::from(96));
        let message = help_message(&prefix);
        assert!(message.contains("@​everyoneˋ"));
        assert!(!message.contains("@everyone"));
    }
}
