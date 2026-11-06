# Moonbyte Privacy Policy and Data Practices

- Policy owner: Moonbyte maintainers.
- Version: 1.0.0.
- Status note: initial pre-launch policy baseline. Review again after the Discord application, hosting provider, database provider and operational logging configuration are selected.
- Source status: this repository contains pre-launch Rust/PostgreSQL implementation work for the runtime, games, profiles, Atlas/Chronicle, five account-bound postcards, two fixed Atlas materials, one deterministic recipe, Wayfarer, Expedition and Mosslight Resonance Puzzle Vault foundations. No Discord bot application has been registered or deployed. The current player-owned schema has private data inventory, version 12 JSON export and confirmed deletion coverage, including opt-in Guild Project memberships.
- Last updated: 2026-11-07.
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

The current runtime stores player profiles/preferences, versioned Terms of Service and Privacy Policy acceptance, game-session snapshots, completed result metadata, per-game statistics, achievement milestones, earned-only wallet/ledger records, personal Atlas location/discovery state, private Chronicle fragment records and account-bound Atlas postcard inventory in PostgreSQL. It also persists Wayfarer activity, daily quest rotations/progress and private rivalry and Expedition records. Additional Expedition routes, authored story quests, more crafting recipes, additional Vault blueprints and Bazaar trades are not yet implemented. Mosslight Resonance is the first implemented solo Vault. The `.privacy_data`, `.privacy_export` and `.privacy_delete` commands inspect, export and delete records in the current player-owned schema. Guild command-routing records remain separate from player-owned data. The first Guild Project board is server-owned and keyed by guild ID; membership is personal, included in export version 12 and removed on account deletion. Contribution rows contain no contributor ID, and the private board exposes only aggregate counts. Deletion clears the player's Expedition checkpoint actor references but does not roll back anonymous server progress. Actual bot removal removes the guild board; a temporary guild outage does not.

Every future persisted feature must extend the same inventory/export/deletion contract before release.

## 2. Data Moonbyte needs

The bot may process or store these categories when the relevant feature is enabled. The current runtime stores the profile, session, completed-result, aggregate-statistics, achievement, earned-only wallet/ledger, personal Atlas, Wayfarer, quest, rivalry, Expedition and Mosslight Resonance Vault fields described below; remaining categories are future design targets.

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
| Chronicle archive | Authored fragment ID, linked Atlas location and first-unlock timestamp. | Preserve the player's private lore collection without storing arbitrary user-authored content. |
| Wayfarer activity | Player ID and one activity date per calendar day. | Record cumulative, non-punitive player journey days. |
| Daily quest rotation | Player ID, date, private UUID seed and SHA-512 fingerprint. | Keep a stable three-objective board for that player/day; the private seed is never exported. |
| Daily quest progress | Quest ID, date, status, progress, target, fixed reward amount and completion/claim timestamps. | Track objectives the player explicitly accepted and prevent duplicate reward claims. |
| Rivalry records | Two participant IDs, invitation sender/status, match/win/draw counters and timestamps. | Store private head-to-head records after both participants opt in. |
| Expedition runs | Route ID/version, optional guild ID, host, shared flag, state, checkpoint position, row version and timestamps. | Resume a solo or group Lantern Road route and validate state transitions. |
| Expedition participants | Run ID, participant ID, active flag, join/leave timestamps and last checkpoint. | Apply shared progress only to people who explicitly joined. |
| Expedition checkpoint history | Run ID/version, checkpoint ID/index, stable choice ID, completing participant and timestamp. | Prevent duplicate checkpoint effects and retain route history. |
| Puzzle Vault runs and save stones | Player ID, fixed blueprint ID/version, run state, current chamber index, row version and timestamps; solved room ID/index and timestamp. | Resume a private authored puzzle sequence and prevent duplicate chamber completion. Wrong guesses and answer text are not stored. |
| Account-bound inventory | Stable item ID, quantity, source type, source reference and acquisition timestamp for Atlas postcards. | Show a player's keepsakes and preserve verified discovery provenance; postcards cannot be transferred or traded. |
| Future authored quests and resources | Authored quest graph, story progress, checkpoints and future crafting materials. | Not currently stored; requires reviewed persistence and privacy handling before release. |
| Guild settings | Guild ID, custom command prefix, optional command-channel ID, last editor and configuration timestamps. | Apply moderators' command-routing choices within that guild. |
| Guild Project memberships | Guild ID, fixed project key and join timestamp linked to the player's Discord ID. | Allow explicit enrolment/leave; included in export and deletion inventory. |
| Guild Project contributions | Guild project ID, source Expedition locator, checkpoint ID and timestamp; no contributor ID or name. | Maintain anonymous aggregate progress until the bot is removed from that server. |
| Policy acceptance | Discord user ID, Terms version, Privacy Policy version, acceptance timestamp and optional guild ID. | Confirm that the user accepted the current documents before ordinary commands run. |
| Operational diagnostics | Event type, command name, error category, timing, pseudonymous correlation ID. | Diagnose outages, bugs, rate limits, and performance without recording message contents. |
| Security records | Limited identifiers and event metadata associated with an incident. | Protect the bot and investigate abuse. |

