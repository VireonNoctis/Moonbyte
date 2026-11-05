//! Discord-native commands. Handlers adapt user input to application services;
//! the game module remains independent of Serenity and Poise.

use crate::{
    Data, Error,
    game::{GameKind, GamePhase, SessionSnapshot},
    profile::PlayerProfile,
};
use poise::serenity_prelude::{
    AutocompleteChoice, ButtonStyle, ChannelId, ComponentInteraction, CreateActionRow,
    CreateAttachment, CreateAutocompleteResponse, CreateButton, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};

use crate::guild_settings::{GuildCommandSettings, validate_prefix};

const TERMS_DOCUMENT: &str = include_str!("../docs/TERMS_OF_SERVICE.md");
const PRIVACY_DOCUMENT: &str = include_str!("../docs/PRIVACY.md");

type Context<'a> = poise::Context<'a, Data, Error>;

const GAME_COMPONENT_PREFIX: &str = "mbg";

fn parse_game_component_id(custom_id: &str) -> Option<(String, u32, u8)> {
    let mut parts = custom_id.split(':');
    if parts.next()? != GAME_COMPONENT_PREFIX {
        return None;
    }

    let session_id = parts.next()?.to_owned();
    if session_id.is_empty() {
        return None;
    }
    let action_version = parts.next()?.parse::<u32>().ok()?;
    let value = parts.next()?.parse::<u8>().ok()?;
    if parts.next().is_some() {
        return None;
    }

    Some((session_id, action_version, value))
}

fn game_action_input(game: GameKind, value: u8) -> Option<String> {
    match game {
        GameKind::TicTacToe if value < 9 => Some(format!("{},{}", value / 3 + 1, value % 3 + 1)),
        GameKind::ConnectFour if (1..=7).contains(&value) => Some(value.to_string()),
        GameKind::MemoryMatch if (1..=8).contains(&value) => Some(value.to_string()),
        GameKind::WordScramble => None,
        _ => None,
    }
}

fn game_components(snapshot: &SessionSnapshot) -> Vec<CreateActionRow> {
    let (start, count, per_row) = match snapshot.game {
        GameKind::TicTacToe => (0_u8, 9_u8, 3_usize),
        GameKind::ConnectFour => (1_u8, 7_u8, 5_usize),
        GameKind::MemoryMatch => (1_u8, 8_u8, 4_usize),
        GameKind::WordScramble => return Vec::new(),
    };

    // Waiting lobbies have no legal moves yet. Terminal boards retain disabled
    // controls so the final message clearly shows that play has ended.
    if snapshot.phase == GamePhase::WaitingForPlayers {
        return Vec::new();
    }
    let disabled = snapshot.phase != GamePhase::Active;
    let values = (start..start + count).collect::<Vec<_>>();

    values
        .chunks(per_row)
        .map(|group| {
            CreateActionRow::Buttons(
                group
                    .iter()
                    .map(|value| {
                        let label = if snapshot.game == GameKind::TicTacToe {
                            (value + 1).to_string()
                        } else {
                            value.to_string()
                        };
                        CreateButton::new(format!(
                            "{GAME_COMPONENT_PREFIX}:{}:{}:{}",
                            snapshot.id, snapshot.action_count, value
                        ))
                        .label(label)
                        .style(ButtonStyle::Primary)
                        .disabled(disabled)
                    })
                    .collect(),
            )
        })
        .collect()
}

fn game_reply(content: String, snapshot: &SessionSnapshot) -> poise::CreateReply {
    let components = game_components(snapshot);
    let content = if components.is_empty() {
        content
    } else {
        format!("{content}\n\nChoose a numbered button below, or use the command format above.")
    };

    let reply = poise::CreateReply::new().embed(
        CreateEmbed::new()
            .title(snapshot.game.definition().name)
            .description(content),
    );
    if components.is_empty() {
        reply
    } else {
        reply.components(components)
    }
}

async fn component_notice(
    discord: &poise::serenity_prelude::Context,
    interaction: &ComponentInteraction,
    content: impl Into<String>,
) -> Result<(), Error> {
    interaction
        .create_response(
            discord,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(true),
            ),
        )
        .await?;
    Ok(())
}

