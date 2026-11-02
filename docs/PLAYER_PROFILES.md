# Player Profiles and Privacy Controls

- Status: Player profiles, preferences, completed-game results, aggregate per-game statistics, data inventory, private export and confirmed deletion are implemented.
- Storage: PostgreSQL through SQLx.
- Owner identity: Discord user ID. Moonbyte does not collect email addresses, real names, phone numbers or precise location for this feature.

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

## Commands

| Command | Behavior |
| --- | --- |
| `.profile` | Create the default profile if needed and show it privately with the player's aggregate statistics. |
| `.profile_edit Name Here \| Short bio` | Update display name and bio. Use `-` after the separator to clear the bio. |
| `.settings private false true` | Example: save visibility, statistics preference and quiet mode. Accepted visibility values are `private` and `public`. |
| `.stats` | Privately show completed game totals, wins and draws by game. |
| `.privacy_data` | Privately inspect the profile and session record counts. |
| `.privacy_export` | Receive a JSON attachment containing the profile and stored session metadata. |
| `.privacy_delete` | Start a deletion request and explain its scope. |
| `.privacy_delete DELETE` | Explicitly confirm deletion. |

The same commands are registered as slash commands. When invoked in a guild, personal profile details, inventories and exports are sent to the requester's DM. If Discord prevents delivery, the command fails without posting personal details in the channel.

## Export format

The export is a versioned JSON document containing:

- Export format version and generation timestamp.
- Player profile and saved preferences, when a profile exists.
- Session IDs, game IDs, phase, action counts and session timestamps for snapshots containing the requester.
- A scope note explaining that the document represents the current Moonbyte PostgreSQL schema only.

The export deliberately excludes other participants' Discord IDs and raw message content. Discord independently retains interactions under its own policies.

## Deletion behavior

Deletion runs in one PostgreSQL transaction:

1. Delete every `game_sessions` row whose JSONB player list contains the requesting Discord ID.
2. Delete the matching `player_profiles` row.
3. Commit both changes together and report the affected row counts.

Game state is stored as one shared snapshot. Removing only one participant ID from that snapshot would require a domain migration and could leave a broken or partially recoverable session. The current safe behavior therefore ends every stored session containing the requester, including sessions shared with other players. The command explains this before asking for the explicit `DELETE` confirmation.

The result covers the current player-owned schema: `player_profiles`, `game_sessions`, `game_results` and `player_game_stats`. Result rows are deleted with their parent sessions; aggregate statistics are deleted directly. When new player-owned tables are introduced, their export and deletion behavior must be added in the same change before the feature is considered complete. Deletion does not remove Discord messages, copies held by other players, Discord account data, or independent infrastructure backups.

## Engineering invariants

- Default profile visibility is private.
- User-controlled text is bounded and escaped before being displayed.
- Private account data is delivered by DM from guild invocations.
- Export data is generated from the current database state, not a cached profile.
- Deletion of player and session records is atomic.
- No profile feature introduces currency, random rewards, wagering or paid progression.
- Completed-game results and aggregate statistics are written in the same transaction as the completed session snapshot. The session ID is the idempotency key.
- Session-level result metadata expires with its parent session; aggregate statistics remain until player deletion.
- Database tests cover profile creation, editing, preferences, result/stat persistence, export, inventory and deletion of linked records.
