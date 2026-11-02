# Moonbyte Command Reference

- Status: Four playable games, PostgreSQL sessions, player profiles, game results/statistics, earned-only Moonbits and the initial Tavern Wayfinder are implemented.
- Framework: Poise on Serenity.
- Default prefix: period; override with DEFAULT_PREFIX.

## Available commands

| Prefix | Slash | Behavior |
| --- | --- | --- |
| .help | /help | Lists commands available in this build and the configured prefix. |
| .about | /about | Shows the application version and implementation boundaries. |
| .ping | /ping | Confirms handler responsiveness and reports process uptime. |
| .tavern | /tavern | Opens the Tavern home and Wayfinder for implemented destinations. |
| .privacy | /privacy | Explains current data handling and privacy limits. |
| .privacy_data | /privacy_data | Privately inspects the requester's current data inventory. |
| .privacy_export | /privacy_export | Sends a private JSON export attachment. |
| .privacy_delete | /privacy_delete | Starts deletion confirmation. |
| .privacy_delete DELETE | /privacy_delete | Confirms deletion of the requester's profile, statistics, wallet/ledger and linked sessions/results. |
| .profile | /profile | Creates or displays the requester's private player profile. |
| .profile_edit | /profile_edit | Updates the profile display name and bio. |
| .settings | /settings | Saves profile visibility and player preference flags. |
| .stats | /stats | Privately displays completed-game totals and per-game wins/draws. |
| .wallet | /wallet | Privately displays earned-only Moonbits balance and ledger-entry count. |
| .achievements | /achievements | Privately lists the player's unlocked achievement milestones. |
| .atlas | /atlas | Privately displays the player's current place, discoveries and connected routes. |
| .travel <location> | /travel | Travels to a directly connected location and records the visit.
| .games | /games | Lists four playable games, player counts and controls. |
| .play <game> | /play | Starts a solo game or creates a two-player lobby. |
| .join <session> | /join | Joins an open two-player lobby in its originating server. |
| .action <session> <input> | /action | Submits a game-specific action after participant, guild, phase and turn validation. |
| .session <session> | /session | Reads the current snapshot of a session in which the requester participates. |
| .sessions | /sessions | Lists up to 10 most recently updated, non-expired sessions the requester participates in, scoped to the current guild or DMs. |
| .leave <session> | /leave | Ends a session the requester participates in. |

All commands use shared Rust handlers for prefix and slash invocation. The help output lists only commands registered in this build; it does not advertise planned economy, quest or RPG features as working.

## Profile commands

- `.profile_edit Display Name | Short bio` sets the display name and bio. The name is limited to 32 characters and the bio to 160 characters. Use `-` after the separator to clear the bio.
- `.settings private false true` is an example that sets private visibility, hides statistics, and enables the quiet-mode preference. Visibility must be `public` or `private`.
- Visibility defaults to private. This build does not yet expose a public profile directory or statistics/leaderboard surface; the preferences are persisted for those later surfaces.
- Profile, settings, statistics and account-data responses are sent by DM when invoked in a server. The public `.privacy` overview and Tavern home may be shown in the invoking channel. In DMs, private responses are returned in the current conversation.

## Privacy controls and scope

1. Run `.privacy_data` to inspect profile, session, result, statistic, achievement, wallet/ledger and Atlas state/discovery records associated with the requester.
2. Run `.privacy_export` to receive `moonbyte-data-export.json` by DM. The version 5 export contains the player's profile, session metadata, completed-result metadata, aggregate statistics, achievement records, wallet balance/ledger and Atlas current-location/discovery history, but not other participants' IDs or raw message content.
3. Run `.privacy_delete` to read the exact deletion scope. Nothing is deleted at this step.
4. Run `.privacy_delete DELETE` to confirm. This transaction removes the player's profile, statistic and achievement rows, Moonbits wallet/ledger, Atlas current-location/discovery records, plus every stored game-session snapshot and linked result containing that Discord ID.

