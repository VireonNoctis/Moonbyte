# Player Profiles and Privacy Controls

- Status: Player profiles, preferences, game results, per-game statistics, five persistent achievement milestones, earned-only Moonbits, data inventory, private export and confirmed deletion are implemented.
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

## Commands

| Command | Behavior |
| --- | --- |
| `.profile` | Create the default profile if needed and show it privately with the player's aggregate statistics. |
| `.profile_edit Name Here \| Short bio` | Update display name and bio. Use `-` after the separator to clear the bio. |
| `.settings private false true` | Example: save visibility, statistics preference and quiet mode. Accepted visibility values are `private` and `public`. |
| `.stats` | Privately show completed game totals, wins and draws by game. |
| `.wallet` | Privately show the Moonbits balance and number of earned ledger entries. |
| `.privacy_data` | Privately inspect profile, session, result, statistics and wallet record counts. |
| `.privacy_export` | Receive a JSON attachment containing the current player-owned records. |
| `.privacy_delete` | Start a deletion request and explain its scope. |
| `.privacy_delete DELETE` | Explicitly confirm deletion. |

The same commands are registered as slash commands. When invoked in a guild, personal profile details, inventory counts, wallet data and exports are sent to the requester's DM. If Discord prevents delivery, private commands fail without posting personal details in the channel.

## Export format

The version 3 JSON export contains:

- Export format version and generation timestamp.
- Player profile and saved preferences, when a profile exists.
- Session IDs, game IDs, phase, action counts and session timestamps for snapshots containing the requester.
- Completed-result metadata for the requester: game, outcome, action count and completion time.
- Aggregate per-game completed, win and draw counts.
- Current Moonbits wallet balance.
- Wallet ledger entries, including amount, fixed reason, source session, idempotency key and timestamp.

The export deliberately excludes other participants' Discord IDs and raw message content. It describes the current Moonbyte PostgreSQL schema only. Discord independently retains interactions under its own policies.

## Deletion behavior

Deletion runs in one PostgreSQL transaction:

1. Lock every `game_sessions` row whose JSONB player list contains the requesting Discord ID.
2. Delete the linked `game_results` rows and the matching session rows.
3. Delete every `player_game_stats` row for the requester.
4. Delete the requester's `wallet_ledger_entries` rows.
5. Delete the matching `player_wallets` row.
6. Delete the matching `player_profiles` row.
7. Commit all changes together and report the affected row counts.

Game state is stored as one shared snapshot. Removing only one participant ID from that snapshot would require a domain migration and could leave a broken or partially recoverable session. The current safe behavior therefore ends every stored session containing the requester, including sessions shared with other players. The command explains this before asking for the explicit `DELETE` confirmation.

The operation covers the current player-owned schema: `player_profiles`, `game_sessions`, `game_results`, `player_game_stats`, `player_achievements`, `player_wallets` and `wallet_ledger_entries`. Result rows are removed with their parent sessions; aggregate statistics and wallet records are removed directly. Deletion does not remove Discord messages, copies held by other players, Discord account data, or independent infrastructure backups.

## Engineering invariants

- Default profile visibility is private.
- User-controlled text is bounded and escaped before being displayed.
- Private account data is delivered by DM from guild invocations.
- Export is scoped to the authenticated player and the current schema.
- Deletion of player and session records is atomic.
- Achievement unlocks are checked against server-owned aggregate statistics and stored once per player/achievement.\n- The first-completion milestone is fixed, disclosed and non-random.
- Ledger writes and wallet balance updates share the game-completion transaction.
- A unique player/game idempotency key prevents repeat completion credits.
- No purchases, cash-out, wagering, staking, or player transfers exist.
- Future player-owned tables must extend the inventory, export, deletion and retention contract in the same implementation change.
- PostgreSQL tests cover profile creation/editing, preferences, result/stat persistence, first-completion currency, duplicate-completion protection, export, inventory and linked-record deletion.
