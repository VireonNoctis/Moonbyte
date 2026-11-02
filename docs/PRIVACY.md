# Moonbyte Privacy Policy and Data Practices

- Policy owner: Moonbyte maintainers.
- Status: Current runtime includes PostgreSQL-backed player profiles/preferences, game sessions/results/statistics, five achievement milestones, an earned-only Moonbits wallet/ledger, a five-location Atlas with personal discoveries, data inventory, private version 5 JSON export and confirmed deletion across all current player-owned tables. Inventory, quests, Expeditions, Vaults, spending and Bazaar systems remain planned.
- Last updated: 2026-11-02.
- Review this document against the actual code, deployment, schema, logs, and service providers before public launch.

## 1. Our stance

Moonbyte is designed around **privacy by default, data minimisation, transparency, and user control**.

We want a gaming bot that can remember a player's progress without collecting a dossier about that person. Data should exist because a feature needs it, not because it might be useful for advertising, profiling, or future monetisation.

Our commitments:
- We do not sell or rent personal data.
- We do not use player data for advertising or behavioural profiling.
- We do not build advertising trackers or third-party analytics into the bot.
- We do not use player messages or gameplay data to train AI models.
- We do not intentionally collect real names, email addresses, phone numbers, precise location, or contacts.
- We do not retain raw message content as general-purpose analytics.
- We provide a way to inspect, export, and request deletion of Moonbyte profile data.
- We keep guild-specific data scoped to the guild that created it.
- We explain technical limits honestly and do not make unsupported encryption claims.

The current runtime stores player profiles/preferences, game-session snapshots, completed result metadata, per-game statistics, achievement milestones, earned-only wallet/ledger records and personal Atlas location/discovery state in PostgreSQL. It does not yet persist inventory, quests, Expeditions, Vault state or Bazaar trades. The `.privacy_data`, `.privacy_export` and `.privacy_delete` commands inspect, export and delete records in the current player-owned schema. Every future persisted feature must extend the same inventory/export/deletion contract before release.

## 2. Data Moonbyte needs

The bot may process or store these categories when the relevant feature is enabled. The current runtime stores the profile, session, completed-result, aggregate-statistics, achievement, earned-only wallet/ledger and personal Atlas fields described below; the remaining categories are future design targets.

| Data category | Examples | Why it is needed |
| --- | --- | --- |
| Discord identifiers | User ID; guild ID; channel or message ID for an active game where necessary. | Identify the player, scope settings, and update an active game message. |
| Player profile | Discord user ID, player-selected display name, optional short bio, visibility and two preference flags. | Render and configure the player's profile. |
| Player game statistics | Discord user ID, game ID, completed count, wins, draws and update timestamp. | Show the player their own completed-game record. |
| Moonbits wallet | Discord user ID, non-negative balance and version counter. | Show fixed, earned-only game-completion milestones. |
| Moonbits ledger | Transaction ID, player ID, signed amount, fixed reason, source session, idempotency key and timestamp. | Audit each earned credit and prevent duplicate milestone awards. |
| Game state | Current turn, board state, action data needed to resume, participants, quest checkpoint. | Run, recover, and fairly resolve games. |
| Game results | Game ID, outcome, winner ID when applicable, participant IDs, action count and completion time. | Record completed sessions and derive requested per-game statistics. |
| Achievements | Achievement ID, source session ID and unlock timestamp. | Show fixed, server-authoritative milestones earned through play. |
| Atlas exploration | Current location ID, discovered location IDs, first discovery time, last visit time and visit count. | Resume the player's position and show their own charted places. |
| Future inventory and quests | Items, authored quest progress and checkpoints. | Not currently stored; requires reviewed persistence and privacy handling before release. |
| Guild settings | Enabled games, leaderboard visibility, channel configuration, server preferences. | Apply administrators' choices within that guild. |
| Operational diagnostics | Event type, command name, error category, timing, pseudonymous correlation ID. | Diagnose outages, bugs, rate limits, and performance without recording message contents. |
| Security records | Limited identifiers and event metadata associated with an incident. | Protect the bot and investigate abuse. |

Moonbyte should not require Lunar account linking, an email address, real name, or separate website account.

