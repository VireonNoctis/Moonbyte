//! Discord-native commands. Handlers adapt user input to application services;
//! the game module remains independent of Serenity and Poise.

use crate::{
    Data, Error,
    game::{GameKind, GamePhase},
    profile::PlayerProfile,
};
use poise::serenity_prelude::{ChannelId, CreateAttachment, CreateMessage};

use crate::guild_settings::{GuildCommandSettings, validate_prefix};

type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    ctx.say(help_message(&prefix)).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let message = format!(
        "Moonbyte v{} — a colourful world of games inside Discord.\n\n\
         This build includes four playable games, PostgreSQL-backed sessions, persistent \
         player profiles, per-game results and statistics, earned-only Moonbits milestones, \
         private data export and confirmed profile deletion. The initial Tavern Wayfinder \
         links to live destinations; the wider world is in active development.",
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

#[poise::command(prefix_command, slash_command, guild_only, ephemeral)]
pub async fn prefix(
    ctx: Context<'_>,
    #[description = "show | set <prefix> [#channel] | channel <#channel|off> | reset"]
    #[rest]
    changes: Option<String>,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say("Server command settings are only available inside a Discord server.")
            .await?;
        return Ok(());
    };

    let guild_key = guild_id.to_string();
    let current = ctx.data().guild_settings().get(&guild_key);
    let action = match parse_prefix_update(changes.as_deref()) {
        Ok(action) => action,
        Err(problem) => {
            ctx.say(format!(
                "**Server command settings**\n\n{problem}\n\nUsage: prefix set ! #bot-commands | prefix channel #bot-commands | prefix channel off | prefix reset."
            ))
            .await?;
            return Ok(());
        }
    };

    match action {
        PrefixUpdate::Show => {
            ctx.say(render_guild_settings(current.as_ref(), ctx.data().prefix()))
                .await?;
        }
        PrefixUpdate::Reset => {
            ctx.data().guild_settings().reset(&guild_key).await?;
            ctx.say(format!(
                "Server settings reset. The default prefix is now {} and commands are allowed in every channel.",
                escape_user_text(ctx.data().prefix())
            ))
            .await?;
        }
        PrefixUpdate::Set {
            prefix: new_prefix,
            channel,
        } => {
            let channel_id = match channel.as_deref() {
                Some(value) => match resolve_guild_channel(ctx, guild_id, value).await? {
                    Some(channel_id) => Some(channel_id),
                    None => {
                        ctx.say("That channel is not available in this server. Mention a channel from this server.")
                            .await?;
                        return Ok(());
                    }
                },
                None => current
                    .as_ref()
                    .and_then(|settings| settings.command_channel_id.clone()),
            };

            ctx.data()
                .guild_settings()
                .save(
                    &guild_key,
                    &new_prefix,
                    channel_id.as_deref(),
                    &ctx.author().id.to_string(),
                )
                .await?;

            let settings = ctx.data().guild_settings().get(&guild_key);
            ctx.say(format!(
                "Server settings saved.\n\n{}",
                render_guild_settings(settings.as_ref(), ctx.data().prefix())
            ))
            .await?;
        }
        PrefixUpdate::Channel(channel) => {
            let channel_id = match channel.as_deref() {
                Some(value) => match resolve_guild_channel(ctx, guild_id, value).await? {
                    Some(channel_id) => Some(channel_id),
                    None => {
                        ctx.say("That channel is not available in this server. Mention a channel from this server.")
                            .await?;
                        return Ok(());
                    }
                },
                None => None,
            };
            let active_prefix = current
                .as_ref()
                .map(|settings| settings.prefix.clone())
                .unwrap_or_else(|| ctx.data().prefix().to_owned());

            ctx.data()
                .guild_settings()
                .save(
                    &guild_key,
                    &active_prefix,
                    channel_id.as_deref(),
                    &ctx.author().id.to_string(),
                )
                .await?;

            let settings = ctx.data().guild_settings().get(&guild_key);
            ctx.say(format!(
                "Command channel updated.\n\n{}",
                render_guild_settings(settings.as_ref(), ctx.data().prefix())
            ))
            .await?;
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum PrefixUpdate {
    Show,
    Reset,
    Set {
        prefix: String,
        channel: Option<String>,
    },
    Channel(Option<String>),
}

fn parse_prefix_update(input: Option<&str>) -> Result<PrefixUpdate, &'static str> {
    let Some(input) = input.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(PrefixUpdate::Show);
    };

    let mut parts = input.split_whitespace();
    match parts.next() {
        Some("show") if parts.next().is_none() => Ok(PrefixUpdate::Show),
        Some("reset") if parts.next().is_none() => Ok(PrefixUpdate::Reset),
        Some("set") => {
            let prefix = parts.next().ok_or("Provide a prefix after set.")?;
            validate_prefix(prefix)?;
            let channel = parts.next().map(parse_channel_reference).transpose()?;
            if parts.next().is_some() {
                return Err("Use only one optional channel after the new prefix.");
            }
            Ok(PrefixUpdate::Set {
                prefix: prefix.to_owned(),
                channel,
            })
        }
        Some("channel") => {
            let channel = parts
                .next()
                .ok_or("Provide a channel mention, ID, or off.")?;
            let channel = if channel.eq_ignore_ascii_case("off") {
                None
            } else {
                Some(parse_channel_reference(channel)?)
            };
            if parts.next().is_some() {
                return Err("The channel setting accepts one channel or off.");
            }
            Ok(PrefixUpdate::Channel(channel))
        }
        _ => Err("Unknown option. Use show, set, channel, or reset."),
    }
}

fn parse_channel_reference(value: &str) -> Result<String, &'static str> {
    let value = value
        .strip_prefix("<#")
        .and_then(|value| value.strip_suffix('>'))
        .unwrap_or(value);
    let id = value
        .parse::<u64>()
        .ok()
        .filter(|id| *id != 0)
        .ok_or("Mention a channel from this server, or provide its numeric channel ID.")?;
    Ok(id.to_string())
}