/// Handles versioned game-board buttons from Discord's native component event.
pub async fn handle_game_component(
    discord: &poise::serenity_prelude::Context,
    data: &Data,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    let Some((session_id, expected_action_version, value)) =
        parse_game_component_id(&interaction.data.custom_id)
    else {
        return Ok(());
    };

    let player_id = interaction.user.id.to_string();
    let guild_id = interaction.guild_id;
    let guild_key = guild_id.map(|id| id.to_string());

    if !data.consents().is_current(&player_id).await? {
        let prefix = escape_user_text(&data.command_prefix(guild_id));
        return component_notice(
            discord,
            interaction,
            format!(
                "Before using game controls, review `{prefix}terms` and `{prefix}privacy`, then run `{prefix}consent accept`."
            ),
        )
        .await;
    }

    if let Some(guild_id) = guild_key.as_deref()
        && let Some(settings) = data.guild_settings().get(guild_id)
        && let Some(allowed_channel) = settings.command_channel_id
        && allowed_channel != interaction.channel_id.to_string()
    {
        return component_notice(
            discord,
            interaction,
            format!("Game controls for this server are restricted to <#{allowed_channel}>."),
        )
        .await;
    }

    let current = match data
        .games()
        .get(&session_id, &player_id, guild_key.as_deref())
        .await
    {
        Ok(snapshot) => snapshot,
        Err(error) => {
            return component_notice(discord, interaction, error.to_string()).await;
        }
    };

    // Every board's controls are bound to the action count they displayed.
    // A retry or a click from an older board refreshes the view without applying
    // another move.
    if current.action_count != expected_action_version || current.phase != GamePhase::Active {
        let refreshed = CreateInteractionResponseMessage::new()
            .content(format!(
                "{}\n\nThis board was refreshed because that control was out of date.",
                current.render()
            ))
            .components(game_components(&current));
        interaction
            .create_response(discord, CreateInteractionResponse::UpdateMessage(refreshed))
            .await?;
        return Ok(());
    }

    let Some(input) = game_action_input(current.game, value) else {
        return component_notice(discord, interaction, "That game control is not valid.").await;
    };

    match data
        .games()
        .act_at_version(
            &session_id,
            &player_id,
            guild_key.as_deref(),
            &input,
            expected_action_version,
        )
        .await
    {
        Ok(snapshot) => {
            let updated = CreateInteractionResponseMessage::new()
                .content(snapshot.render())
                .components(game_components(&snapshot));
            interaction
                .create_response(discord, CreateInteractionResponse::UpdateMessage(updated))
                .await?;
            tracing::debug!(
                event = "game_component_action_applied",
                game = snapshot.game.definition().id
            );
        }
        Err(crate::game::GameRuntimeError::StaleAction) => {
            let latest = data
                .games()
                .get(&session_id, &player_id, guild_key.as_deref())
                .await?;
            let refreshed = CreateInteractionResponseMessage::new()
                .content(format!(
                    "{}\n\nThis board was refreshed because another move was accepted first.",
                    latest.render()
                ))
                .components(game_components(&latest));
            interaction
                .create_response(discord, CreateInteractionResponse::UpdateMessage(refreshed))
                .await?;
        }
        Err(error) => {
            component_notice(discord, interaction, error.to_string()).await?;
        }
    }

    Ok(())
}

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
    let started = std::time::Instant::now();
    let database_ready = ctx.data().database().health_check().await.is_ok();
    let database_latency = started.elapsed().as_millis();

    if !database_ready {
        tracing::warn!(event = "database_readiness_check_failed");
    }

    let database_status = if database_ready {
        format!("reachable · {database_latency}ms")
    } else {
        "unavailable".to_owned()
    };
    ctx.say(format!(
        "Moonbyte process: responsive · uptime: {}s\nDatabase: {database_status}",
        ctx.data().uptime().as_secs()
    ))
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, ephemeral)]
pub async fn terms(ctx: Context<'_>) -> Result<(), Error> {
    let attachment = CreateAttachment::bytes(
        TERMS_DOCUMENT.as_bytes().to_vec(),
        "moonbyte-terms-of-service.md",
    );
    ctx.send(
        poise::CreateReply::new()
            .content(format!(
                "Moonbyte Terms of Service, version {}. The complete document is attached. Please review it before accepting.",
                crate::consent::TERMS_VERSION
            ))
            .attachment(attachment)
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, ephemeral)]
pub async fn consent(
    ctx: Context<'_>,
    #[description = "Type accept after reviewing the Terms of Service and Privacy Policy"]
    #[rest]
    action: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
    match action.as_deref().map(str::trim) {
        Some(value) if value.eq_ignore_ascii_case("accept") => {
            let guild_id = ctx.guild_id().map(|id| id.to_string());
            let record = ctx
                .data()
                .consents()
                .accept(&player_id, guild_id.as_deref())
                .await?;
            send_private(
                ctx,
                format!(
                    "Agreement recorded. Terms of Service: v{}; Privacy Policy: v{}; accepted at {}. You're all set. Run your previous command again to continue.",
                    record.terms_version,
                    record.privacy_version,
                    record.accepted_at
                ),
            )
            .await?;
        }
        Some(_) => {
            send_private(
                ctx,
                format!(
                    "To agree to both current documents, review the Terms of Service and Privacy Policy, then run {prefix}consent accept."
                ),
            )
            .await?;
        }
        None => {
            let current = ctx.data().consents().is_current(&player_id).await?;
            let message = if current {
                format!(
                    "Your agreement is up to date. Terms of Service: v{}; Privacy Policy: v{}.",
                    crate::consent::TERMS_VERSION,
                    crate::consent::PRIVACY_VERSION
                )
            } else {
                format!(
                    "Agreement required. Read {prefix}terms and {prefix}privacy, then run {prefix}consent accept."
                )
            };
            send_private(ctx, message).await?;
        }
    }
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

#[poise::command(prefix_command, slash_command, ephemeral)]
pub async fn privacy(ctx: Context<'_>) -> Result<(), Error> {
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    let attachment = CreateAttachment::bytes(
        PRIVACY_DOCUMENT.as_bytes().to_vec(),
        "moonbyte-privacy-policy.md",
    );
    ctx.send(
        poise::CreateReply::new()
            .content(format!(
                "{} The complete Privacy Policy is attached. Please review it before accepting.",
                render_privacy_overview(&prefix)
            ))
            .attachment(attachment)
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn privacy_data(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
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
         Policy acceptance record: {}\n\
         Wallet record: {}\n\
         Wallet ledger entries: {}\n\
         Atlas world-state record: {}\n\
         Atlas discovery records: {}\n\
         Chronicle archive pages: {}\n\
         Account-bound Atlas keepsakes: {}\n\
         Wayfarer active days: {}\n\
         Daily quest rotations: {}\n\
         Daily quest records: {}\n\
         Rivalry records: {}\nExpedition participations: {}\nExpedition checkpoints: {}\n\n\
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
        if inventory.consent_record {
            "present"
        } else {
            "not recorded"
        },
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
        inventory.archive_fragment_records,
        inventory.inventory_item_records,
        inventory.wayfarer_active_days,
        inventory.quest_rotation_records,
        inventory.daily_quest_records,
        inventory.rivalry_records,
        inventory.expedition_run_records,
        inventory.expedition_checkpoint_records,
        inventory.session_retention
    );
    send_private_title_panel(
        ctx,
        "Your Moonbyte Data Inventory",
        title.as_deref(),
        message,
    )
    .await
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
    #[description = "Type DELETE to confirm removal of your profile, sessions, wallet, keepsakes, Chronicle pages, quests, Wayfarer history and rivalries"]
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
             This removes your profile, policy-acceptance record, unlocked achievements, Moonbits wallet and ledger \
             entries, Atlas location/discovery records, Chronicle archive pages, account-bound keepsakes and their provenance, daily quest rotations and progress, Wayfarer activity days, rivalry invitations and match records, and every stored game-session snapshot \
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
         Atlas discovery records removed: {}\n\
         Chronicle archive pages removed: {}\n\
         Inventory items removed: {}\n\
         Wayfarer activity days removed: {}\n\
         Daily quest rotations removed: {}\n\
         Daily quest records removed: {}\n\
         Rivalry records removed: {}\nExpedition participations removed: {}\nExpedition runs transferred to a new host: {}\nExpedition runs cancelled: {}\nEmpty Expedition runs removed: {}\n\
         Policy acceptance record removed: {}\n\n\
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
        deletion.atlas_discoveries_removed,
        deletion.archive_fragments_removed,
        deletion.inventory_items_removed,
        deletion.wayfarer_days_removed,
        deletion.quest_rotations_removed,
        deletion.daily_quest_records_removed,
        deletion.rivalry_records_removed,
        deletion.expedition_participations_removed,
        deletion.expedition_runs_host_transferred,
        deletion.expedition_runs_cancelled,
        deletion.expedition_runs_removed,
        if deletion.consent_record_removed {
            "yes"
        } else {
            "no record existed"
        }
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
pub async fn profile(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let profile = ctx
        .data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let stats = ctx.data().profiles().game_stats(&player_id).await?;
    send_private_title_panel(
        ctx,
        "Your Moonbyte Profile",
        title.as_deref(),
        render_profile(&profile, &stats),
    )
    .await
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
pub async fn wallet(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
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
    send_private_title_panel(ctx, "Moonbits Wallet", title.as_deref(), message).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn achievements(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    let player_id = ctx.author().id.to_string();
    let unlocked = ctx
        .data()
        .achievements()
        .list_for_player(&player_id)
        .await?;

    if unlocked.is_empty() {
        return send_private_title_panel(
            ctx,
            "Achievement Gallery",
            title.as_deref(),
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

    send_private_title_panel(
        ctx,
        "Achievement Gallery",
        title.as_deref(),
        format!(
            "**Your Achievement Gallery** · {} unlocked\n\n{}",
            unlocked.len(),
            entries
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn inventory(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let snapshot = ctx.data().inventory().snapshot(&player_id).await?;
    send_private_title_panel(
        ctx,
        "Your Satchel",
        title.as_deref(),
        crate::inventory::render_inventory(&snapshot),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn collection(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let snapshot = ctx.data().inventory().snapshot(&player_id).await?;
    send_private_title_panel(
        ctx,
        "Postcard Wall",
        title.as_deref(),
        crate::inventory::render_collection(&snapshot),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn craft(
    ctx: Context<'_>,
    #[description = "Choose a recipe. Suggestions show the output and the materials required."]
    #[max_length = 100]
    #[autocomplete = "autocomplete_craft_recipe"]
    recipe: String,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let recipe_id =
        crate::inventory::resolve_recipe_id(&recipe).unwrap_or_else(|| recipe.trim());
    let outcome = ctx
        .data()
        .inventory()
        .craft_recipe(&player_id, recipe_id)
        .await?;

    let message = match outcome {
        crate::inventory::CraftOutcome::Crafted => {
            "✨ **Wayfarer's Lantern crafted.** It has been added to your Satchel.".to_owned()
        }
        crate::inventory::CraftOutcome::AlreadyCrafted => {
            "You already crafted the Wayfarer's Lantern. The recipe can only be completed once."
                .to_owned()
        }
        crate::inventory::CraftOutcome::MissingIngredients => {
            "The recipe needs 1 Mosslight Silk and 1 Tideglass. Discover Mosslight Grove and Glasswater Pier through the Atlas to earn them."
                .to_owned()
        }
        crate::inventory::CraftOutcome::UnknownRecipe => {
            match suggest_known_value(recipe.trim(), &["wayfarer-lantern"]) {
                Some(value) => format!(
                    "Unknown recipe '{}'. Did you mean {}?\n\nThe Workshop currently offers one fixed recipe.",
                    escape_user_text(recipe.trim()), value
                ),
                None => format!(
                    "Unknown recipe '{}'. The available recipe is wayfarer-lantern.",
                    escape_user_text(recipe.trim())
                ),
            }
        }
    };

    send_title_panel(
        ctx,
        "Wayfarer Workshop · Crafting",
        title.as_deref(),
        &message,
        true,
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn quests(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let quests = ctx.data().wayfarer().daily_quests(&player_id).await?;
    let entries = quests
        .iter()
        .map(|quest| {
            let progress = format!("{}/{}", quest.progress, quest.target);
            let action = match quest.status.as_str() {
                "available" => format!("Accept with .quest accept {}", quest.id),
                "active" => format!("In progress · {}", progress),
                "completed" => format!("Ready to claim · .quest claim {}", quest.id),
                "claimed" => "Reward claimed".to_owned(),
                _ => "Status unavailable".to_owned(),
            };
            format!(
                "**{}** · {} Moonbits\n{}\n{}\nID: {}",
                quest.title, quest.reward_moonbits, quest.description, action, quest.id
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let fingerprint = quests
        .first()
        .map(|quest| {
            quest
                .rotation_fingerprint
                .chars()
                .take(16)
                .collect::<String>()
        })
        .unwrap_or_else(|| "unavailable".to_owned());
    send_private_title_panel(
        ctx,
        "Daily Quest Board",
        title.as_deref(),
        format!(
            "**The Wayfarer · Daily Quest Board**\n\n{}\n\nRotation fingerprint: SHA-512 {fingerprint}…\nThe board is fixed for the day. Accept only the quests you want to pursue; missed days do not remove progress or rewards. Rewards are fixed and credited to your immutable Moonbits ledger when claimed.",
            entries
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn quest(
    ctx: Context<'_>,
    #[description = "Choose whether to accept a quest or claim its reward."]
    #[autocomplete = "autocomplete_quest_action"]
    action: String,
    #[description = "The quest ID shown on the daily board"] quest_id: String,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    match action.trim().to_ascii_lowercase().as_str() {
        "accept" => {
            ctx.data()
                .wayfarer()
                .accept_quest(&player_id, &quest_id)
                .await?;
            send_private(
                ctx,
                format!(
                    "Quest accepted: {}. Your progress is recorded automatically from completed games and first-time Atlas discoveries. Check .quests to follow it.",
                    escape_user_text(&quest_id)
                ),
            )
            .await
        }
        "claim" => {
            let reward = ctx
                .data()
                .wayfarer()
                .claim_quest(&player_id, &quest_id)
                .await?;
            send_private(
                ctx,
                format!(
                    "Quest complete. **{reward} Moonbits** have been credited to your wallet and recorded in the ledger. The reward cannot be claimed twice."
                ),
            )
            .await
        }
        _ => {
            let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
            let suggestion = suggest_known_value(action.trim(), &["accept", "claim"]);
            let guidance = suggestion
                .map(|value| format!("Did you mean {value}? "))
                .unwrap_or_default();
            send_private(
                ctx,
                format!("{guidance}Use {prefix}quest accept <quest-id> or {prefix}quest claim <quest-id>."),
            )
            .await
        }
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn wayfarer(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let snapshot = ctx.data().wayfarer().snapshot(&player_id).await?;
    let next = snapshot
        .next_milestone
        .map(|threshold| {
            format!(
                "{} more active day(s) to the next companion milestone",
                threshold - snapshot.active_days
            )
        })
        .unwrap_or_else(|| "The final currently defined companion milestone is yours.".to_owned());
    send_private_title_panel(
        ctx,
        "Wayfarer Thread",
        title.as_deref(),
        format!(
            "**The Wayfarer Thread**\n\nJourney days: **{}**\nCompanion: **{}**\nJourney stage: **{}**\n{}\n\nMeaningful game completions and first-time Atlas discoveries count once per calendar day. Your total never decreases; missing a day carries no penalty. Optional quests and cosmetic companion milestones never affect game outcomes.",
            snapshot.active_days, snapshot.companion_name, snapshot.companion_stage, next
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn companion(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let snapshot = ctx.data().wayfarer().snapshot(&player_id).await?;
    send_private_title_panel(
        ctx,
        "Companion Journal",
        title.as_deref(),
        format!(
            "**{} · Companion Journal**\n\nCurrent stage: **{}**\nJourney days recorded: **{}**\n\nCompanion milestones are cosmetic. They do not boost stats, change game rules, or grant random rewards. Keep exploring at your own pace.",
            snapshot.companion_name, snapshot.companion_stage, snapshot.active_days
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn rivals(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let records = ctx.data().wayfarer().rivalries(&player_id).await?;
    if records.is_empty() {
        let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
        return send_private_title_panel(
            ctx,
            "Rivalry Hall",
            title.as_deref(),
            format!(
                "**Rivalry Hall**\n\nNo invitations or rivalries yet. A rivalry starts only when both players opt in. Use {prefix}rival declare @player to invite someone who has opened Moonbyte."
            ),
        )
        .await;
    }

    let entries = records
        .iter()
        .map(|record| {
            let state = match record.status.as_str() {
                "pending" if record.invited_by == player_id => "Invitation sent · awaiting response",
                "pending" => "Invitation received · use .rival accept <id> or .rival decline <id>",
                "active" => "Active rivalry",
                "declined" => "Declined",
                _ => "Unknown status",
            };
            if record.status == "active" {
                format!(
                    "**<@{}>** · {} · {} matches\nYour wins: {} · Their wins: {} · Draws: {}\nRivalry ID: {}",
                    record.opponent_id, state, record.matches_played, record.your_wins,
                    record.opponent_wins, record.draws, record.rivalry_id
                )
            } else {
                format!(
                    "**<@{}>** · {}\nRivalry ID: {}",
                    record.opponent_id, state, record.rivalry_id
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    send_private_title_panel(
        ctx,
        "Rivalry Hall",
        title.as_deref(),
        format!(
            "**Rivalry Hall**\n\n{entries}\n\nOnly matches completed after a rivalry becomes active are counted. Rivalry records are private to their two participants; they do not create public leaderboards."
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn rival(
    ctx: Context<'_>,
    #[description = "Choose a mutual-rivalry action."]
    #[autocomplete = "autocomplete_rival_action"]
    action: String,
    #[description = "A player mention for declare, or a rivalry ID for accept/decline"]
    target: String,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let action = action.trim().to_ascii_lowercase();
    match action.as_str() {
        "declare" => {
            let opponent_id = parse_discord_user_id(&target)
                .ok_or("Mention a valid Discord player to invite to a rivalry.")?;
            let became_active = ctx
                .data()
                .wayfarer()
                .declare_rival(&player_id, &opponent_id)
                .await?;
            if became_active {
                send_private(
                    ctx,
                    "You both sent an invitation. Your rivalry is now active; future completed two-player games between you will be recorded.",
                )
                .await
            } else {
                send_private(
                    ctx,
                    format!(
                        "Rivalry invitation sent to <@{opponent_id}>. It becomes active only after they accept. No match history is collected until both players agree."
                    ),
                )
                .await
            }
        }
        "accept" | "decline" => {
            let rivalry_id = target
                .trim()
                .parse::<i64>()
                .map_err(|_| "Enter the numeric rivalry ID shown in .rivals.")?;
            if action == "accept" {
                ctx.data()
                    .wayfarer()
                    .accept_rival(&player_id, rivalry_id)
                    .await?;
                send_private(
                    ctx,
                    "Rivalry accepted. Future completed two-player games between you will appear in your private Rivalry Hall.",
                )
                .await
            } else {
                ctx.data()
                    .wayfarer()
                    .decline_rival(&player_id, rivalry_id)
                    .await?;
                send_private(
                    ctx,
                    "Rivalry invitation declined. No match history will be recorded.",
                )
                .await
            }
        }
        _ => {
            let prefix = escape_user_text(&ctx.data().command_prefix(ctx.guild_id()));
            let suggestion = suggest_known_value(action.trim(), &["declare", "accept", "decline"]);
            let guidance = suggestion
                .map(|value| format!("Did you mean {value}? "))
                .unwrap_or_default();
            send_private(
                ctx,
                format!(
                    "{guidance}Use {prefix}rival declare @player, {prefix}rival accept <id>, or {prefix}rival decline <id>. View the private list with {prefix}rivals."
                ),
            )
            .await
        }
    }
}

fn parse_discord_user_id(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let digits = if let Some(inner) = trimmed.strip_prefix("<@").and_then(|v| v.strip_suffix('>')) {
        inner.strip_prefix('!').unwrap_or(inner)
    } else {
        trimmed
    };
    if (17..=20).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_digit()) {
        Some(digits.to_owned())
    } else {
        None
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let stats = ctx.data().profiles().game_stats(&player_id).await?;
    send_private_title_panel(
        ctx,
        "Game Statistics",
        title.as_deref(),
        render_statistics(&stats),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "Choose whether your profile is public or private."]
    #[autocomplete = "autocomplete_profile_visibility"]
    visibility: String,
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
pub async fn tavern(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    let message = crate::tavern::render(&prefix);
    send_title_panel(ctx, "Tavern Wayfinder", title.as_deref(), &message, false).await
}

#[poise::command(prefix_command, slash_command)]
pub async fn atlas(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    let snapshot = ctx.data().atlas().snapshot(&player_id).await?;
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    send_private_title_panel(
        ctx,
        "Atlas",
        title.as_deref(),
        crate::atlas::render(&snapshot, &prefix),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn chronicle(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    ctx.data()
        .profiles()
        .get_or_create(&player_id, &ctx.author().name)
        .await?;
    ctx.data().atlas().snapshot(&player_id).await?;
    let snapshot = ctx.data().chronicle().snapshot(&player_id).await?;
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    send_private_title_panel(
        ctx,
        "Chronicle Archive",
        title.as_deref(),
        crate::chronicle::render(&snapshot, &prefix),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn travel(
    ctx: Context<'_>,
    #[description = "Choose an Atlas destination. Route reachability is checked after selection."]
    #[autocomplete = "autocomplete_atlas_location"]
    location: String,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
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
            let known = crate::atlas::LOCATION_CATALOGUE
                .iter()
                .map(|place| place.id)
                .collect::<Vec<_>>();
            let suggestion = suggest_known_value(&location, &known);
            let guidance = match suggestion {
                Some(value) => format!("Did you mean {value}? "),
                None => String::new(),
            };
            return send_private_title_panel(
                ctx,
                "Atlas Journey",
                title.as_deref(),
                format!(
                    "{guidance}That place is not in the Atlas. Known locations: {}.",
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
            return send_private_title_panel(
                ctx,
                "Atlas Journey",
                title.as_deref(),
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
    send_private_title_panel(
        ctx,
        "Atlas Journey",
        title.as_deref(),
        format!(
            "{}\n\n{}",
            message,
            crate::atlas::render(&result.snapshot, &prefix)
        ),
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn expedition(
    ctx: Context<'_>,
    #[description = "Choose an Expedition action."]
    #[autocomplete = "autocomplete_expedition_action"]
    action: Option<String>,
    #[description = "Route ID or run ID"] target: Option<String>,
    #[description = "Run mode for start, or checkpoint ID and choice for advance"]
    #[rest]
    details: Option<String>,
) -> Result<(), Error> {
    let player_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|id| id.to_string());
    let prefix = ctx.data().command_prefix(ctx.guild_id());
    let action = action
        .unwrap_or_else(|| "list".to_owned())
        .trim()
        .to_ascii_lowercase();

    match action.as_str() {
        "list" => {
            send_private_title_panel(
                ctx,
                "Expedition Routes",
                None,
                crate::expedition::render_catalogue(&prefix),
            )
            .await
        }
        "start" => {
            let route = target.as_deref().unwrap_or(crate::expedition::ROUTE_ID);
            if route != crate::expedition::ROUTE_ID {
                return send_private(ctx, "The current route is lantern-road. Other routes will appear when authored and released.").await;
            }
            let mode = details
                .as_deref()
                .unwrap_or("solo")
                .trim()
                .to_ascii_lowercase();
            if mode != "solo" && mode != "group" {
                return send_private(ctx, "Choose solo or group for the Lantern Road.").await;
            }
            ctx.data()
                .profiles()
                .get_or_create(&player_id, &ctx.author().name)
                .await?;
            ctx.data().atlas().snapshot(&player_id).await?;
            let shared = mode == "group";
            let run = ctx
                .data()
                .expeditions()
                .start(&player_id, guild_id.as_deref(), shared)
                .await?;
            if shared {
                let safe_prefix = escape_user_text(&prefix);
                ctx.say(format!(
                    "**Lantern Road group run #{} is recruiting.** Share the run ID with up to three friends. Each person must join explicitly with {}expedition join {}. The host can begin when everyone is ready. Personal Atlas progress is not shown here.",
                    run.run_id, safe_prefix, run.run_id
                )).await?;
                Ok(())
            } else {
                send_private_title_panel(
                    ctx,
                    "Lantern Road Expedition",
                    None,
                    crate::expedition::render(&run, &prefix),
                )
                .await
            }
        }
        "join" => {
            let Some(guild) = guild_id.as_deref() else {
                return send_private(
                    ctx,
                    "Shared Expeditions can only be joined inside their Discord server.",
                )
                .await;
            };
            let id = parse_expedition_id(target.as_deref())?;
            ctx.data()
                .profiles()
                .get_or_create(&player_id, &ctx.author().name)
                .await?;
            ctx.data().atlas().snapshot(&player_id).await?;
            let run = ctx.data().expeditions().join(&player_id, guild, id).await?;
            send_private_title_panel(
                ctx,
                "Lantern Road Expedition",
                None,
                crate::expedition::render(&run, &prefix),
            )
            .await
        }
        "begin" | "status" | "pause" | "resume" | "leave" | "cancel" => {
            let id = parse_expedition_id(target.as_deref())?;
            let run = match action.as_str() {
                "begin" => {
                    ctx.data()
                        .expeditions()
                        .begin(&player_id, guild_id.as_deref(), id)
                        .await?
                }
                "status" => {
                    ctx.data()
                        .expeditions()
                        .snapshot(&player_id, guild_id.as_deref(), id)
                        .await?
                }
                "pause" => {
                    ctx.data()
                        .expeditions()
                        .pause(&player_id, guild_id.as_deref(), id)
                        .await?
                }
                "resume" => {
                    ctx.data()
                        .expeditions()
                        .resume(&player_id, guild_id.as_deref(), id)
                        .await?
                }
                "leave" => {
                    ctx.data()
                        .expeditions()
                        .leave(&player_id, guild_id.as_deref(), id)
                        .await?
                }
                "cancel" => {
                    ctx.data()
                        .expeditions()
                        .cancel(&player_id, guild_id.as_deref(), id)
                        .await?
                }
                _ => unreachable!(),
            };
            send_private_title_panel(
                ctx,
                "Lantern Road Expedition",
                None,
                crate::expedition::render(&run, &prefix),
            )
            .await
        }
        "advance" => {
            let id = parse_expedition_id(target.as_deref())?;
            let Some(details) = details else {
                return send_private(ctx, "Use expedition advance RUN_ID CHECKPOINT_ID CHOICE.")
                    .await;
            };
            let mut parts = details.split_whitespace();
            let checkpoint = parts.next().ok_or("Enter the next checkpoint ID.")?;
            let choice = parts.next().ok_or("Enter one of the checkpoint choices.")?;
            if parts.next().is_some() {
                return send_private(ctx, "Enter one checkpoint ID and one choice only.").await;
            }
            let run = ctx
                .data()
                .expeditions()
                .advance(&player_id, guild_id.as_deref(), id, checkpoint, choice)
                .await?;
            send_private_title_panel(
                ctx,
                "Lantern Road Expedition",
                None,
                crate::expedition::render(&run, &prefix),
            )
            .await
        }
        _ => {
            let suggestion = suggest_known_value(
                action.as_str(),
                &[
                    "list", "start", "join", "begin", "status", "advance", "pause", "resume",
                    "leave", "cancel",
                ],
            );
            let guidance = suggestion
                .map(|value| format!("Did you mean {value}? "))
                .unwrap_or_default();
            send_private(
                ctx,
                format!("{guidance}Use {0}expedition list, start lantern-road solo|group, join RUN_ID, begin RUN_ID, status RUN_ID, advance RUN_ID CHECKPOINT_ID CHOICE, pause RUN_ID, resume RUN_ID, leave RUN_ID, or cancel RUN_ID.", escape_user_text(&prefix)),
            ).await
        }
    }
}

fn parse_expedition_id(value: Option<&str>) -> Result<i64, Error> {
    value
        .and_then(|text| text.trim().parse::<i64>().ok())
        .filter(|id| *id > 0)
        .ok_or_else(|| "Enter a valid numeric Expedition run ID.".into())
}

#[poise::command(prefix_command, slash_command)]
pub async fn games(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
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
    let message = format!(
        "First completion of each game type grants {} Moonbits once per player. Repeat completions do not grant more. Use {prefix}wallet to view your balance.\n\n{entries}",
        crate::wallet::FIRST_COMPLETION_REWARD
    );
    send_title_panel(
        ctx,
        "Moonbyte Game Library",
        title.as_deref(),
        &message,
        false,
    )
    .await
}

#[poise::command(prefix_command, slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Choose a game from the playable library."]
    #[autocomplete = "autocomplete_game"]
    game: String,
) -> Result<(), Error> {
    let game = match GameKind::parse(&game) {
        Ok(game) => game,
        Err(_) => {
            let known = GameKind::ALL
                .iter()
                .map(|kind| kind.definition().id)
                .collect::<Vec<_>>();
            let suggestion = suggest_known_value(&game, &known);
            let guidance = match suggestion {
                Some(value) => format!("Did you mean {value}? "),
                None => String::new(),
            };
            return send_private(
                ctx,
                format!("{guidance}That game is not available yet. Use .games to browse the playable library."),
            )
            .await;
        }
    };
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
    ctx.send(game_reply(
        format!("{intro}\n\n{}", snapshot.render()),
        &snapshot,
    ))
    .await?;
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
    ctx.send(game_reply(
        format!("You joined the game.\n\n{}", snapshot.render()),
        &snapshot,
    ))
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
    ctx.send(game_reply(snapshot.render(), &snapshot)).await?;
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
    ctx.send(game_reply(snapshot.render(), &snapshot)).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn sessions(
    ctx: Context<'_>,
    #[description = "Optional heading for this result. Leave blank to use Moonbyte's default title."]
    #[max_length = 256]
    #[rest]
    title: Option<String>,
) -> Result<(), Error> {
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
        return send_private_title_panel(
            ctx,
            "Active Game Sessions",
            title.as_deref(),
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

    send_private_title_panel(
        ctx,
        "Active Game Sessions",
        title.as_deref(),
        format!(
            "**Your active sessions in {scope}**\n\n{entries}\n\nUse `session <id>` to inspect a board or `action <id> <input>` to play. Showing up to 10 most recently updated sessions."
        ),
    )
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
    ctx.send(game_reply(
        format!("Session ended.\n\n{}", snapshot.render()),
        &snapshot,
    ))
    .await?;
    Ok(())
}

pub fn all() -> Vec<poise::Command<Data, Error>> {
    let mut tavern_command = tavern();
    tavern_command.aliases = vec!["t".into()].into();
    let mut games_command = games();
    games_command.aliases = vec!["g".into()].into();
    let mut play_command = play();
    play_command.aliases = vec!["p".into()].into();
    let mut join_command = join();
    join_command.aliases = vec!["j".into()].into();
    let mut leave_command = leave();
    leave_command.aliases = vec!["l".into()].into();
    let mut atlas_command = atlas();
    atlas_command.aliases = vec!["a".into()].into();
    let mut travel_command = travel();
    travel_command.aliases = vec!["tr".into()].into();
    let mut profile_command = profile();
    profile_command.aliases = vec!["me".into()].into();
    let mut wallet_command = wallet();
    wallet_command.aliases = vec!["w".into()].into();
    let mut stats_command = stats();
    stats_command.aliases = vec!["st".into()].into();
    let mut achievements_command = achievements();
    achievements_command.aliases = vec!["ach".into()].into();
    let mut quests_command = quests();
    quests_command.aliases = vec!["q".into()].into();
    let mut wayfarer_command = wayfarer();
    wayfarer_command.aliases = vec!["wf".into()].into();

    let mut prefix_command = prefix();
    prefix_command.required_permissions = poise::serenity_prelude::Permissions::MANAGE_GUILD;
    prefix_command.default_member_permissions = poise::serenity_prelude::Permissions::MANAGE_GUILD;

    vec![
        help(),
        about(),
        ping(),
        terms(),
        consent(),
        prefix_command,
        privacy(),
        privacy_data(),
        privacy_export(),
        privacy_delete(),
        profile_command,
        profile_edit(),
        settings(),
        wallet_command,
        achievements_command,
        inventory(),
        collection(),
        craft(),
        quests_command,
        quest(),
        wayfarer_command,
        companion(),
        rivals(),
        rival(),
        stats_command,
        tavern_command,
        atlas_command,
        chronicle(),
        travel_command,
        expedition(),
        games_command,
        play_command,
        join_command,
        action(),
        session(),
        sessions(),
        leave_command,
    ]
}

/// Sends a compact Discord embed panel for substantial command results.
/// Slash invocations are ephemeral; prefix invocations retain Discord's normal message visibility.
fn panel_title(override_title: Option<&str>, fallback: &str) -> String {
    let value = override_title
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .unwrap_or(fallback);
    let collapsed = value.split_whitespace().collect::<Vec<_>>().join(" ");
    escape_user_text(&collapsed).chars().take(256).collect()
}

fn panel_description(content: &str) -> String {
    let content = content.trim();
    let Some((heading, remainder)) = content.split_once('\n') else {
        return content.to_owned();
    };
    let Some(heading) = heading.strip_prefix("**") else {
        return content.to_owned();
    };
    let Some(close) = heading.find("**") else {
        return content.to_owned();
    };

    let suffix = heading[close + 2..].trim();
    let remainder = remainder.trim_start();
    match (suffix.is_empty(), remainder.is_empty()) {
        (true, true) => String::new(),
        (true, false) => remainder.to_owned(),
        (false, true) => suffix.to_owned(),
        (false, false) => format!("{suffix}\n\n{remainder}"),
    }
}

fn build_title_panel(fallback: &str, override_title: Option<&str>, content: &str) -> CreateEmbed {
    let description = panel_description(content);
    let description = if description.is_empty() {
        "No additional details.".to_owned()
    } else {
        description.chars().take(4096).collect()
    };

    CreateEmbed::new()
        .title(panel_title(override_title, fallback))
        .description(description)
}

async fn send_title_panel(
    ctx: Context<'_>,
    fallback: &str,
    override_title: Option<&str>,
    description: &str,
    ephemeral: bool,
) -> Result<(), Error> {
    let embed = build_title_panel(fallback, override_title, description);
    if ctx.prefix() == "/" {
        ctx.send(poise::CreateReply::new().embed(embed).ephemeral(ephemeral))
            .await?;
    } else if ephemeral && ctx.guild_id().is_some() {
        let dm = ctx.author().create_dm_channel(ctx.http()).await?;
        dm.send_message(ctx.http(), CreateMessage::new().embed(embed))
            .await?;
        ctx.say("I sent that panel to your DMs.").await?;
    } else {
        ctx.send(poise::CreateReply::new().embed(embed)).await?;
    }
    Ok(())
}

async fn send_private_title_panel(
    ctx: Context<'_>,
    fallback: &str,
    override_title: Option<&str>,
    description: impl Into<String>,
) -> Result<(), Error> {
    let embed = build_title_panel(fallback, override_title, &description.into());
    if ctx.guild_id().is_some() {
        let dm = ctx.author().create_dm_channel(ctx.http()).await?;
        dm.send_message(ctx.http(), CreateMessage::new().embed(embed))
            .await?;
        ctx.say("I sent those details to your DMs.").await?;
    } else {
        ctx.send(poise::CreateReply::new().embed(embed)).await?;
    }
    Ok(())
}

fn normalise_autocomplete_text(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect()
}

fn autocomplete_value_matches(partial: &str, label: &str, value: &str) -> bool {
    let query = normalise_autocomplete_text(partial);
    query.is_empty()
        || normalise_autocomplete_text(label).contains(&query)
        || normalise_autocomplete_text(value).contains(&query)
}

fn build_autocomplete_response(
    partial: &str,
    options: Vec<(String, String)>,
) -> CreateAutocompleteResponse {
    let choices = options
        .into_iter()
        .filter(|(label, value)| autocomplete_value_matches(partial, label, value))
        .take(25)
        .map(|(label, value)| {
            AutocompleteChoice::new(
                label.chars().take(100).collect::<String>(),
                value.chars().take(100).collect::<String>(),
            )
        })
        .collect();

    CreateAutocompleteResponse::new().set_choices(choices)
}

async fn autocomplete_craft_recipe(
    _ctx: Context<'_>,
    partial: &str,
) -> CreateAutocompleteResponse {
    let options = crate::inventory::crafting_recipe_options()
        .into_iter()
        .map(|recipe| (recipe.label, recipe.recipe_id))
        .collect();
    build_autocomplete_response(partial, options)
}

async fn autocomplete_game(
    _ctx: Context<'_>,
    partial: &str,
) -> CreateAutocompleteResponse {
    let options = GameKind::ALL
        .iter()
        .map(|game| {
            let definition = game.definition();
            (
                format!("{} · {}", definition.name, definition.players),
                definition.id.to_owned(),
            )
        })
        .collect();
    build_autocomplete_response(partial, options)
}

async fn autocomplete_quest_action(
    _ctx: Context<'_>,
    partial: &str,
) -> CreateAutocompleteResponse {
    let options = [
        ("Accept · Start tracking a daily quest", "accept"),
        ("Claim · Collect a completed quest's fixed reward", "claim"),
    ]
    .into_iter()
    .map(|(label, value)| (label.to_owned(), value.to_owned()))
    .collect();
    build_autocomplete_response(partial, options)
}

async fn autocomplete_rival_action(
    _ctx: Context<'_>,
    partial: &str,
) -> CreateAutocompleteResponse {
    let options = [
        ("Declare · Invite a player to a mutual rivalry", "declare"),
        ("Accept · Accept an invitation you received", "accept"),
        ("Decline · Decline an invitation", "decline"),
    ]
    .into_iter()
    .map(|(label, value)| (label.to_owned(), value.to_owned()))
    .collect();
    build_autocomplete_response(partial, options)
}

async fn autocomplete_expedition_action(
    _ctx: Context<'_>,
    partial: &str,
) -> CreateAutocompleteResponse {
    let options = [
        ("List · Browse available routes", "list"),
        ("Start · Create a solo or group run", "start"),
        ("Join · Join a recruiting group", "join"),
        ("Begin · Start a recruiting run", "begin"),
        ("Status · Inspect a run", "status"),
        ("Advance · Submit the next checkpoint choice", "advance"),
        ("Pause · Pause a run", "pause"),
        ("Resume · Resume a paused run", "resume"),
        ("Leave · Leave a group run", "leave"),
        ("Cancel · Cancel a run you host", "cancel"),
    ]
    .into_iter()
    .map(|(label, value)| (label.to_owned(), value.to_owned()))
    .collect();
    build_autocomplete_response(partial, options)
}

async fn autocomplete_atlas_location(
    _ctx: Context<'_>,
    partial: &str,
) -> CreateAutocompleteResponse {
    let options = crate::atlas::LOCATION_CATALOGUE
        .iter()
        .map(|location| {
            (
                format!("{} · {}", location.name, location.region),
                location.id.to_owned(),
            )
        })
        .collect();
    build_autocomplete_response(partial, options)
}

async fn autocomplete_profile_visibility(
    _ctx: Context<'_>,
    partial: &str,
) -> CreateAutocompleteResponse {
    let options = [
        ("Private · Only you can view your profile", "private"),
        ("Public · Allow your profile to be viewed", "public"),
    ]
    .into_iter()
    .map(|(label, value)| (label.to_owned(), value.to_owned()))
    .collect();
    build_autocomplete_response(partial, options)
}

/// Returns a conservative suggestion for a misspelled command argument.
/// Suggestions are advisory only: the user must submit the corrected value themselves.
fn suggest_known_value<'a>(input: &str, choices: &'a [&'a str]) -> Option<&'a str> {
    let input = input.trim().to_ascii_lowercase();
    if input.is_empty() {
        return None;
    }

    choices
        .iter()
        .copied()
        .map(|choice| (edit_distance(&input, choice), choice))
        .filter(|(distance, choice)| {
            let longest = input.len().max(choice.len());
            *distance <= 3 && distance.saturating_mul(100) <= longest.saturating_mul(40)
        })
        .min_by_key(|(distance, _)| *distance)
        .map(|(_, choice)| choice)
}

fn edit_distance(left: &str, right: &str) -> usize {
    let mut previous = (0..=right.len()).collect::<Vec<_>>();
    let mut current = vec![0; right.len() + 1];

    for (left_index, left_byte) in left.bytes().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_byte) in right.bytes().enumerate() {
            let substitution = previous[right_index] + usize::from(left_byte != right_byte);
            current[right_index + 1] = (previous[right_index + 1] + 1)
                .min(current[right_index] + 1)
                .min(substitution);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[right.len()]
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
         {safe_prefix}terms / /terms — read the Terms of Service\n\
         {safe_prefix}consent accept / /consent — agree to the current Terms and Privacy Policy\n\
         {safe_prefix}prefix [set|channel|reset] / /prefix — configure this server's prefix and command channel (Manage Server)\n\
         {safe_prefix}privacy / /privacy — read the full Privacy Policy\n\
         {safe_prefix}privacy_data / /privacy_data — inspect your stored data\n\
         {safe_prefix}privacy_export / /privacy_export — receive a private JSON export\n\
         {safe_prefix}privacy_delete [DELETE] / /privacy_delete — confirm profile deletion\n\
         {safe_prefix}profile / /profile — view your private player profile\n\
         {safe_prefix}profile_edit <name> | <bio> / /profile_edit — edit your profile\n\
         {safe_prefix}settings <visibility> <show_stats> <quiet_mode> / /settings — update preferences\n\
         {safe_prefix}wallet / /wallet — view your earned-only Moonbits balance\n\
         {safe_prefix}achievements / /achievements — view your unlocked achievements\n\
         {safe_prefix}inventory / /inventory — inspect your account-bound keepsakes\n\
         {safe_prefix}collection / /collection — view your Atlas postcard collection\n\
         {safe_prefix}craft wayfarer-lantern / /craft — make a fixed, one-time keepsake\n\
         {safe_prefix}stats / /stats — view completed game statistics\n\
         {safe_prefix}quests / /quests — open today's optional quest board\n\
         {safe_prefix}quest accept|claim <id> / /quest — accept or claim a daily quest\n\
         {safe_prefix}wayfarer / /wayfarer — view your non-resetting journey record\n\
         {safe_prefix}companion / /companion — view cosmetic companion milestones\n\
         {safe_prefix}rivals / /rivals — view private rivalry invitations and records\n\
         {safe_prefix}rival declare|accept|decline <user/id> / /rival — manage mutual rivalries\n\
         {safe_prefix}tavern / /tavern — open the Tavern and live Wayfinder\n\
         {safe_prefix}atlas / /atlas — inspect your world map and known routes\n\
         {safe_prefix}travel <location> / /travel — follow a connected Atlas route\n\
         {safe_prefix}expedition list|start|join|begin|status|advance|pause|resume|leave|cancel / /expedition — play the Lantern Road\n\
         {safe_prefix}games / /games — browse the playable library\n\
         {safe_prefix}play <game> / /play — start a solo game or multiplayer lobby\n\
         {safe_prefix}join <session> / /join — join an open lobby in this server\n\
         {safe_prefix}action <session> <input> / /action — submit a legal game action\n\
         {safe_prefix}session <session> / /session — inspect your own session\n\
         {safe_prefix}sessions / /sessions — list your active sessions here\n\
         {safe_prefix}leave <session> / /leave — end a session you joined\n\
         Prefix shortcuts (prefix commands only):\n\
         {safe_prefix}t → {safe_prefix}tavern · {safe_prefix}g → {safe_prefix}games · {safe_prefix}p → {safe_prefix}play · {safe_prefix}j → {safe_prefix}join · {safe_prefix}l → {safe_prefix}leave\n\
         {safe_prefix}a → {safe_prefix}atlas · {safe_prefix}tr → {safe_prefix}travel · {safe_prefix}me → {safe_prefix}profile · {safe_prefix}w → {safe_prefix}wallet · {safe_prefix}st → {safe_prefix}stats · {safe_prefix}ach → {safe_prefix}achievements · {safe_prefix}q → {safe_prefix}quests · {safe_prefix}wf → {safe_prefix}wayfarer.\n\
         Help, Terms, consent and all privacy commands deliberately have no shortcuts.\n\n\
         Multiplayer sessions are scoped to their originating server. Sessions expire \
         after 24 hours of inactivity."
    )
}

const PRIVACY_OVERVIEW: &str = "\
**Moonbyte privacy — current build**\n\n\
Moonbyte processes Discord interactions to run commands and games. PostgreSQL stores \
player profiles, session snapshots, completed-game result metadata, per-game aggregate \
statistics, earned-only wallet/ledger records, unlocked achievements, personal Atlas location/discovery records, private Chronicle fragments, account-bound postcards and their acquisition provenance, and Lantern Road Expedition participation and checkpoint history. Sessions include game state and participant IDs, are scoped to their originating \
server, and expire after 24 hours without activity. Completed result rows expire with their \
parent session; aggregate statistics and earned wallet/ledger records remain until the player \
requests deletion. The first completion of each game type earns 10 Moonbits once per player. \
Achievements such as first completion, first victory, completing all four games, ten total \
games and five wins unlock once and remain until account deletion; they do not grant currency. \
Atlas discoveries grant account-bound keepsakes and fixed materials with recorded provenance. Use `.inventory` and `.collection` to inspect them; `.craft wayfarer-lantern` consumes one of each material to make one lantern. Trading is not enabled.\n\n\
Player profiles store a Discord user ID, display name, optional bio, private-by-default \
visibility and two preference flags. Use `.privacy_data` to inspect your current records, \
`.privacy_export` to receive a private JSON export, and `.privacy_delete` to begin confirmed \
deletion. Profile and privacy responses are sent by DM when invoked in a server.\n\n\
Deletion removes the profile, per-game statistics, achievements, Moonbits wallet/ledger records, \
Atlas location/discovery records, private Chronicle fragments, postcards, fixed materials, crafted keepsakes and their provenance, your Expedition participation, and every session/result snapshot containing your ID from the current PostgreSQL schema. It can end shared sessions. It cannot delete \
Discord's own messages, Discord account data, other participants' copies, or backups outside \
the current database.\n\n\
Moonbyte does not claim end-to-end encryption or encryption at rest. This is a summary of the \
current implementation, not a statement about Discord's independent data practices.";

#[cfg(test)]
mod tests {
    use super::{
        PrefixUpdate, escape_user_text, help_message, parse_discord_user_id, parse_prefix_update,
        render_privacy_overview,
    };
    use crate::game::GameKind;

    #[test]
    fn game_component_ids_are_versioned_and_strictly_parsed() {
        assert_eq!(
            super::parse_game_component_id("mbg:MB-123:7:4"),
            Some(("MB-123".to_owned(), 7, 4))
        );
        assert_eq!(super::parse_game_component_id("other:MB-123:7:4"), None);
        assert_eq!(super::parse_game_component_id("mbg:MB-123:7:4:extra"), None);
        assert_eq!(super::parse_game_component_id("mbg::7:4"), None);
        assert_eq!(super::parse_game_component_id("mbg:MB-123:bad:4"), None);
    }

    #[test]
    fn game_button_values_map_to_the_existing_text_action_contract() {
        assert_eq!(
            super::game_action_input(GameKind::TicTacToe, 0),
            Some("1,1".to_owned())
        );
        assert_eq!(
            super::game_action_input(GameKind::TicTacToe, 8),
            Some("3,3".to_owned())
        );
        assert_eq!(
            super::game_action_input(GameKind::ConnectFour, 7),
            Some("7".to_owned())
        );
        assert_eq!(
            super::game_action_input(GameKind::MemoryMatch, 8),
            Some("8".to_owned())
        );
        assert_eq!(super::game_action_input(GameKind::WordScramble, 1), None);
        assert_eq!(super::game_action_input(GameKind::TicTacToe, 9), None);
    }

    #[test]
    fn help_lists_commands_registered_in_this_build() {
        let message = help_message(".");
        for command in [
            ".help / /help",
            ".about / /about",
            ".ping / /ping",
            ".terms / /terms",
            ".consent accept / /consent",
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
            ".inventory / /inventory",
            ".collection / /collection",
            ".craft wayfarer-lantern / /craft",
            ".stats / /stats",
            ".quests / /quests",
            ".quest accept|claim <id> / /quest",
            ".wayfarer / /wayfarer",
            ".companion / /companion",
            ".rivals / /rivals",
            ".rival declare|accept|decline <user/id> / /rival",
            ".tavern / /tavern",
            ".atlas / /atlas",
            ".travel <location> / /travel",
            ".expedition list|start|join|begin|status|advance|pause|resume|leave|cancel / /expedition",
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
        assert!(message.contains(".q → .quests"));
        assert!(message.contains(".wf → .wayfarer"));
        assert!(message.contains(".t → .tavern"));
        assert!(message.contains(".g → .games"));
        assert!(message.contains(
            "Help, Terms, consent and all privacy commands deliberately have no shortcuts."
        ));
    }

    #[test]
    fn prefix_shortcuts_are_registered_and_legal_privacy_commands_stay_full_length() {
        let commands = super::all();
        let aliases = |name: &str| {
            commands
                .iter()
                .find(|command| command.name.as_ref() == name)
                .unwrap_or_else(|| panic!("missing command {name}"))
                .aliases
                .iter()
                .map(|alias| alias.as_ref())
                .collect::<Vec<_>>()
        };

        assert_eq!(aliases("tavern"), vec!["t"]);
        assert_eq!(aliases("games"), vec!["g"]);
        assert_eq!(aliases("play"), vec!["p"]);
        assert_eq!(aliases("join"), vec!["j"]);
        assert_eq!(aliases("leave"), vec!["l"]);
        assert_eq!(aliases("atlas"), vec!["a"]);
        assert_eq!(aliases("travel"), vec!["tr"]);
        assert_eq!(aliases("profile"), vec!["me"]);
        assert_eq!(aliases("wallet"), vec!["w"]);
        assert_eq!(aliases("stats"), vec!["st"]);
        assert_eq!(aliases("achievements"), vec!["ach"]);
        assert!(aliases("inventory").is_empty());
        assert!(aliases("collection").is_empty());
        assert_eq!(aliases("quests"), vec!["q"]);
        assert_eq!(aliases("wayfarer"), vec!["wf"]);

        for name in [
            "help",
            "terms",
            "consent",
            "privacy",
            "privacy_data",
            "privacy_export",
            "privacy_delete",
        ] {
            assert!(
                aliases(name).is_empty(),
                "{name} must not have a short prefix alias"
            );
        }
    }

    #[test]
    fn discord_user_id_parser_accepts_mentions_and_rejects_arbitrary_text() {
        assert_eq!(
            parse_discord_user_id("<@123456789012345678>"),
            Some("123456789012345678".to_owned())
        );
        assert_eq!(
            parse_discord_user_id("<@!123456789012345678>"),
            Some("123456789012345678".to_owned())
        );
        assert_eq!(
            parse_discord_user_id("123456789012345678"),
            Some("123456789012345678".to_owned())
        );
        assert_eq!(parse_discord_user_id("@everyone"), None);
        assert_eq!(parse_discord_user_id("123"), None);
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
    fn command_argument_suggestions_are_conservative_and_never_auto_correct() {
        assert_eq!(
            super::suggest_known_value("wayfarer-lantarn", &["wayfarer-lantern"]),
            Some("wayfarer-lantern")
        );
        assert_eq!(
            super::suggest_known_value(
                "conect-four",
                &[
                    "tic-tac-toe",
                    "connect-four",
                    "memory-match",
                    "word-scramble",
                ],
            ),
            Some("connect-four")
        );
        assert_eq!(
            super::suggest_known_value("resmue", &["pause", "resume", "cancel"]),
            Some("resume")
        );
        assert_eq!(
            super::suggest_known_value("wayfarer-lantern", &["wayfarer-lantern"]),
            Some("wayfarer-lantern")
        );
        assert_eq!(
            super::suggest_known_value("unrelated", &["wayfarer-lantern"]),
            None
        );
        assert_eq!(super::edit_distance("craft", "crfat"), 2);
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

#[cfg(test)]
mod title_panel_tests {
    use super::{panel_description, panel_title};

    #[test]
    fn custom_title_is_normalised_mention_safe_and_bounded() {
        assert_eq!(
            panel_title(Some("  @everyone  My _Workshop_  "), "Default"),
            "@\u{200B}everyone My \\_Workshop\\_"
        );
        assert_eq!(panel_title(Some("   "), "Default"), "Default");
        assert_eq!(
            panel_title(Some(&"x".repeat(300)), "Default")
                .chars()
                .count(),
            256
        );
    }

    #[test]
    fn first_markdown_heading_moves_into_the_panel_title_without_losing_its_suffix() {
        assert_eq!(
            panel_description("**Your Satchel** · 2 item records\n\nMosslight Silk"),
            "· 2 item records\n\nMosslight Silk"
        );
        assert_eq!(panel_description("A simple result"), "A simple result");
    }
}


#[cfg(test)]
mod autocomplete_tests {
    use super::autocomplete_value_matches;

    #[test]
    fn autocomplete_matches_labels_and_values_without_case_or_separator_sensitivity() {
        assert!(autocomplete_value_matches(
            "wayfarer lantern",
            "Wayfarer's Lantern · 1× Mosslight Silk + 1× Tideglass",
            "wayfarer-lantern"
        ));
        assert!(autocomplete_value_matches(
            "TIDEGLASS",
            "Wayfarer's Lantern · 1× Mosslight Silk + 1× Tideglass",
            "wayfarer-lantern"
        ));
        assert!(autocomplete_value_matches("Lantern", "Wayfarer's Lantern", "wayfarer-lantern"));
    }

    #[test]
    fn empty_autocomplete_input_keeps_the_full_catalogue_available() {
        assert!(autocomplete_value_matches("", "Wayfarer's Lantern", "wayfarer-lantern"));
        assert!(autocomplete_value_matches("   ", "Wayfarer's Lantern", "wayfarer-lantern"));
    }

    #[test]
    fn unrelated_autocomplete_input_does_not_return_a_false_suggestion() {
        assert!(!autocomplete_value_matches("moonbridge", "Wayfarer's Lantern", "wayfarer-lantern"));
    }
}
