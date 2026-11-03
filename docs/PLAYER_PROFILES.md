# Player Profiles and Privacy Controls

- Status: Player profiles/preferences, versioned Terms/Privacy acceptance, game results/statistics, five persistent achievements, earned-only Moonbits, the five-location Atlas, Wayfarer activity, daily quests, cosmetic companion milestones, opt-in rivalries, version 7 private export and confirmed deletion are implemented.
- Storage: PostgreSQL through SQLx.
- Owner identity: Discord user ID. Moonbyte does not collect email addresses, real names, phone numbers or precise location for these features.

## Profile record

The `player_profiles` table stores:

| Field | Purpose |
| --- | --- |
| `discord_user_id` | Stable Discord identity key; not shown in public profile output. |
| `display_name` | Player-selected name, 1–32 characters. |
| `bio` | Optional short description, maximum 160 characters. |
| `visibility` | `private` or `public`; defaults to private. |
| `show_statistics` | Saved preference for future statistics surfaces. |
| `quiet_mode` | Saved preference for future optional notices. |
| `created_at`, `updated_at` | Profile lifecycle timestamps. |

Profile visibility is stored now, but a public profile directory, social graph and leaderboard are not implemented. Profile commands return personal data through DM when called in a server. The statistics and quiet-mode flags are preferences for future public/social surfaces; this build does not send unsolicited player notifications. The `.stats` command and private profile view show the player's own completed-game aggregates.

## Terms and Privacy acceptance

The global command gate requires explicit acceptance of the current Terms of Service and Privacy Policy versions before ordinary gameplay, profile, progression or guild-configuration commands can run. The full documents are available through the legal commands, and privacy controls remain available before acceptance.

The independent user_consents table stores one current record per Discord user ID:

| Field | Purpose |
| --- | --- |
| discord_user_id | Owner of the agreement record. |
| terms_version | Version of the Terms of Service explicitly accepted. |
| privacy_version | Version of the Privacy Policy explicitly accepted. |
| accepted_at | Timestamp of the explicit acceptance. |
| accepted_in_guild_id | Optional server where the user accepted; the agreement remains global to their Discord account. |

A material change to either document requires a version bump in the Rust consent module. The next ordinary command then prompts the user to review and accept the current versions. The consent record is included in the data inventory and version 7 export, and is deleted in the same account-deletion transaction as the player's other records.

## Completed-game records

- `game_results` contains one completed result per session, linked to `game_sessions` with a cascading foreign key.
- `player_game_stats` stores durable per-player/per-game totals for games played, wins and draws.
- Result metadata expires with its parent session after the existing 24-hour inactivity window.
- Aggregate statistics remain until the player requests deletion.
- Result and aggregate-stat writes occur in the same transaction as the final game-session snapshot. The session ID is the idempotency key for the result row.

## Achievements

Achievements are fixed, server-authoritative milestones evaluated after a completed game updates the player's aggregate statistics. Unlock rows are written in the same PostgreSQL transaction as the completed result, per-game statistics and Moonbits milestone. The unique key on `(discord_user_id, achievement_id)` makes each unlock idempotent.

| Achievement | Unlock condition |
| --- | --- |
| **First Steps** | Complete the first game. |
| **Bright Start** | Earn the first game victory. |
| **Game Hopper** | Complete all four games currently in the starter library. |
| **Table Regular** | Complete ten games across the library. |
| **Rising Champion** | Win five games. |

Achievements are cosmetic records, not currency. They do not grant Moonbits, competitive power, paid boosts or item drops. The private `.achievements` command and Tavern's **The Gallery** show unlocked achievements with their category and unlock time. A later catalogue expansion must version the achievement definition and update the collection milestone deliberately.

Achievement records store the achievement ID, title/description through the definition catalogue, source session ID and unlock timestamp. They remain until account deletion; expiry of the source game session does not remove a persistent achievement.

## Moonbits wallet

Moonbits are an earned-only, non-redeemable game milestone. The initial rule is deliberately fixed and narrow:

- The first completed session of each game type earns **10 Moonbits** for each participant.
- A player receives that milestone once per game type. Later completions of the same game type do not issue more credits.
- The reward is independent of winning, losing or drawing. It recognises completing the activity, not competitive outcome.
- The credit is written to `wallet_ledger_entries` and the `player_wallets` balance is updated in the same PostgreSQL transaction as the final game snapshot, result and statistics.
- The unique idempotency key is `first-game-completion:<game_id>` per player. It prevents duplicate rewards when the player completes repeat sessions.
- Wallets are not purchasable, cashable, transferable outside Moonbyte, stakeable or redeemable. Spending, player transfers, Vault keys and Bazaar purchases are not enabled.