async fn resolve_guild_channel(
    ctx: Context<'_>,
    guild_id: poise::serenity_prelude::GuildId,
    channel_id: &str,
) -> Result<Option<String>, Error> {
    let parsed_id = channel_id.parse::<u64>()?;
    let channel = ChannelId::new(parsed_id)
        .to_channel(ctx.serenity_context())
        .await?;
    Ok(match channel {
        poise::serenity_prelude::Channel::Guild(channel) if channel.guild_id == guild_id => {
            Some(channel.id.to_string())
        }
        _ => None,
    })
}

fn render_guild_settings(settings: Option<&GuildCommandSettings>, default_prefix: &str) -> String {
    let prefix = settings
        .map(|settings| settings.prefix.as_str())
        .unwrap_or(default_prefix);
    let channel = settings
        .and_then(|settings| settings.command_channel_id.as_deref())
        .map(|id| format!("<#{id}>"))
        .unwrap_or_else(|| "Every channel".to_owned());
    format!(
        "**Server command settings**\n\nPrefix: {}\nCommand channel: {}\n\nUse prefix set <prefix> [#channel] to update the prefix.\nUse prefix channel <#channel|off> to change the command channel.\nUse prefix reset to restore defaults.\n\nWhen a command channel is configured, all other commands are accepted only there. The prefix configuration command remains available from every channel.",
        escape_user_text(prefix),
        channel
    )
}

