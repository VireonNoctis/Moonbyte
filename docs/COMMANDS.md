# Moonbyte Command Reference

- Status: Four playable games, PostgreSQL sessions and minimal player-theme preferences are implemented.
- Framework: Poise on Serenity.
- Default prefix: period; override with DEFAULT_PREFIX.

## Available commands

| Prefix | Slash | Behavior |
| --- | --- | --- |
| .help | /help | Lists commands available in this build and the configured prefix. |
| .about | /about | Shows the application version and current implementation boundaries. |
| .ping | /ping | Confirms handler responsiveness and reports process uptime. It is not a network-latency measurement. |
| .privacy | /privacy | Explains current PostgreSQL session/profile storage and unavailable privacy controls. |
| .profile [theme] | /profile | Views the requester's saved cosmetic theme, sets a supported theme, or removes only that preference after the explicit `delete confirm` phrase. |
| .games | /games | Lists four playable games, player counts and controls. |
| .play <game> | /play | Starts a solo game or creates a two-player lobby. |
| .join <session> | /join | Joins an open two-player lobby in its originating server. |
| .action <session> <input> | /action | Submits a game-specific action after participant, guild, phase and turn validation. |
| .session <session> | /session | Reads the current snapshot of a session in which the requester participates. |
| .leave <session> | /leave | Ends a session the requester participates in. |

All commands use shared Rust handlers for prefix and slash invocation. The help output lists only commands registered in this build. The profile command stores only one validated cosmetic board theme; it does not imply player statistics, inventory, progression or RPG systems are implemented.

## Game controls

| Game | Start argument | Action input |
| --- | --- | --- |
| Tic-tac-toe | tic-tac-toe | 1-based row,column, for example 2,3 |
| Connect Four | connect-four | 1-based column from 1 to 7 |
| Memory Match | memory-match | 1-based position from 1 to 8 |
| Word Scramble | word-scramble | Answer text; the current starter puzzle is BITOR → ORBIT |

Tic-tac-toe and Connect Four create lobbies for two players. A second participant must join from the same Discord server. Solo games start immediately. Session reads are restricted to participants.

## Discord setup

1. Set DISCORD_TOKEN in the process environment. Never commit the token.
2. Invite the application with bot and applications.commands OAuth scopes.
3. Enable the Message Content privileged intent in Discord developer settings if prefix commands are required.
4. Set DEFAULT_PREFIX if the period prefix is not desired.
5. Start the process with cargo run --locked.

The runtime requests only GUILDS, GUILD_MESSAGES and MESSAGE_CONTENT. The Message Content intent is privileged and must be enabled for the application. Slash commands are registered globally during startup; Discord may take up to an hour to propagate global command changes.

## Runtime and privacy boundary

Active session state is stored in PostgreSQL. The JSONB snapshot includes the game state, participant Discord IDs, optional guild ID, lifecycle phase and action count; mutations are committed transactionally with a row lock. Multiplayer lobbies require a guild context, sessions are bound to their originating guild, and session reads are limited to participants. An inactive session expires after 24 hours and is removed by the central cleanup task.

The privacy command is informational only. Profile storage is limited to Discord user ID and a board-theme preference. `.profile delete confirm` removes only that preference row. Player statistics, inventory, progression, a complete stored-data inventory, private export and full account deletion are not implemented. This command must not be treated as a deletion request. Discord independently processes interactions under its own policies; Moonbyte makes no end-to-end-encryption or encryption-at-rest claim.

## Planned command groups

Collections, inventory, quests, parties, leaderboards, achievements, advanced settings, the persistent Vault, Bazaar, privacy data/export/delete and server administration remain planned. Add each only with complete validation, authorization, persistence, retention, recovery, tests and documentation.
