# Moonbyte Command Reference

- Status: The first Discord command slice is implemented.
- Framework: Poise on Serenity.
- Default prefix: `.`; override with `DEFAULT_PREFIX`.

## Available commands

| Prefix | Slash | Behavior |
| --- | --- | --- |
| `.help` | `/help` | Lists the commands available in this build and the configured prefix. |
| `.about` | `/about` | Shows the application version and clearly states which product systems are not implemented. |
| `.ping` | `/ping` | Confirms that a command handler responded and reports process uptime. It is not a Discord network-latency measurement. |
| `.privacy` | `/privacy` | Explains current interaction handling, the absence of SQLite persistence, and unavailable privacy controls. |

All four commands use shared Rust handlers for prefix and slash invocation. The help output lists only implemented commands; it does not advertise planned game or economy features as working.

## Discord setup

1. Set `DISCORD_TOKEN` in the process environment. Never commit the token.
2. Invite the application with the `bot` and `applications.commands` OAuth scopes.
3. Enable the Message Content privileged intent in the Discord developer settings if prefix commands are required.
4. Set `DEFAULT_PREFIX` if the period prefix is not desired.
5. Start the process with `cargo run --locked`.

The runtime requests only `GUILDS`, `GUILD_MESSAGES`, and `MESSAGE_CONTENT`. The Message Content intent is privileged and must be enabled for the application. Slash commands are registered globally during startup; Discord may take up to an hour to propagate global command changes.

## Privacy boundary

The current build processes Discord command interactions in memory and does not connect to SQLite or persist player profiles, game sessions, or results. The privacy command is informational only. Data inventory, private export, and confirmed deletion are not implemented, and the command must not be treated as a deletion request.

Discord independently processes interactions under its own policies. Moonbyte makes no end-to-end-encryption or database-encryption claim. The project policy and production controls must be revisited when persistence is introduced.

## Planned command groups

Game discovery and play, profiles, collections, inventory, quests, parties, leaderboards, achievements, settings, session stop controls, and privacy data/export/delete flows remain planned. They should be added only with complete validation, authorization, state handling, tests, and documentation.