#[poise::command(prefix_command, slash_command)]
pub async fn privacy(ctx: Context<'_>) -> Result<(), Error> {
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    ctx.say(render_privacy_overview(&prefix)).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn privacy_data(ctx: Context<'_>) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
    let inventory = ctx.data().profiles().inventory(&player_id).await?;
    let message = format!(
        "**Your Moonbyte data inventory**\n\n\
         Profile record: {}\n\
         Game-session records currently retained: {}\n\
         Completed result records retained: {}\n\
         Per-game statistic rows: {}\n\
         Achievement records: {}\n\
         Wallet record: {}\n\
         Wallet ledger entries: {}\n\
         Atlas world-state record: {}\n\
         Atlas discovery records: {}\n\n\
         {}\n\n\
         Use `{prefix}privacy_export` to receive a JSON copy, or `{prefix}privacy_delete` to begin \
         the two-step deletion flow. This inventory covers the current PostgreSQL schema.",
        if inventory.profile_record {
            "present"
        } else {
            "not created"
        },
        inventory.game_session_records,
        inventory.game_result_records,
        inventory.game_stat_rows,
        inventory.achievement_rows,
        if inventory.wallet_record {
            "present"
        } else {
            "not created"
        },
        inventory.wallet_ledger_records,
        if inventory.world_state_record {
            "present"
        } else {
            "not created"
        },
        inventory.atlas_discovery_records,
        inventory.session_retention
    );
    send_private(ctx, message).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn privacy_export(ctx: Context<'_>) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let export = ctx.data().profiles().export_json(&player_id).await?;
    let attachment = CreateAttachment::bytes(export.into_bytes(), "moonbyte-data-export.json");
    let dm = ctx.author().create_dm_channel(ctx.http()).await?;
    dm.send_files(
        ctx.http(),
        [attachment],
        CreateMessage::new().content(
            "Your Moonbyte data export is attached. It covers the current PostgreSQL schema.",
        ),
    )
    .await?;
    if ctx.guild_id().is_some() {
        ctx.say("I sent your data export to your DMs.").await?;
    }
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn privacy_delete(
    ctx: Context<'_>,
    #[description = "Type DELETE to confirm removal of your Moonbyte profile, achievements, sessions and wallet data"]
    confirmation: Option<String>,
) -> Result<(), Error> {
    let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
    if !confirmation
        .as_deref()
        .is_some_and(|value| value.eq_ignore_ascii_case("DELETE"))
    {
        return send_private(
            ctx,
            format!(
                "**Delete your Moonbyte profile?**\n\n\
             This removes your profile, unlocked achievements, Moonbits wallet and ledger \
             entries, Atlas location/discovery records, and every stored game-session snapshot \
             containing your Discord ID. Shared sessions will end for \
             every participant because their state is stored together. This cannot be undone.\n\n\
             To confirm, run `{prefix}privacy_delete DELETE`. No deletion has happened yet.",
            ),
        )
        .await;
    }

    // Establish the private receipt channel before mutating data. If DMs are
    // unavailable, the command fails without deleting records silently.
    let private_channel = if ctx.guild_id().is_some() {
        Some(ctx.author().create_dm_channel(ctx.http()).await?)
    } else {
        None
    };
    let player_id = ctx.author().id.to_string();
    let deletion = ctx.data().profiles().delete_account(&player_id).await?;
    let receipt = format!(
        "**Moonbyte deletion completed**\n\n\
         Profile record removed: {}\n\
         Session records removed: {}\n\
         Completed results removed: {}\n\
         Per-game statistic rows removed: {}\n\
         Achievement records removed: {}\n\
         Wallet record removed: {}\n\
         Wallet ledger entries removed: {}\n\
         Atlas world-state record removed: {}\n\
         Atlas discovery records removed: {}\n\n\
         This confirms deletion from the current Moonbyte PostgreSQL tables only. \
         It cannot delete Discord messages, Discord account data, or copies retained \
         by other participants or infrastructure backups.",
        if deletion.profile_removed {
            "yes"
        } else {
            "no profile record existed"
        },
        deletion.sessions_removed,
        deletion.results_removed,
        deletion.stat_rows_removed,
        deletion.achievement_rows_removed,
        if deletion.wallet_removed {
            "yes"
        } else {
            "no wallet record existed"
        },
        deletion.wallet_ledger_records_removed,
        if deletion.world_state_removed {
            "yes"
        } else {
            "no record existed"
        },
        deletion.atlas_discoveries_removed
    );
    if let Some(dm) = private_channel {
        dm.say(ctx.http(), receipt).await?;
        ctx.say("Your deletion completed. I sent the private receipt to your DMs.")
            .await?;
    } else {
        ctx.say(receipt).await?;
    }
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn profile(ctx: Context<'_>) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let profile = ctx
        .data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let stats = ctx.data().profiles().game_stats(&player_id).await?;
    send_private(ctx, render_profile(&profile, &stats)).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn profile_edit(
    ctx: Context<'_>,
    #[description = "Display name | short bio; enter - after the separator to clear the bio"]
    #[rest]
    changes: String,
) -> Result<(), Error> {
    let Some((display_name, bio)) = changes.split_once('|') else {
        let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
        return send_private(
            ctx,
            format!("Use `{prefix}profile_edit Display Name | A short bio`. The display name is limited to 32 characters and the bio to 160."),
        )
        .await;
    };
    let player_id = ctx.author().id.to_string();
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let bio = bio.trim();
    let bio = if bio == "-" { "" } else { bio };
    let profile = ctx
        .data()
        .profiles()
        .update_identity(&player_id, display_name.trim(), bio)
        .await?;
    let stats = ctx.data().profiles().game_stats(&player_id).await?;
    send_private(
        ctx,
        format!("Profile saved.\n\n{}", render_profile(&profile, &stats)),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn wallet(ctx: Context<'_>) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let wallet = ctx.data().wallets().snapshot(&player_id).await?;
    let message = format!(
        "**Your Moonbits wallet**\n\nBalance: **{} Moonbits**\nLedger entries: {}\n\n\
         The first completion of each game type earns {} Moonbits once per player. \
         Replaying a game does not repeat that milestone. Moonbits are earned-only, \
         cannot be purchased or cashed out, and cannot be staked. Spending and trading \
         are not enabled yet.",
        wallet.balance,
        wallet.ledger_entries,
        crate::wallet::FIRST_COMPLETION_REWARD
    );
    send_private(ctx, message).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn achievements(ctx: Context<'_>) -> Result<(), Error> {
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    let player_id = ctx.author().id.to_string();
    let unlocked = ctx
        .data()
        .achievements()
        .list_for_player(&player_id)
        .await?;

    if unlocked.is_empty() {
        return send_private(
            ctx,
            format!(
                "**Your Achievement Gallery**\n\nNo achievements unlocked yet. \
                 Complete games to record your milestones. Use `{}games` to browse.",
                prefix
            ),
        )
        .await;
    }

    let entries = unlocked
        .iter()
        .map(|achievement| {
            format!(
                "🏅 **{}** · {}\n{}\nUnlocked: {}",
                achievement.title,
                achievement.category,
                achievement.description,
                achievement.unlocked_at
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    send_private(
        ctx,
        format!(
            "**Your Achievement Gallery** · {} unlocked\n\n{}",
            unlocked.len(),
            entries
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let stats = ctx.data().profiles().game_stats(&player_id).await?;
    send_private(ctx, render_statistics(&stats)).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "Profile visibility: public or private"] visibility: String,
    #[description = "Allow profile statistics to be shown: true or false"] show_statistics: bool,
    #[description = "Enable quiet mode for future optional notices: true or false"]
    quiet_mode: bool,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let profile = ctx
        .data()
        .profiles()
        .update_preferences(&player_id, &visibility, show_statistics, quiet_mode)
        .await?;
    let message = format!(
        "**Settings saved**\n\nVisibility: {}\nShow statistics: {}\nQuiet mode: {}",
        profile.visibility, profile.show_statistics, profile.quiet_mode
    );
    send_private(ctx, message).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn tavern(ctx: Context<'_>) -> Result<(), Error> {
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    let message = crate::tavern::render(&prefix);
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn atlas(ctx: Context<'_>) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let snapshot = ctx.data().atlas().snapshot(&player_id).await?;
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    send_private(ctx, crate::atlas::render(&snapshot, &prefix)).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn travel(
    ctx: Context<'_>,
    #[description = "A directly connected location ID or name, such as mosslight-grove"]
    location: String,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let result = match ctx.data().atlas().travel(&player_id, &location).await {
        Ok(result) => result,
        Err(crate::atlas::AtlasError::UnknownLocation) => {
            return send_private(
                ctx,
                format!(
                    "That place is not in the Atlas. Known locations: {}.",
                    crate::atlas::LOCATION_CATALOGUE
                        .iter()
                        .map(|place| format!("{} ({})", place.name, place.id))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
            .await;
        }
        Err(crate::atlas::AtlasError::NotReachable) => {
            return send_private(
                ctx,
                format!("That destination is not connected to your current location. Open `{}`atlas` to see the routes you can travel.", escape_user_text(&prefix)),
            )
            .await;
        }
        Err(error) => return Err(Box::new(error)),
    };

    let message = if result.moved {
        format!(
            "You travelled from **{}** to **{}**.",
            result.from.name, result.to.name
        )
    } else {
        format!("You are already at **{}**.", result.to.name)
    };
    send_private(
        ctx,
        format!(
            "{}\n\n{}",
            message,
            crate::atlas::render(&result.snapshot, &prefix)
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn games(ctx: Context<'_>) -> Result<(), Error> {
    let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
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
        "**Moonbyte Game Library**\n\n{entries}\n\n\
         First completion of each game type grants {} Moonbits once per player. \
         Repeat completions do not grant more. Use `{prefix}wallet` to view your balance.",
        crate::wallet::FIRST_COMPLETION_REWARD
    ))
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
    let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
    let intro = if snapshot.phase == GamePhase::WaitingForPlayers {
        format!(
            "Lobby created. Another player in this server can join with `{prefix}join {}`.",
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
        ctx.say(format!("You don't have any active sessions in {scope}."))
            .await?;
        return Ok(());
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

    ctx.say(format!(
        "**Your active sessions in {scope}**\n\n{entries}\n\nUse `session <id>` to inspect a board or `action <id> <input>` to play. Showing up to 10 most recently updated sessions."
    ))
    .await?;
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
    let mut prefix_command = prefix();
    prefix_command.required_permissions = poise::serenity_prelude::Permissions::MANAGE_GUILD;
    prefix_command.default_member_permissions = poise::serenity_prelude::Permissions::MANAGE_GUILD;

    vec![
        help(),
        about(),
        ping(),
        prefix_command,
        privacy(),
        privacy_data(),
        privacy_export(),
        privacy_delete(),
        profile(),
        profile_edit(),
        settings(),
        wallet(),
        achievements(),
        stats(),
        tavern(),
        atlas(),
        travel(),
        games(),
        play(),
        join(),
        action(),
        session(),
        sessions(),
        leave(),
    ]
}

async fn send_private(ctx: Context<'_>, content: impl Into<String>) -> Result<(), Error> {
    let content = content.into();
    if ctx.guild_id().is_some() {
        let dm = ctx.author().create_dm_channel(ctx.http()).await?;
        dm.say(ctx.http(), content).await?;
        ctx.say("I sent those details to your DMs.").await?;
    } else {
        ctx.say(content).await?;
    }
    Ok(())
}

fn render_profile(profile: &PlayerProfile, stats: &[crate::profile::PlayerGameStat]) -> String {
    let bio = profile
        .bio
        .as_deref()
        .map(escape_user_text)
        .unwrap_or_else(|| "No bio yet.".to_owned());
    format!(
        "**Your Moonbyte profile**\n\nDisplay name: {}\nBio: {}\nVisibility: {}\nShow statistics: {}\nQuiet mode: {}\n\n{}",
        escape_user_text(&profile.display_name),
        bio,
        profile.visibility,
        profile.show_statistics,
        profile.quiet_mode,
        render_statistics(stats)
    )
}

fn render_privacy_overview(prefix: &str) -> String {
    let prefix = escape_user_text(prefix);
    PRIVACY_OVERVIEW.replace("`.privacy", &format!("`{prefix}privacy"))
}

fn render_statistics(stats: &[crate::profile::PlayerGameStat]) -> String {
    if stats.is_empty() {
        return String::from("**Game statistics**\n\nNo completed games are recorded yet.");
    }
    let games_played: i64 = stats.iter().map(|stat| stat.games_played).sum();
    let games_won: i64 = stats.iter().map(|stat| stat.games_won).sum();
    let draws: i64 = stats.iter().map(|stat| stat.draws).sum();
    let entries = stats
        .iter()
        .map(|stat| {
            format!(
                "• **{}** — {} completed · {} wins · {} draws",
                stat.game_id, stat.games_played, stat.games_won, stat.draws
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "**Game statistics**\n\n{} completed · {} wins · {} draws\n\n{}",
        games_played, games_won, draws, entries
    )
}

fn escape_user_text(value: &str) -> String {
    value
        .replace('@', "@\u{200B}")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('~', "\\~")
        .replace('`', "ˋ")
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
         {safe_prefix}prefix [set|channel|reset] / /prefix — configure this server's prefix and command channel (Manage Server)\n\
         {safe_prefix}privacy / /privacy — read the current privacy overview\n\
         {safe_prefix}privacy_data / /privacy_data — inspect your stored data\n\
         {safe_prefix}privacy_export / /privacy_export — receive a private JSON export\n\
         {safe_prefix}privacy_delete [DELETE] / /privacy_delete — confirm profile deletion\n\
         {safe_prefix}profile / /profile — view your private player profile\n\
         {safe_prefix}profile_edit <name> | <bio> / /profile_edit — edit your profile\n\
         {safe_prefix}settings <visibility> <show_stats> <quiet_mode> / /settings — update preferences\n\
         {safe_prefix}wallet / /wallet — view your earned-only Moonbits balance\n\
         {safe_prefix}achievements / /achievements — view your unlocked achievements\n\
         {safe_prefix}stats / /stats — view completed game statistics\n\
         {safe_prefix}tavern / /tavern — open the Tavern and live Wayfinder\n\
         {safe_prefix}atlas / /atlas — inspect your world map and known routes\n\
         {safe_prefix}travel <location> / /travel — follow a connected Atlas route\n\
         {safe_prefix}games / /games — browse the playable library\n\
         {safe_prefix}play <game> / /play — start a solo game or multiplayer lobby\n\
         {safe_prefix}join <session> / /join — join an open lobby in this server\n\
         {safe_prefix}action <session> <input> / /action — submit a legal game action\n\
         {safe_prefix}session <session> / /session — inspect your own session\n\
         {safe_prefix}sessions / /sessions — list your active sessions here\n\
         {safe_prefix}leave <session> / /leave — end a session you joined\n\n\
         Multiplayer sessions are scoped to their originating server. Sessions expire \
         after 24 hours of inactivity."
    )
}

const PRIVACY_OVERVIEW: &str = "\
**Moonbyte privacy — current build**\n\n\
Moonbyte processes Discord interactions to run commands and games. PostgreSQL stores \
player profiles, session snapshots, completed-game result metadata, per-game aggregate \
statistics, earned-only wallet/ledger records, unlocked achievements and personal Atlas location/discovery records. Sessions include game state and participant IDs, are scoped to their originating \
server, and expire after 24 hours without activity. Completed result rows expire with their \
parent session; aggregate statistics and earned wallet/ledger records remain until the player \
requests deletion. The first completion of each game type earns 10 Moonbits once per player. \
Achievements such as first completion, first victory, completing all four games, ten total \
games and five wins unlock once and remain until account deletion; they do not grant currency. \
Spending and trading are not enabled.\n\n\
Player profiles store a Discord user ID, display name, optional bio, private-by-default \
visibility and two preference flags. Use `.privacy_data` to inspect your current records, \
`.privacy_export` to receive a private JSON export, and `.privacy_delete` to begin confirmed \
deletion. Profile and privacy responses are sent by DM when invoked in a server.\n\n\
Deletion removes the profile, per-game statistics, achievements, Moonbits wallet/ledger records, \
Atlas location/discovery records and every session/result snapshot containing your ID from the current PostgreSQL schema. It can end shared sessions. It cannot delete \
Discord's own messages, Discord account data, other participants' copies, or backups outside \
the current database.\n\n\
Moonbyte does not claim end-to-end encryption or encryption at rest. This is a summary of the \
current implementation, not a statement about Discord's independent data practices.";

#[cfg(test)]
mod tests {
    use super::{
        PrefixUpdate, escape_user_text, help_message, parse_prefix_update, render_privacy_overview,
    };

    #[test]
    fn help_lists_commands_registered_in_this_build() {
        let message = help_message(".");
        for command in [
            ".help / /help",
            ".about / /about",
            ".ping / /ping",
            ".prefix [set|channel|reset] / /prefix",
            ".privacy / /privacy",
            ".privacy_data / /privacy_data",
            ".privacy_export / /privacy_export",
            ".privacy_delete [DELETE] / /privacy_delete",
            ".profile / /profile",
            ".profile_edit <name> | <bio> / /profile_edit",
            ".settings <visibility> <show_stats> <quiet_mode> / /settings",
            ".wallet / /wallet",
            ".achievements / /achievements",
            ".stats / /stats",
            ".tavern / /tavern",
            ".atlas / /atlas",
            ".travel <location> / /travel",
            ".games / /games",
            ".play <game> / /play",
            ".join <session> / /join",
            ".action <session> <input> / /action",
            ".session <session> / /session",
            ".sessions / /sessions",
            ".leave <session> / /leave",
        ] {
            assert!(message.contains(command), "missing help entry: {command}");
        }
    }

    #[test]
    fn prefix_parser_supports_show_set_channel_and_reset() {
        assert_eq!(parse_prefix_update(None), Ok(PrefixUpdate::Show));
        assert_eq!(parse_prefix_update(Some("show")), Ok(PrefixUpdate::Show));
        assert_eq!(
            parse_prefix_update(Some("set mb! <#123456789012345678>")),
            Ok(PrefixUpdate::Set {
                prefix: "mb!".to_owned(),
                channel: Some("123456789012345678".to_owned()),
            })
        );
        assert_eq!(
            parse_prefix_update(Some("channel off")),
            Ok(PrefixUpdate::Channel(None))
        );
        assert_eq!(parse_prefix_update(Some("reset")), Ok(PrefixUpdate::Reset));
    }

    #[test]
    fn prefix_parser_rejects_invalid_prefixes_and_extra_arguments() {
        assert!(parse_prefix_update(Some("set two words")).is_err());
        assert!(parse_prefix_update(Some("set ! 123 456")).is_err());
        assert!(parse_prefix_update(Some("channel")).is_err());
        assert!(parse_prefix_update(Some("channel not-a-channel")).is_err());
        assert!(parse_prefix_update(Some("reset extra")).is_err());
    }

    #[test]
    fn privacy_overview_uses_the_guild_prefix() {
        let message = render_privacy_overview("mb!");
        assert!(message.contains("`mb!privacy_data`"));
        assert!(message.contains("`mb!privacy_export`"));
        assert!(message.contains("`mb!privacy_delete`"));
        assert!(!message.contains("`.privacy_data`"));
    }

    #[test]
    fn user_content_cannot_create_mentions_or_markdown() {
        let escaped = escape_user_text("@everyone **hello** `code`");
        assert!(!escaped.contains("@everyone"));
        assert!(!escaped.contains("**"));
        assert!(!escaped.contains("`code`"));
    }

    #[test]
    fn help_safely_displays_custom_prefixes() {
        let prefix = format!("@everyone{} ", char::from(96));
        let message = help_message(&prefix);
        assert!(message.contains("@\u{200B}everyoneˋ"));
        assert!(!message.contains("@everyone"));
    }
}