Moonbyte does not require a separate game-service account, email address or real name.

The persisted profile fields are the Discord user ID, display name, optional bio, visibility, statistics preference and quiet-mode preference. Profile visibility defaults to private. Session snapshots contain the session ID, game ID, player Discord IDs, optional guild ID, phase, serialised game state, action count, short result text and timestamps. Completed games create a `game_results` row and update `player_game_stats` for each participant in the same PostgreSQL transaction as the final session snapshot. Result rows are linked to their parent session and expire with it after 24 hours of inactivity; aggregate statistics remain until deletion. A player's first completion of each game type also inserts a unique `first_game_completion` ledger entry for 10 Moonbits and updates the wallet in that same transaction. Achievement checks run after aggregate-stat updates in the same transaction; the five current milestone records are unique per player and remain until deletion. The Atlas stores one current location and per-player discovery rows; the five-location route catalogue is static code, while current position, first-discovery time, last-visit time and visit counts are stored in PostgreSQL. Travel only follows a direct route. The first discovery of each authored location unlocks one private Chronicle fragment and one account-bound postcard; revisits do not duplicate either record. Atlas state is personal rather than guild-scoped. Raw Discord message content is not saved as an analytics transcript. Sessions recover after restart. Statistics visibility and quiet-mode flags are saved preferences; the private `.stats` command is available, while public profile/leaderboard and optional notification surfaces are not implemented. Fixed-reward daily objectives, cumulative Wayfarer activity and private opt-in rivalry records are implemented. Lantern Road v1 stores versioned run state, guild scope, host, state, checkpoint position, participants and checkpoint history. Active participants' Atlas movement and Chronicle unlocks are committed atomically with each checkpoint. The five Atlas postcards, two fixed discovery materials, one crafted lantern and the one-time Mosslight Seal are account-bound and retain their acquisition provenance. Mosslight Resonance stores only the player's blueprint/version, run state and correct chamber checkpoint IDs/timestamps; the fixed clues and accepted answers live in reviewed source code. Submitted answers are validated in memory. Neither incorrect guesses nor answer text are persisted. Completing the Vault adds a private Chronicle page and a Wayfarer activity event. Additional materials/recipes, spending, transfers, authored story-quest networks and trading are not implemented.

## 3. Message and interaction content

Discord sends interaction data to the bot so it can process commands and game actions.

- Prefix commands require Discord's Message Content privileged intent. When enabled, the bot parses prefix and arguments in memory.
- Slash commands deliver structured interaction payloads.
- A game input such as a guess, quiz answer, or move may be stored as part of active game state when necessary to validate or resume that game. Mosslight Resonance is deliberately different: accepted answers are compared in memory, and only correct room IDs and timestamps are saved.
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
| Terms and Privacy acceptance | Keep the latest accepted versions until player deletion; replace the record when the user accepts a newer document version. |
| Moonbits wallet and ledger | Until profile deletion; session expiry does not remove earned ledger entries. |
| Achievement milestones | Until account deletion. |
| Personal Atlas location and discovery history | Until account deletion. The authored five-location catalogue is static code, not player data. |
| Wayfarer activity and companion milestones | Until account deletion; activity days are cumulative and are not reset by inactivity. |
| Daily quest rotation and progress | Until account deletion. A completed quest remains claimable after its board date; the private rotation seed is not exported. |
| Rivalry invitations and match records | Until account deletion by either participant; only post-acceptance matches are recorded. |
| Account-bound Atlas postcards and source provenance | Until account deletion; existing discoveries are backfilled and new postcards are awarded once per location. |
| Mosslight Resonance Vault runs and solved chamber checkpoints | Until account deletion. Active, paused and left runs do not expire; a player may resume from the last committed save stone. Incorrect guesses are not stored. |
| Additional authored story quests, additional Vault blueprints, and future crafting materials | Not currently stored. Future retention must be documented with the feature. |
| Guild settings | While the bot remains installed and settings are needed; delete or anonymise after removal, target maximum 30 days. |
| Routine operational logs | Target maximum 30 days, then delete or aggregate without direct identifiers. |
| Security incident records | Only as long as reasonably necessary; restrict access and document the purpose. |
| Backups | Rotate on a documented schedule; deleted profile data should expire from backups within a target maximum of 30 days. |

