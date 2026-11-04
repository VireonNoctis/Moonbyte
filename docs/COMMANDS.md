# Moonbyte Command Reference

- Status: Four playable games, PostgreSQL sessions, player profiles, game results/statistics, achievements, earned-only Moonbits, the five-location Atlas, private Chronicle pages and the Wayfarer progression slice and Lantern Road Expedition v1 are implemented.
- Framework: Poise on Serenity.
- Default prefix: period; override with DEFAULT_PREFIX. Servers can set a guild-specific prefix with the Manage Server-only prefix command. First-time users must review and accept both current policy versions before ordinary commands are enabled.

## Available commands

| Prefix | Slash | Behavior |
| --- | --- | --- |
| .help | /help | Lists commands available in this build and the configured prefix. |
| .about | /about | Shows the application version and implementation boundaries. |
| .ping | /ping | Confirms handler responsiveness and reports process uptime. |
| .terms | /terms | Receives the current Terms of Service document. |
| .consent | /consent | Checks acceptance status; use accept after reviewing both policy documents. |
| .prefix | /prefix | Manage Server only; reads or updates the guild prefix and optional command channel. |
| .tavern | /tavern | Opens the Tavern home and Wayfinder for implemented destinations. |
| .privacy | /privacy | Receives the full Privacy Policy document and summary. |
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
| .quests | /quests | Opens today's optional daily quest board. |
| .quest accept <id> / .quest claim <id> | /quest | Opts into an objective or claims its fixed, completed reward. |
| .wayfarer | /wayfarer | Shows cumulative activity days and cosmetic companion stage. |
| .companion | /companion | Opens the companion journal. |
| .rivals | /rivals | Lists private rivalry invitations and active match records. |
| .rival declare @player / accept <id> / decline <id> | /rival | Manages bilateral, opt-in rivalries. |
| .atlas | /atlas | Privately displays the player's current place, discoveries and connected routes. |
| .travel <location> | /travel | Travels to a directly connected location and records the visit. |
| .chronicle | /chronicle | Privately reads lore pages unlocked by first-time Atlas discoveries. |
| .expedition | /expedition | Lists the Lantern Road, starts solo/group runs, joins, begins, advances, pauses, resumes, leaves or cancels a run. |
| .games | /games | Lists four playable games, player counts and controls. |
| .play <game> | /play | Starts a solo game or creates a two-player lobby. |
| .join <session> | /join | Joins an open two-player lobby in its originating server. |
| .action <session> <input> | /action | Submits a game-specific action after participant, guild, phase and turn validation. |
| .session <session> | /session | Reads the current snapshot of a session in which the requester participates. |
| .sessions | /sessions | Lists up to 10 most recently updated, non-expired sessions the requester participates in, scoped to the current guild or DMs. |
| .leave <session> | /leave | Ends a session the requester participates in. |

Before normal commands run, new users must review the Terms of Service with .terms and the full Privacy Policy with .privacy, then explicitly agree using .consent accept. A material update to either document requires renewed acceptance. Legal and privacy commands remain available while blocked.

Server moderators with Manage Server can use .prefix to view settings, .prefix set ! #bot-commands to set a prefix and command channel, .prefix channel #bot-commands to change only the channel, .prefix channel off to allow every channel, or .prefix reset to restore the process default. The equivalent slash command accepts the same instruction in its changes field. The configuration command remains available from any channel so a server can recover from an incorrect channel setting. When a command channel is set, every other prefix and slash command is restricted to it. DMs continue to use the process default prefix.

Poise prefix aliases are available only for the listed short forms: .t for .tavern, .g for .games, .p for .play, .j for .join, .l for .leave, .a for .atlas, .tr for .travel, .me for .profile, .w for .wallet, .st for .stats, .ach for .achievements, .q for .quests and .wf for .wayfarer. They do not create slash-command names. Help, Terms, consent and all privacy commands deliberately have no shortcuts.

All commands use shared Rust handlers for prefix and slash invocation. The help output lists only commands registered in this build; it does not advertise proposed RPG, inventory or market features as working.

## Lantern Road Expedition v1

The first route is Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory. Use .expedition or .expedition list to inspect it. Start a solo run with .expedition start lantern-road solo, or create a group run with .expedition start lantern-road group inside a Discord server. A group start announces the run ID, not anyone's private Atlas progress. Every additional player must explicitly join with .expedition join RUN_ID; shared runs support up to four players from the same server. All participants must be at Lantern Square before the host can begin with .expedition begin RUN_ID.

Use .expedition status RUN_ID to inspect a run you joined. Advance with .expedition advance RUN_ID CHECKPOINT_ID CHOICE. The route choices are listen or inspect at Mosslight Grove, star at the Ember Archive, and chart at Starfall Observatory. The incorrect lantern clue does not move the run. The expected next checkpoint is required, so stale or duplicate advances are rejected.

Any active participant can pause or resume the run. A participant can leave independently; if the host leaves, ownership transfers to the earliest remaining active participant. A run with no active participants is cancelled. The host can cancel a non-terminal run. Runs do not expire, have no attendance deadline and do not remove progress for a break. Each successful checkpoint updates only the personal Atlas and Chronicle records of the active participants in the same PostgreSQL transaction as run history. Completion records a Wayfarer activity day; it grants no random reward or additional Moonbits.

## Profile commands

