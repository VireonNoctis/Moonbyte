# Moonbyte Command Reference

- Status: The first playable game/runtime slice is implemented.
- Framework: Poise on Serenity.
- Default prefix: period; override with DEFAULT_PREFIX.

## Available commands

| Prefix | Slash | Behavior |
| --- | --- | --- |
| .help | /help | Lists commands available in this build and the configured prefix. |
| .about | /about | Shows the application version and current implementation boundaries. |
| .ping | /ping | Confirms handler responsiveness and reports process uptime. It is not a network-latency measurement. |
| .privacy | /privacy | Explains current PostgreSQL session storage and the limits of available privacy controls. |
| .privacy-data | /privacy-data | Privately counts the requester's non-expired session records across all guild and DM scopes. It does not export record contents. |
| .privacy-export | /privacy-export | Privately sends a JSON attachment containing the requester's non-expired session records across all scopes; other participants' Discord IDs are anonymised. Exports above 8 MiB are not sent or truncated. |
| .games | /games | Lists seven playable games, player counts and controls. |
| .rules <game> | /rules | Shows the selected game's implemented rules and controls before play. |
| .play <game> | /play | Starts a solo game or creates a two-player lobby, subject to the 10 active-session cap per player per guild/DM scope. |
| .join <session> | /join | Joins an open two-player lobby in its originating server, subject to the same per-player cap. |
| .action <session> <input> | /action | Submits a game-specific action after participant, guild, phase and turn validation. |
| .session <session> | /session | Privately reads the current snapshot of a session in which the requester participates. |
| .sessions | /sessions | Privately lists up to 10 most recently updated, non-expired active sessions the requester participates in, scoped to the current guild or DMs. |
| .history | /history | Privately lists up to 10 non-expired completed or ended sessions the requester participated in, scoped to the current guild or DMs. |
| .leave <session> | /leave | Ends a session the requester participates in. |

All commands use shared Rust handlers for prefix and slash invocation. Use `.rules <game>` or `/rules` to read the implemented rules and controls before starting a game.

The prefix `.action` command consumes the full remaining input. For Minesweeper, use `.action <session> reveal 1,1` or `.action <session> flag 1,1`; `/action` accepts the same text in its `input` option. The help output lists only commands registered in this build; it does not advertise planned economy, profile or RPG features as working.

Private session inspection/listing and export commands (`.session`, `.sessions`, `.history`, `.privacy-data`, `.privacy-export`) keep player-specific details out of shared guild channels. Slash invocations receive ephemeral responses. Guild-prefix invocations deliver the details by DM and post only a generic delivery notice; DM-prefix invocations reply in that already-private channel. If DMs are blocked, the bot posts a generic retry notice without exposing session details.

Gameplay commands (`.play`, `.join`, `.action`, `.leave`) post game boards or results in the invoking channel so participants can play together. In a guild, members with access to that channel can see those responses and may see the command invocation itself. Use a DM for solo play that should not appear in a shared server channel.

## Game controls

| Game | Start argument | Action input |
| --- | --- | --- |
| Tic-tac-toe | tic-tac-toe | 1-based row,column, for example 2,3 |
| Connect Four | connect-four | 1-based column from 1 to 7 |
| Memory Match | memory-match | 1-based position from 1 to 8 |
| Minesweeper | minesweeper | `reveal row,column` or `flag row,column`; 1-based coordinates from 1 to 8 |
| Word Scramble | word-scramble | Answer text; the current starter puzzle is BITOR → ORBIT |
| Sequence Recall | sequence-recall | Use `hide`, then repeat the full pattern with comma-separated values 1–4. |
| 2048 | 2048 | Slide tiles with `up`, `down`, `left`, or `right` (`w`, `s`, `a`, `d` also work). |

Tic-tac-toe and Connect Four create lobbies for two players. A second participant must join from the same Discord server. Solo games start immediately. Session reads and session listings are restricted to participants. `.sessions` and `.history` only list sessions from the current guild, or solo sessions in DMs; they do not search other servers. `.history` uses the existing 24-hour inactivity retention window and does not create a separate long-term results store.

Sequence Recall uses four authored patterns selected deterministically from the session ID; patterns are not random or fairness-certified.

## Discord setup

1. Set DISCORD_TOKEN in the process environment. Never commit the token.
2. Invite the application with bot and applications.commands OAuth scopes.
3. Enable the Message Content privileged intent in Discord developer settings if prefix commands are required.
4. Set DEFAULT_PREFIX if the period prefix is not desired.
5. Start the process with cargo run --locked.

The runtime requests only GUILDS, GUILD_MESSAGES and MESSAGE_CONTENT. The Message Content intent is privileged and must be enabled for the application. Slash commands are registered globally during startup; Discord may take up to an hour to propagate global command changes.

## Runtime and privacy boundary

Active session state is stored in PostgreSQL. The JSONB snapshot includes the game state, participant Discord IDs, optional guild ID, lifecycle phase and action count; mutations are committed transactionally with a row lock. Multiplayer lobbies require a guild context, sessions are bound to their originating guild, and session reads are limited to participants. Each player can have at most 10 active or waiting sessions per guild or DM scope; starts and joins serialize on a per-player advisory transaction lock so concurrent requests cannot exceed the cap. Completed, abandoned and expired sessions do not count. An inactive session expires after 24 hours and is removed by the central cleanup task. `.history` reads up to 10 completed or ended snapshots before that same expiry.

The `.privacy` command is an overview. `.privacy-data` privately counts non-expired session records for the requester across all guild and DM scopes and separates active/waiting from completed/ended records. `.privacy-export` privately sends the complete retained session snapshots across those scopes as a JSON attachment. Other participants' Discord IDs are replaced with a neutral label; the export includes session state and retention timestamps. Exports above 8 MiB are rejected without truncation. Player profiles, inventory, progression, full profile inventory/export and confirmed deletion are not implemented. Neither command is a deletion request. Discord independently processes interactions under its own policies; Moonbyte makes no end-to-end-encryption or encryption-at-rest claim.

## Planned command groups

Profiles, collections, inventory, quests, parties, leaderboards, achievements, settings, the persistent Vault, Bazaar, full profile export/delete and server administration remain planned. Add each only with complete validation, authorization, persistence, retention, recovery, tests and documentation.