These are implementation targets. Before launch, maintainers must configure and test cleanup jobs and backup rotation. Data must not be kept longer merely because storage is cheap.

## 7. Privacy command and user controls

The current prefix and slash commands provide:

| Command | Purpose |
| --- | --- |
| `.terms` | Receive the current Terms of Service document. |
| `.privacy` | Receive the full current Privacy Policy document. |
| `.consent` | Check whether the current document versions have been accepted. |
| `.consent accept` | Explicitly accept the current Terms of Service and Privacy Policy versions. |
| `.privacy_data` | Privately inspect profile, session, result, statistic, achievement, wallet/ledger, Atlas, inventory, Wayfarer, quest, rivalry, Expedition and Puzzle Vault record counts. |
| `.privacy_export` | Receive a private version 12 JSON copy of profile, policy acceptance, sessions, results, statistics, achievements, wallet/ledger, Atlas, Chronicle, account-bound inventory provenance, Wayfarer, daily quest history/fingerprints, rivalry records, the requester's own Expedition participation/checkpoint history, Puzzle Vault runs/solved chamber checkpoints, and opt-in Guild Project memberships, and opt-in Guild Project memberships. Other participants' Discord IDs, private quest seeds and unsaved Vault guesses are excluded. |
| `.privacy_delete` | Read the deletion scope and confirmation instructions. |
| `.privacy_delete DELETE` | Confirm deletion of the profile, statistics, achievements, wallet/ledger, Atlas, Chronicle, account-bound inventory/provenance, Wayfarer, daily quest history, rivalries, Expedition participation, Puzzle Vault runs/checkpoints and all session snapshots containing the requester. Host ownership is transferred where possible; a run is cancelled if no active participant remains and removed when no participant history remains. |

When invoked in a server, profile details and personal title panels use ephemeral slash responses or DM delivery for prefix commands. JSON exports and deletion receipts are sent by DM. These outputs are never posted as public channel content. The Terms and Privacy documents are delivered as private attachments for slash commands. Prefix invocations can show the documents in the channel where the command was run. The export describes the current schema only and excludes other participants' Discord IDs, private quest seeds, and unsaved Vault guesses or answer text. It is assembled inside one read-only PostgreSQL `REPEATABLE READ` transaction, so profile, progress, wallet balances and ledger entries come from one consistent snapshot even when activity commits during export. Account-bound Atlas keepsakes are included with their acquisition provenance. If DM delivery fails, the command returns an error rather than exposing private data. The Privacy Policy summary and attachment may be shown in the invoking channel for prefix invocations; slash-command policy attachments are ephemeral.

### Consent gate

Before any ordinary gameplay, profile, progression, or configuration command runs, the bot checks whether the user accepted the current Terms and Privacy Policy versions. A user who has not accepted is shown instructions for the full-document commands and the explicit acceptance command. The `.terms`, `.privacy`, `.consent`, `.privacy_data`, `.privacy_export` and `.privacy_delete` commands remain available without acceptance so users can review the documents and use privacy controls. Acceptance is keyed to the Discord user ID, stores both document versions and the acceptance timestamp, and is updated when either document version changes. Acceptance is not an analytics or advertising signal.

### Deletion flow

1. Explain that the profile, aggregate statistics, achievements, Moonbits wallet/ledger, Atlas records, Wayfarer activity, quest rotations/progress, rivalry records, Puzzle Vault runs/save stones and every session snapshot containing the requester's Discord ID will be removed.
2. Require the requester to run the explicit second command `.privacy_delete DELETE`.
3. Use the authenticated Discord command author as the account owner; arbitrary target IDs are not accepted.
4. Establish the private receipt channel before changing records when deletion is invoked in a server.
5. Lock all matching session rows and the player's current Atlas world-state row; delete linked `game_results`, session rows, `player_game_stats`, `player_achievements`, `wallet_ledger_entries`, `player_wallets`, `player_world_state`, `player_atlas_discoveries`, `player_archive_fragments`, `player_vault_checkpoints`, `player_vault_runs`, `player_daily_quests`, `player_quest_rotations`, `player_wayfarer_days` and any `player_rivalries` involving the requester, then delete `user_consents` and `player_profiles` in one PostgreSQL transaction.
6. Commit before reporting the actual deleted row counts.

Profile creation re-checks both current policy versions while holding the per-player advisory lock also used by deletion. Game-session creation, joining and completion serialize against deletion; activity handlers require an existing profile and cannot recreate one. A command that passed an earlier consent preflight cannot restore a deleted profile or consent row.