The current schema stores:

| Table | Stored information |
| --- | --- |
| `player_wallets` | Discord user ID, non-negative balance, version and timestamps. |
| `wallet_ledger_entries` | Immutable entry ID, player ID, signed amount, fixed reason, source session ID, idempotency key and timestamp. |

The `.wallet` command displays the player's balance and ledger-entry count privately. The Tavern Wayfinder exposes it as **The Ledger**. The `.games` output discloses the first-completion reward rule.

## Atlas and personal discoveries

The first Atlas slice is a five-location authored map. It stores a player's current location plus a personal discovery row for each place they have visited.

| Location ID | Name | Region | Direct routes |
| --- | --- | --- | --- |
| `lantern-square` | Lantern Square | Hearthland | Mosslight Grove, Glasswater Pier |
| `mosslight-grove` | Mosslight Grove | Verdant Reach | Lantern Square, Ember Archive, Starfall Observatory |
| `glasswater-pier` | Glasswater Pier | Western Coast | Lantern Square, Starfall Observatory |
| `ember-archive` | The Ember Archive | Cinder Vale | Mosslight Grove, Starfall Observatory |
| `starfall-observatory` | Starfall Observatory | High Astral | Mosslight Grove, Glasswater Pier, Ember Archive |

- The route graph is static, reviewed Rust data with stable IDs and reciprocal edges.
- Opening `.atlas` creates the player's starting point at Lantern Square if one does not already exist.
- `.travel` accepts a known location ID or display name, but only a direct route from the current location is valid.
- Travel, current-location updates and visit counters are persisted in PostgreSQL.
- Reopening the Atlas is read-only after initialisation. Travelling to the current location is a no-op.
- Visiting a location does not grant Moonbits, achievements, items or repeatable rewards.
- The expanded Atlas design, Expeditions, narrative and community mapping remain proposals.

The database stores `player_world_state` and `player_atlas_discoveries`, both owned by `player_profiles` through foreign keys. Atlas locations and visit history are included in the private data inventory, version 7 export and account deletion transaction. Wayfarer progress is described below.

## Wayfarer activity, daily quests and companion milestones

The Wayfarer is a cumulative, non-punitive player journey. A completed game or first-time Atlas discovery records one activity day per player/date. Repeated games and visits on the same date do not inflate the day count. Missing days do not reset or reduce progress.

The daily board selects three optional objectives for each player/date from a fixed catalogue: complete one game (5 Moonbits), complete three games (8), win one game (8), complete two different game types (10), or discover one new Atlas location (7). The board does not accept objectives automatically. Players choose with `.quest accept <id>`. Once completed, a player claims with `.quest claim <id>`. The fixed amount is credited via `wallet_ledger_entries` under a unique player/date/quest idempotency key in the same transaction as marking the objective claimed. No random reward table, purchase, stake, cash-out or wager is involved.

PostgreSQL generates a UUID seed once per player/date. SHA-512 with a separate domain derives a deterministic permutation using rejection sampling for unbiased index selection. A fingerprint of the seed and date is visible as a short rotation reference and included in the export; the private seed is excluded. SHA-512 is a hash, not entropy or encryption.

Mallow is the first cosmetic companion. The stages are New Companion (1 day), Pathfinder (3), Trail Scout (7), Wayfinder (14), Starbound (30), Constellation Keeper (60) and Elder Star (100). Stages are cosmetic and do not grant competitive advantages, item drops or currency.

## Mutual rivalries

`.rival declare @player` creates a pending invitation to an existing Moonbyte player. Only the recipient may accept or decline. If both players independently send one another a declaration, the rivalry activates because both have explicitly opted in. Completed two-player game results are counted only after activation. Each participant receives a private view from `.rivals`; there is no public rivalry leaderboard.

## Commands