The persisted profile fields are the Discord user ID, display name, optional bio, visibility, statistics preference and quiet-mode preference. Profile visibility defaults to private. Session snapshots contain the session ID, game ID, player Discord IDs, optional guild ID, phase, serialised game state, action count, short result text and timestamps. Completed games create a `game_results` row and update `player_game_stats` for each participant in the same PostgreSQL transaction as the final session snapshot. Result rows are linked to their parent session and expire with it after 24 hours of inactivity; aggregate statistics remain until deletion. A player's first completion of each game type also inserts a unique `first_game_completion` ledger entry for 10 Moonbits and updates the wallet in that same transaction. Achievement checks run after aggregate-stat updates in the same transaction; the five current milestone records are unique per player and remain until deletion. The Atlas stores one current location and per-player discovery rows; the five-location route catalogue is static code, while current position, first-discovery time, last-visit time and visit counts are stored in PostgreSQL. Travel only follows a direct route and grants no rewards. Atlas state is personal rather than guild-scoped. Raw Discord message content is not saved as an analytics transcript. Sessions recover after restart. Statistics visibility and quiet-mode flags are saved preferences; the private `.stats` command is available, while public profile/leaderboard and optional notification surfaces are not implemented. Spending, transfers, items, quests, Expeditions and trading are not implemented.

## 3. Message and interaction content

Discord sends interaction data to the bot so it can process commands and game actions.

- Prefix commands require Discord's Message Content privileged intent. When enabled, the bot parses prefix and arguments in memory.
- Slash commands deliver structured interaction payloads.
- A game input such as a guess, quiz answer, or move may be stored as part of active game state when necessary to validate or resume that game.
- Raw messages must not be retained as a general-purpose transcript or analytics stream.
- Logs must not contain raw command bodies, message content, tokens, authorization headers, private export contents, or unnecessary user data.
- If a future feature needs to retain user-authored content, it requires a documented purpose, visible notice, a retention rule, and a deletion path where practical.

Discord independently processes data under its own policies. Moonbyte cannot control what Discord itself retains.

## 4. Permitted purposes

Data use is limited to delivering commands and games, maintaining active sessions and recovery, saving requested progress and statistics, maintaining guild settings and optional leaderboards, preventing abuse, debugging reliability through minimised diagnostics, and fulfilling the player's own export/deletion request.

We do not use this data for ad targeting, data brokerage, profiling unrelated to gameplay, or AI model training.

## 5. Sharing and external services

Moonbyte should not sell, rent, or share player data with advertisers, data brokers, or analytics companies.

Data necessarily passes through:
- **Discord:** the platform that hosts bot interactions and messages. Discord's own privacy practices apply.
- **Hosting and infrastructure providers:** the operator's selected host may process the runtime, database, and backups to provide the service.
- **Backup storage:** only if backups are enabled; access must be restricted and retention documented.

No additional external game API, analytics SDK, AI provider, or third-party data processor is approved by this policy. Any new provider must be reviewed, documented here, and disclosed before the integration is enabled.

The provider list describes the intended model; it does not claim a production hosting provider has already been selected.

## 6. Retention targets

| Data | Intended retention |
| --- | --- |
| Active session state | While active, with a maximum 24-hour recovery window after inactivity or interruption unless a game documents a different save model. |
| Completed game results | Expire with the parent session after 24 hours of inactivity; delete on account deletion. |
| Player settings and aggregate statistics | Until profile deletion. |
| Moonbits wallet and ledger | Until profile deletion; session expiry does not remove earned ledger entries. |
| Achievement milestones | Until account deletion. |
| Personal Atlas location and discovery history | Until account deletion. The authored five-location catalogue is static code, not player data. |
| Inventory, quests and other future progression | Not currently stored. Future retention must be documented with the feature. |
| Guild settings | While the bot remains installed and settings are needed; delete or anonymise after removal, target maximum 30 days. |
| Routine operational logs | Target maximum 30 days, then delete or aggregate without direct identifiers. |
| Security incident records | Only as long as reasonably necessary; restrict access and document the purpose. |
| Backups | Rotate on a documented schedule; deleted profile data should expire from backups within a target maximum of 30 days. |

These are implementation targets. Before launch, maintainers must configure and test cleanup jobs and backup rotation. Data must not be kept longer merely because storage is cheap.

## 7. Privacy command and user controls

The current prefix and slash commands provide:

| Command | Purpose |
| --- | --- |
| `.privacy` | Read a short privacy summary. |
| `.privacy_data` | Privately inspect profile, session, result, statistic, achievement, wallet/ledger and Atlas record counts. |
| `.privacy_export` | Receive a private version 5 JSON copy of profile, sessions, results, statistics, achievements, wallet/ledger and Atlas state/discoveries. |
| `.privacy_delete` | Read the deletion scope and confirmation instructions. |
| `.privacy_delete DELETE` | Confirm deletion of the profile, statistics, achievements, wallet/ledger, Atlas records and all session snapshots containing the requester. |