Game-session state is stored as one shared snapshot. Deleting a session row can therefore end a session for co-participants. The command explains this before confirmation. The private deletion receipt reports the counts of session, result, statistic, achievement, wallet/ledger and Atlas records removed.

This covers the current Moonbyte PostgreSQL player-owned schema only: `player_profiles`, `game_sessions`, `game_results`, `player_game_stats`, `player_achievements`, `player_wallets`, `wallet_ledger_entries`, `player_world_state` and `player_atlas_discoveries`. It cannot delete Discord messages, Discord account data, copies held by other participants, or backups managed outside this database. The command never claims otherwise.

## Moonbits

Moonbits are an earned-only milestone, not a payment feature. The first completed session of each game type earns 10 Moonbits per participant, once for that player/game type. The reward is not tied to winning and repeat completions do not grant additional credits. The credit and immutable ledger entry are committed in the same PostgreSQL transaction as the completed game record. Moonbits cannot be purchased, cashed out, staked or transferred outside Moonbyte. Spending and player-to-player trading are not enabled. Use `.wallet` to inspect the balance and ledger-entry count privately.

## Game controls

| Game | Start argument | Action input |
| --- | --- | --- |
| Tic-tac-toe | tic-tac-toe | 1-based row,column, for example 2,3 |
| Connect Four | connect-four | 1-based column from 1 to 7 |
| Memory Match | memory-match | 1-based position from 1 to 8 |
| Word Scramble | word-scramble | Answer text; the current starter puzzle is BITOR → ORBIT |

Tic-tac-toe and Connect Four create lobbies for two players. A second participant must join from the same Discord server. Solo games start immediately. Session reads and session listings are restricted to participants. `.sessions` only lists sessions from the current guild, or solo sessions in DMs; it does not search other servers.

## Discord setup

1. Set DISCORD_TOKEN in the process environment. Never commit the token.
2. Invite the application with bot and applications.commands OAuth scopes.
3. Enable the Message Content privileged intent in Discord developer settings if prefix commands are required.
4. Set DEFAULT_PREFIX if the period prefix is not desired.
5. Start the process with cargo run --locked.

The runtime requests only GUILDS, GUILD_MESSAGES and MESSAGE_CONTENT. The Message Content intent is privileged and must be enabled for the application. Slash commands are registered globally during startup; Discord may take up to an hour to propagate global command changes.

## Persistence and privacy boundary

Player profile data is stored in `player_profiles`; game sessions are stored in `game_sessions`; completed results are stored in `game_results`; durable per-game aggregates are stored in `player_game_stats`; and the earned-only currency foundation is stored in `player_wallets` plus `wallet_ledger_entries`. Versioned migrations 0002 through 0006 add those tables. A profile stores the Discord user ID, display name, optional bio, private/public visibility and two preference flags. It does not store real names, email addresses or location.

Session snapshots contain game state and participant IDs, optional guild scope, lifecycle phase and action count. On completion, result metadata and each participant's per-game counters are written in the same transaction as the final snapshot. The session ID prevents duplicate result awards. Each participant's first completion of a game type also earns a fixed 10 Moonbits once; the ledger idempotency key is unique per player/game type and wallet balance updates commit in the same transaction. Repeat completions do not pay again. Session-level result rows are removed with their parent session after the 24-hour inactivity window; aggregate statistics and wallet milestones remain until account deletion.

See [Player Profiles](PLAYER_PROFILES.md) and [Privacy Policy](PRIVACY.md) for exact data coverage, export fields, deletion behavior and the limitations that remain.

## Still planned

Inventory, quests, Expeditions, authored story, parties, public leaderboards, personal Vaults, crafting, Bazaar trading, a live authenticated game API, Discord Activity integration and server administration remain planned. Add each only with complete validation, authorization, persistence, retention, recovery, tests and documentation.
