//! Discord-native commands. Handlers adapt user input to application services;
//! the game module remains independent of Serenity and Poise.

use crate::{
    Data, Error,
    game::{GameKind, GamePhase},
};
use poise::serenity_prelude::CreateMessage;

type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrivateResponseRoute {
    EphemeralInteraction,
    DirectMessage,
    ExistingPrivateChannel,
}

fn private_response_route(is_application: bool, in_guild: bool) -> PrivateResponseRoute {
    match (is_application, in_guild) {
        (true, _) => PrivateResponseRoute::EphemeralInteraction,
        (false, true) => PrivateResponseRoute::DirectMessage,
        (false, false) => PrivateResponseRoute::ExistingPrivateChannel,
    }
}

fn private_delivery_notice(delivered: bool) -> &'static str {
    if delivered {
        "I sent your session details in a DM."
    } else {
        "I couldn't DM you. Enable direct messages from this server and try again."
    }
}

/// Keep player-specific session inspection and listing responses out of shared guild channels.
async fn send_private(ctx: Context<'_>, message: String) -> Result<(), Error> {
    let route = private_response_route(
        matches!(&ctx, poise::Context::Application(_)),
        ctx.guild_id().is_some(),
    );

    match (ctx, route) {
        (poise::Context::Application(application), PrivateResponseRoute::EphemeralInteraction) => {
            application
                .send(
                    poise::CreateReply::default()
                        .content(message)
                        .ephemeral(true),
                )
                .await?;
        }
        (poise::Context::Prefix(prefix), PrivateResponseRoute::DirectMessage) => {
            let delivered = prefix
                .author()
                .direct_message(
                    prefix.serenity_context(),
                    CreateMessage::new().content(message),
                )
                .await
                .is_ok();
            prefix.say(private_delivery_notice(delivered)).await?;
        }
        (poise::Context::Prefix(prefix), PrivateResponseRoute::ExistingPrivateChannel) => {
            prefix.say(message).await?;
        }
        _ => unreachable!("private response route must match its Poise context"),
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(help_message(ctx.data().prefix())).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let message = format!(
        "Moonbyte v{} — a colourful world of games inside Discord.\n\n\
         This build includes five playable games, guild-scoped multiplayer lobbies, \
         turn validation, private session reads and explicit leave controls. Sessions \
         are persisted in PostgreSQL and expire after 24 hours of inactivity; player \
         profiles, progression, Vaults and the Bazaar are not connected yet.",
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
    ctx.say(format!(
        "**Moonbyte Game Library**\n\n{entries}\n\nUse `.rules <game>` or `/rules` for full instructions."
    ))
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn rules(
    ctx: Context<'_>,
    #[description = "A game ID or name, such as connect-four"] game: String,
) -> Result<(), Error> {
    let game = GameKind::parse(&game)?;
    ctx.say(game_rules_message(game)).await?;
    Ok(())
}

fn game_rules_message(game: GameKind) -> String {
    let definition = game.definition();
    format!(
        "**{} — How to play**\n{}\n\n**Controls**\n{}\n\nStart with `.play {}` or `/play` using game `{}`.",
        definition.name, definition.rules, definition.controls, definition.id, definition.id
    )
}

#[poise::command(prefix_command, slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "tic-tac-toe, connect-four, memory-match, word-scramble, or sequence-recall"] game: String,
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
    #[description = "Use the shown controls; Sequence Recall accepts hide or comma-separated values 1–4."]
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
    send_private(ctx, snapshot.render()).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn sessions(ctx: Context<'_>) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let scope = if guild_id.is_some() {
        "this server"
    } else {
        "your DMs"
    };
    let sessions = ctx
        .data()
        .games()
        .list_active_for_player(&player_id, guild_id.as_deref())
        .await?;

    if sessions.is_empty() {
        return send_private(
            ctx,
            format!("You don't have any active sessions in {scope}."),
        )
        .await;
    }

    let entries = sessions
        .iter()
        .map(|snapshot| {
            format!(
                "• **{}** · `{}` · {}\n  {}",
                snapshot.game.definition().name,
                snapshot.id,
                snapshot.phase,
                snapshot.last_result
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    send_private(
        ctx,
        format!(
            "**Your active sessions in {scope}**\n\n{entries}\n\nUse `session <id>` to inspect a board or `action <id> <input>` to play. Showing up to 10 most recently updated sessions."
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn history(ctx: Context<'_>) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let scope = if guild_id.is_some() {
        "this server"
    } else {
        "your DMs"
    };
    let sessions = ctx
        .data()
        .games()
        .list_recent_finished_for_player(&player_id, guild_id.as_deref())
        .await?;

    if sessions.is_empty() {
        return send_private(
            ctx,
            format!("You don't have any recent finished sessions in {scope}."),
        )
        .await;
    }

    let entries = sessions
        .iter()
        .map(|snapshot| {
            format!(
                "• **{}** · `{}` · {}\n  {}",
                snapshot.game.definition().name,
                snapshot.id,
                snapshot.phase,
                snapshot.last_result
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    send_private(
        ctx,
        format!(
            "**Your recent finished sessions in {scope}**\n\n{entries}\n\nShowing up to 10 sessions still inside the 24-hour inactivity window."
        ),
    )
    .await
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
        games(),
        rules(),
        play(),
        join(),
        action(),
        session(),
        sessions(),
        history(),
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
         {safe_prefix}games / /games — browse the playable library\n\
         {safe_prefix}rules <game> / /rules — read a game's rules and controls\n\
         {safe_prefix}play <game> / /play — start a solo game or multiplayer lobby\n\
         {safe_prefix}join <session> / /join — join an open lobby in this server\n\
         {safe_prefix}action <session> <input> / /action — submit a legal game action\n\
         {safe_prefix}session <session> / /session — inspect your own session\n\
         {safe_prefix}sessions / /sessions — list your active sessions here\n\
         {safe_prefix}history / /history — list your recent finished sessions here\n\
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
analytics. Player profiles, inventories, lifetime game history, export, and deletion are not connected. The .history command only lists finished sessions still inside the existing 24-hour inactivity window.\n\n\
**Channel visibility:** In a guild, .play, .join, .action, and .leave post game boards or \
results in the invoking channel. Members who can view that channel can see those \
responses and may see the command invocation itself. Use a DM for solo play you do not \
want shown in a shared server channel. .session, .sessions, and .history keep session \
details private: slash responses are ephemeral, guild-prefix requests deliver details \
by DM, and prefix requests in DMs reply in that private channel. If a DM fails, the bot \
does not post session details publicly.\n\n\
Discord independently processes interactions under its own policies. Moonbyte cannot \
control Discord's retention. PostgreSQL does not by itself prove encryption at rest, and \
this project does not claim end-to-end encryption.\n\n\
**Not implemented yet:** a complete stored-data inventory, private data export, and \
confirmed account deletion. Do not post private personal data in public channels. This \
overview is not a deletion request.";

#[cfg(test)]
mod tests {
    use super::{GameKind, PRIVACY_OVERVIEW, game_rules_message, help_message};

    #[test]
    fn private_session_responses_use_a_private_transport_for_each_context() {
        assert_eq!(
            super::private_response_route(true, true),
            super::PrivateResponseRoute::EphemeralInteraction
        );
        assert_eq!(
            super::private_response_route(true, false),
            super::PrivateResponseRoute::EphemeralInteraction
        );
        assert_eq!(
            super::private_response_route(false, true),
            super::PrivateResponseRoute::DirectMessage
        );
        assert_eq!(
            super::private_response_route(false, false),
            super::PrivateResponseRoute::ExistingPrivateChannel
        );
    }

    #[test]
    fn failed_private_delivery_notice_does_not_expose_session_details() {
        let notice = super::private_delivery_notice(false);
        assert!(notice.contains("Enable direct messages"));
        assert!(!notice.contains("MB-"));
        assert!(!notice.contains("session details"));
    }

    #[test]
    fn privacy_overview_discloses_gameplay_visibility_and_private_session_reads() {
        assert!(PRIVACY_OVERVIEW.contains("post game boards or"));
        assert!(PRIVACY_OVERVIEW.contains("Use a DM for solo play"));
        assert!(PRIVACY_OVERVIEW.contains("slash responses are ephemeral"));
        assert!(PRIVACY_OVERVIEW.contains("does not post session details publicly"));
    }

    #[test]
    fn help_lists_only_commands_registered_in_this_build() {
        let message = help_message(".");
        for command in [
            ".help / /help",
            ".about / /about",
            ".ping / /ping",
            ".privacy / /privacy",
            ".games / /games",
            ".rules <game> / /rules",
            ".play <game> / /play",
            ".join <session> / /join",
            ".action <session> <input> / /action",
            ".session <session> / /session",
            ".sessions / /sessions",
            ".history / /history",
            ".leave <session> / /leave",
        ] {
            assert!(message.contains(command), "missing help entry: {command}");
        }
        assert!(message.contains("expires after 24 hours of inactivity"));
    }

    #[test]
    fn game_rules_include_catalogue_rules_and_controls_for_every_playable_game() {
        for game in GameKind::ALL {
            let definition = game.definition();
            let message = game_rules_message(game);
            assert!(
                !definition.rules.trim().is_empty(),
                "missing rules for {}",
                definition.id
            );
            assert!(
                !definition.controls.trim().is_empty(),
                "missing controls for {}",
                definition.id
            );
            assert!(
                message.contains(definition.rules),
                "missing rules for {}",
                definition.id
            );
            assert!(
                message.contains(definition.controls),
                "missing controls for {}",
                definition.id
            );
            assert!(
                message.contains(definition.id),
                "missing start identifier for {}",
                definition.id
            );
        }
    }

    #[test]
    fn help_safely_displays_custom_prefixes() {
        let prefix = format!("@everyone{}", char::from(96));
        let message = help_message(&prefix);
        assert!(message.contains("@\u{200B}everyoneˋ"));
        assert!(!message.contains("@everyone"));
    }
}