When invoked in a server, profile details, inventories, exports and deletion receipts are sent by direct message; they are never posted to the public channel. The export describes the current schema only and excludes other participants' Discord IDs and raw message content. If DM delivery fails, the command returns an error rather than exposing private data. The public `.privacy` overview may be shown in the invoking channel.

### Deletion flow

1. Explain that the profile, aggregate statistics, achievements, Moonbits wallet/ledger, Atlas location/discovery records and every session snapshot containing the requester's Discord ID will be removed.
2. Require the requester to run the explicit second command `.privacy_delete DELETE`.
3. Use the authenticated Discord command author as the account owner; arbitrary target IDs are not accepted.
4. Establish the private receipt channel before changing records when deletion is invoked in a server.
5. Lock all matching session rows and the player's current Atlas world-state row; delete linked `game_results`, session rows, `player_game_stats`, `player_achievements`, `wallet_ledger_entries`, `player_wallets`, `player_world_state`, and `player_atlas_discoveries`, then delete `player_profiles` in one PostgreSQL transaction.
6. Commit before reporting the actual deleted row counts.

The current player-owned schema is `player_profiles`, `game_sessions`, `game_results`, `player_game_stats`, `player_achievements`, `player_wallets`, `wallet_ledger_entries`, `player_world_state` and `player_atlas_discoveries`. Because session state is one shared JSONB snapshot, every stored session containing the requester is removed, which can end a session for co-participants. Result records are deleted with their session; aggregate statistics, achievements, wallet records and Atlas records are removed directly. Atlas rows also have foreign-key ownership by the player profile, so deleting the profile cannot leave orphaned discovery data. Future player-owned tables must be added to the same deletion transaction and export contract before those features ship. The user may still appear in Discord messages, copies held by other server members, or infrastructure backups. Moonbyte cannot delete Discord's own records.

## 8. Guild privacy

- Guild settings and server leaderboards must be keyed and queried by guild ID.
- A server administrator may inspect and delete that guild's configuration.
- Server administrators must not export or delete another user's global profile through guild controls.
- A guild leaderboard should expose only what its visibility setting permits.
- Private profile fields and exports must not be exposed in public game channels.
- Bot removal should trigger a documented cleanup path for guild-only configuration, subject to backup rotation.

## 9. Security protections

Current and required safeguards:
- Keep bot credentials in environment variables or a deployment secret manager.
- Never commit real tokens or production database files.
- Restrict database and backup filesystem permissions.
- Use protected transport for Discord connections and the deployment's protected connections.
- Restrict operator access by least privilege.
- Use parameterised database queries and validated input.
- Check session membership, turn ownership, and guild scope server-side.
- Apply user/guild rate limits.
- Redact secrets and private payloads from structured logs.
- Test export, deletion, retention cleanup, and cross-guild isolation.

### Encryption: what we do and do not claim

**Moonbyte does not claim end-to-end encryption for bot data.** Discord and the Moonbyte process must be able to read and process commands, game actions, and session state to provide the service. The bot therefore cannot truthfully promise that only the user can decrypt all gameplay data.

PostgreSQL does not automatically prove that data is encrypted at rest. The project has not selected a production host or independently verified database-volume, backup and key-management encryption. TLS behavior depends on the configured connection URL and server. Do not advertise encryption at rest or end-to-end encryption unless the actual deployment, backups and key management have been checked.

## 10. Safety and community

Game content should remain age-appropriate and avoid graphic or sexual content. Provide server moderation controls. Do not design mechanics that encourage excessive play, spending, wagering, or disclosure of private information.

## 11. Policy changes and contact

A change to data categories, purposes, retention, or external providers requires code review and an update to this policy. Significant changes should be announced in release notes before they take effect where practical.

For privacy questions that cannot be completed through the bot, use the repository's published support contact once one is configured. Do not post private personal data in public issues.

## 12. Production launch checklist

Before public release, verify:
- Database schema matches the documented data inventory.
- Actual log fields match the no-content-logging rule.
- Prefix processing does not persist raw messages.
- Export contains every documented user-owned record and is delivered privately.
- Deletion covers every user-owned table and index and has integration tests.
- Guild settings and leaderboards cannot leak across guilds.
- Retention and backup rotation run and are observable.
- Hosting providers and additional processors are documented.
- Database and backup access controls are verified.
- Public privacy claims match the deployed implementation.
- Profile export and deletion cover every currently persisted player table, including Atlas location and discovery records.