- `.profile_edit Display Name | Short bio` sets the display name and bio. The name is limited to 32 characters and the bio to 160 characters. Use `-` after the separator to clear the bio.
- `.settings private false true` is an example that sets private visibility, hides statistics, and enables the quiet-mode preference. Visibility must be `public` or `private`.
- Visibility defaults to private. This build does not yet expose a public profile directory or statistics/leaderboard surface; the preferences are persisted for those later surfaces.
- Profile, settings, statistics and account-data responses are sent by DM when invoked in a server. The public `.privacy` overview and Tavern home may be shown in the invoking channel. In DMs, private responses are returned in the current conversation.

## Privacy controls and scope

1. Run `.privacy_data` to inspect profile, session, result, statistic, achievement, wallet/ledger, Atlas, Chronicle, Wayfarer, rivalry and Expedition records associated with the requester.
2. Run `.privacy_export` to receive `moonbyte-data-export.json` by DM. Version 9 includes the requester's own Expedition membership details and shared checkpoint history, plus a `completed_by_you` flag. It does not expose other participants' Discord IDs or private quest seeds.
3. Run `.privacy_delete` to read the exact deletion scope. Nothing is deleted at this step.
4. Run `.privacy_delete DELETE` to confirm. This transaction removes the player's profile, game/player records and Expedition participation. It transfers host ownership to the earliest remaining active participant where possible, cancels a run when no active participants remain, and removes a run when no participant history remains. It also removes every stored game-session snapshot and linked result containing that Discord ID.

Game-session state is stored as one shared snapshot. Deleting a session row can therefore end a session for co-participants. The command explains this before confirmation. The private deletion receipt reports the counts of session, result, statistic, achievement, wallet/ledger, Atlas, Wayfarer, quest and rivalry records removed, plus Expedition participations removed, hosts transferred, runs cancelled and empty runs removed.

This covers the current Moonbyte PostgreSQL player-owned schema only: `player_profiles`, `user_consents`, `game_sessions`, `game_results`, `player_game_stats`, `player_achievements`, `player_wallets`, `wallet_ledger_entries`, `player_world_state`, `player_atlas_discoveries`, `player_wayfarer_days`, `player_quest_rotations`, `player_daily_quests`, `player_rivalries`, `expedition_runs`, `expedition_participants` and `expedition_checkpoint_history`. It cannot delete Discord messages, Discord account data, copies held by other participants, or backups managed outside this database. The command never claims otherwise.

## The Wayfarer

The daily board selects three objectives from a fixed catalogue for each player and PostgreSQL calendar date. Opening `.quests` generates or reads the saved board; it does not automatically accept objectives. Use `.quest accept <quest-id>` to opt in. Completed games and first-time Atlas discoveries update active progress in the same transaction as their source event. A claim uses `.quest claim <quest-id>` and credits a fixed reward to the wallet ledger once. Replaying actions or repeating a claim cannot duplicate credit.

Rotation uses a PostgreSQL-generated UUID seed, domain-separated SHA-512 selection and unbiased rejection sampling. The short SHA-512 fingerprint is a rotation label, not proof of a fair game outcome. Private seeds are not shown or included in exports. Missing a day has no penalty. `.wayfarer` shows cumulative activity days; `.companion` shows Mallow's cosmetic milestones. Activity days never decrease and companions do not change game rules or stats.

`.rival declare @player` creates a pending invitation for an existing Moonbyte player. The recipient must run `.rival accept <id>` to activate it, or `.rival decline <id>` to decline. Mutual declarations also activate a rivalry because both players have independently opted in. Only later completed two-player games are counted. `.rivals` shows a private record; there are no public rivalry leaderboards, wagers or stakes.

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

Player profile data is stored in `player_profiles`; consent in `user_consents`; game sessions/results/statistics/achievements and wallet records use their corresponding player tables; Atlas state uses `player_world_state` and `player_atlas_discoveries`; Wayfarer uses `player_wayfarer_days`, `player_quest_rotations`, `player_daily_quests` and `player_rivalries`. Versioned migrations 0002 through 0009 add these tables. A profile stores the Discord user ID, display name, optional bio, private/public visibility and two preference flags. It does not store real names, email addresses or location.

Session snapshots contain game state and participant IDs, optional guild scope, lifecycle phase and action count. On completion, result metadata and each participant's per-game counters are written in the same transaction as the final snapshot. The session ID prevents duplicate result awards. Each participant's first completion of a game type also earns a fixed 10 Moonbits once; the ledger idempotency key is unique per player/game type and wallet balance updates commit in the same transaction. Repeat completions do not pay again. Session-level result rows are removed with their parent session after the 24-hour inactivity window; aggregate statistics, achievements, wallet milestones, Atlas discoveries, Wayfarer activity and quest/rivalry records remain until account deletion.

See [Player Profiles](PLAYER_PROFILES.md) and [Privacy Policy](PRIVACY.md) for exact data coverage, export fields, deletion behavior and the limitations that remain.

## Still planned

Additional Expedition routes, authored story routes, parties, public leaderboards, inventory, personal Vaults, crafting, Bazaar trading and server administration remain planned. The optional daily Wayfarer quest board is implemented separately from authored story quests. Lantern Road v1 is registered and playable; see [Expedition Foundation v1](EXPEDITION_FOUNDATION.md).
