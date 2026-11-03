# Guild command configuration

Moonbyte keeps server command settings separate from player profiles. Each Discord server can choose its own prefix and optionally limit bot commands to one channel.

## Permissions

Only members with Discord's Manage Server permission can use the prefix configuration command. The configuration command remains available in every channel so a moderator can correct an unsuitable channel selection.

## Commands

| Invocation | Effect |
| --- | --- |
| /prefix | Show the current prefix and command channel. |
| /prefix changes: set ! #bot-commands | Set the prefix and optionally the command channel. |
| /prefix changes: channel #bot-commands | Change only the command channel. |
| /prefix changes: channel off | Allow commands in every channel. |
| /prefix changes: reset | Remove the guild override and return to DEFAULT_PREFIX. |

The prefix form accepts the same instructions:

- .prefix
- .prefix set mb! #bot-commands
- .prefix channel #bot-commands
- .prefix channel off
- .prefix reset

A prefix must contain 1–8 characters and cannot contain whitespace or a Discord mention. A channel must resolve to a channel belonging to the server where the command was invoked.

## Routing behavior

When a command channel is configured, all Moonbyte prefix and slash commands other than the prefix configuration command are accepted only in that channel. The bot tells users where to go when they try another channel. If no command channel is configured, commands work in all server channels.

Direct messages use the process-wide default prefix and are not affected by guild settings.

## Storage and privacy

Guild settings are stored in PostgreSQL under the guild ID and cached in memory for command dispatch. Each record stores the guild ID, prefix, optional channel ID, last editor ID and timestamps. The settings are not part of a player's profile, JSON export or account deletion. Removing a guild's settings when the bot is uninstalled is a production operations requirement.