The current player-owned schema is `player_profiles`, `user_consents`, `game_sessions`, `game_results`, `player_game_stats`, `player_achievements`, `player_wallets`, `wallet_ledger_entries`, `player_world_state`, `player_atlas_discoveries`, `player_archive_fragments`, `player_vault_runs`, `player_vault_checkpoints`, `player_wayfarer_days`, `player_quest_rotations`, `player_daily_quests`, `player_rivalries`, `expedition_runs`, `expedition_participants` and `expedition_checkpoint_history`. Because session state is one shared JSONB snapshot, every stored session containing the requester is removed, which can end a session for co-participants. Result records are deleted with their session; aggregate statistics, achievements, wallet, Atlas, Chronicle, Vault runs/checkpoints, Wayfarer, quest and rivalry records are removed directly. Atlas rows also have foreign-key ownership by the player profile, so deleting the profile cannot leave orphaned discovery data. Wayfarer tables are already included in the same deletion transaction and export contract. Future player-owned tables must be added before those features ship. The user may still appear in Discord messages, copies held by other server members, or infrastructure backups. Moonbyte cannot delete Discord's own records.

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


## 11. Wayfarer and rivalry records

The current Wayfarer release stores one activity-day record per player and PostgreSQL calendar date. A completed game or first-time Atlas discovery can create that day's record. Repeat events on the same day do not add extra days. These records support cumulative companion milestones; there is no attendance penalty, countdown, or streak reset.

Opening the daily quest board creates a stable set of three optional objectives from a fixed catalogue. PostgreSQL generates a per-player daily UUID seed; SHA-512 derives the selection order and a stored fingerprint. The seed is not included in player exports. Quest objectives, progress, accepted/completed/claimed state, completion timestamps and fixed Moonbits reward amounts are stored in PostgreSQL. Rewards are claimed once through the wallet ledger. This is a deterministic task rotation, not a random reward mechanic.

Rivalry records store the two participant IDs, invitation sender, mutual-acceptance state, match totals, wins and draws. A rivalry is counted only after the recipient accepts. Only completed two-player sessions between those participants after activation update the record. The Rivalry Hall is private to each participant; this release does not expose public rivalry leaderboards.

The private JSON export includes Wayfarer activity dates, daily quest history, SHA-512 rotation fingerprints, and rivalry invitations/match records. It excludes the private quest seed. Confirmed account deletion removes those rows within the same PostgreSQL transaction as the other player records. Deleting either participant also removes the associated rivalry record.

## 12. Lantern Road Expedition records

Expedition state is held in three PostgreSQL tables: expedition_runs, expedition_participants and expedition_checkpoint_history. Export format v10 includes only the requester's membership and shared checkpoint history, with a completed_by_you boolean rather than other participants' identities. The run does not expire automatically. A player deletion removes that player's participation, transfers the host to the earliest remaining active participant where possible, cancels an active run if nobody remains active, and removes the run when no participant records remain. The transaction locks affected runs before locking the player's Atlas state, matching the checkpoint transaction's lock order.

## 13. Mosslight Resonance Puzzle Vault records

The Vault uses `player_vault_runs` and `player_vault_checkpoints`, owned by the Discord player ID through the profile foreign key. A run records the fixed blueprint ID and version, state (`active`, `paused`, `left` or `completed`), current chamber index, row version and created/updated/completed timestamps. Each correct save stone records the canonical room ID, room index and solve timestamp. The run and checkpoint rows have no inactivity expiry. Leaving is non-destructive; entering returns the same open run, and `resume` reactivates a paused or left run. A completed run remains in history and a practice replay creates a separate run. The first implementation is fixed at blueprint content version 1 and exposes no public ranking, party mode, random ordering or timed penalty.

Accepted puzzle answers are authored constants and are never stored as player submissions. Wrong guesses are checked in memory and produce a clue-specific hint only. The completion reward is a single `keepsake.mosslight-seal`; its inventory provenance references the completing run and blueprint version, while a per-player one-time guard prevents additional rewards from replays. The keepsake is account-bound and has no sale, trade, currency or spending path. The final save stone, keepsake, Chronicle page and Wayfarer activity write share one PostgreSQL transaction. The player JSON export includes every run and solved checkpoint; deletion removes `player_vault_runs` and `player_vault_checkpoints` in the same transaction as the player's other records.

## 14. Policy changes and contact


A change to data categories, purposes, retention, or external providers requires code review and an update to this policy. Significant changes should be announced in release notes before they take effect where practical.

For privacy questions that cannot be completed through the bot, use the repository's published support contact once one is configured. Do not post private personal data in public issues.

## 15. Production launch checklist

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
- Profile export and deletion cover every currently persisted player table, including Atlas location/discovery, inventory provenance, Wayfarer activity, quest history, rivalry records and Puzzle Vault runs/checkpoints.