| Command | Behavior |
| --- | --- |
| `.terms` | Receive the current Terms of Service document. |
| `.consent` | Check the current agreement status; use `.consent accept` after reviewing both documents. |
| `.profile` | Create the default profile if needed and show it privately with the player's aggregate statistics. |
| `.profile_edit Name Here \| Short bio` | Update display name and bio. Use `-` after the separator to clear the bio. |
| `.settings private false true` | Example: save visibility, statistics preference and quiet mode. Accepted visibility values are `private` and `public`. |
| `.stats` | Privately show completed game totals, wins and draws by game. |
| `.wallet` | Privately show the Moonbits balance and number of earned ledger entries. |
| `.achievements` | Privately list the player's unlocked milestones. |
| `.quests` | Open the optional daily board. |
| `.quest accept <id>` / `.quest claim <id>` | Opt in to an objective or claim its fixed completed reward. |
| `.wayfarer` / `.companion` | View cumulative journey and companion milestones. |
| `.rivals` / `.rival` | View or manage mutual rivalry invitations and records. |
| `.atlas` | Privately show the current world location, known places and connected routes. |
| `.travel mosslight-grove` | Travel to a directly connected location; use the Atlas for available routes. |
| `.privacy_data` | Privately inspect profile, session, result, statistic, achievement, wallet/ledger and Atlas record counts. |
| `.privacy_export` | Receive a JSON attachment containing the current player-owned records. |
| `.privacy_delete` | Start a deletion request and explain its scope. |
| `.privacy_delete DELETE` | Explicitly confirm deletion. |

The same commands are registered as slash commands. When invoked in a guild, personal profile details, inventory counts, wallet data, achievement lists, Atlas maps and exports are sent to the requester's DM. If Discord prevents delivery, private commands fail without posting personal details in the channel.

## Export format

The version 7 JSON export contains:

- Export format version and generation timestamp.
- Player profile and saved preferences, when a profile exists.
- The current Terms of Service and Privacy Policy versions, acceptance timestamp and optional accepting guild ID, when an acceptance record exists.
- Session IDs, game IDs, phase, action counts and session timestamps for snapshots containing the requester.
- Completed-result metadata for the requester: game, outcome, action count and completion time.
- Aggregate per-game completed, win and draw counts.
- Current Moonbits wallet balance.
- Wallet ledger entries, including amount, fixed reason, source session, idempotency key and timestamp.
- Five achievement milestone records when earned, including their source session and unlock timestamp.
- The player's current Atlas location and the locations they have discovered, with timestamps and visit counts.

The export deliberately excludes other participants' Discord IDs and raw message content. It describes the current Moonbyte PostgreSQL schema only. Discord independently retains interactions under its own policies.

## Deletion behavior

Deletion runs in one PostgreSQL transaction:

1. Lock every `game_sessions` row whose JSONB player list contains the requesting Discord ID.
2. Delete the linked `game_results` rows and the matching session rows.
3. Delete every `player_game_stats` and `player_achievements` row for the requester.
4. Delete the requester's `wallet_ledger_entries` rows and `player_wallets` row.
5. Lock and delete the requester's `player_world_state` row, then delete their `player_atlas_discoveries` rows.
6. Delete the requester's `user_consents` row.
7. Delete the matching `player_profiles` row.
7. Commit all changes together and report the affected row counts.

Game state is stored as one shared snapshot. Removing only one participant ID from that snapshot would require a domain migration and could leave a broken or partially recoverable session. The current safe behavior therefore ends every stored session containing the requester, including sessions shared with other players. The command explains this before asking for the explicit `DELETE` confirmation.

The operation covers the current player-owned schema: `player_profiles`, `user_consents`, `game_sessions`, `game_results`, `player_game_stats`, `player_achievements`, `player_wallets`, `wallet_ledger_entries`, `player_world_state`, `player_atlas_discoveries`, `player_wayfarer_days`, `player_quest_rotations`, `player_daily_quests` and `player_rivalries`. Result rows are removed with their parent sessions; aggregate statistics, achievements, wallet records and Atlas records are removed directly. Atlas tables also have foreign-key ownership by the profile. Deletion does not remove Discord messages, copies held by other players, Discord account data, or independent infrastructure backups.

## Engineering invariants

- Default profile visibility is private.
- User-controlled text is bounded and escaped before being displayed.
- Private account data is delivered by DM from guild invocations.
- Export is scoped to the authenticated player and the current schema.
- Policy acceptance is versioned, user-owned data and is included in export and deletion.
- Deletion of player, session, achievement, wallet, Atlas, Wayfarer, quest and rivalry records is atomic.
- Achievement unlocks are checked against server-owned aggregate statistics and stored once per player/achievement.
- The first-completion milestone is fixed, disclosed and non-random.
- Ledger writes and wallet balance updates share the game-completion transaction.
- A unique player/game idempotency key prevents repeat completion credits.
- No purchases, cash-out, wagering, staking, or player transfers exist.
- Future player-owned tables must extend the inventory, export, deletion and retention contract in the same implementation change.
- Quest seeds are private; player exports include only rotation fingerprints and player-facing quest history.
- Rivalry match records are private to their two participants and are only collected after mutual acceptance.
- PostgreSQL tests cover profile creation/editing, preferences, result/stat persistence, first-completion currency, duplicate-completion protection, export, inventory and linked-record deletion.
