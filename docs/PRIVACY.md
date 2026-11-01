# Moonbyte Privacy Policy and Data Practices

- Policy owner: Moonbyte maintainers.
- Status: Intended product policy; active game sessions and minimal cosmetic profile preferences are persisted in PostgreSQL. Complete privacy inventory, export and account-deletion controls are not yet implemented.
- Last updated: 2026-10-31.
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

These are requirements for implementation. The current runtime stores active session records in PostgreSQL: session IDs, game IDs, Discord participant IDs, optional guild IDs, game state snapshots, phase, action count and expiry metadata. Sessions are recoverable after process restart and are removed after 24 hours without a successful mutation, subject to the 15-minute cleanup interval. The runtime additionally stores a minimal player profile containing a Discord user ID and a validated board-theme preference. It does not yet persist player statistics, inventories or a complete game-results history. The `.privacy` command does not provide a complete data inventory, export or account-deletion flow. A player can remove only the cosmetic theme row with `.profile delete confirm`; this does not delete session history or other records. These limited facts describe this code slice only; broader commitments must be verified before production release.

## 2. Data Moonbyte needs

The planned bot may process or store these categories when the relevant feature is enabled.

| Data category | Examples | Why it is needed |
| --- | --- | --- |
| Discord identifiers | User ID; guild ID; channel or message ID for an active game where necessary. | Identify the player, scope settings, and update an active game message. |
| Player profile | Currently: Discord user ID and one fixed board-theme preference. Planned: accessibility and other explicit display preferences. | Associate and render the player's selected cosmetic presentation. |
| Game state | Current turn, board state, action data needed to resume, participants, quest checkpoint. | Run, recover, and fairly resolve games. |
| Game results | Game type, result, completion time, personal best, statistics. | Show requested records, achievements, and leaderboards. |
| Progress and collections | Earned experience, achievements, items, quests, cosmetic unlocks. | Provide persistent progression features. |
| Guild settings | Enabled games, leaderboard visibility, channel configuration, server preferences. | Apply administrators' choices within that guild. |
| Operational diagnostics | Event type, command name, error category, timing, pseudonymous correlation ID. | Diagnose outages, bugs, rate limits, and performance without recording message contents. |
| Security records | Limited identifiers and event metadata associated with an incident. | Protect the bot and investigate abuse. |

Moonbyte should not require Lunar account linking, an email address, real name, or separate website account.

The currently persisted session fields are the session reference, game ID, player Discord IDs, optional guild ID, phase, serialised board/game state, action count, short result text and timestamps. They are stored in PostgreSQL JSONB plus indexed metadata columns. A separate player_profiles row stores the Discord user ID and one of four fixed cosmetic board themes; it contains no free-text biography, external identity or real-world contact fields. Raw Discord message content is not saved as an analytics transcript. Sessions recover after restart and expire after 24 hours without a successful mutation.

## 3. Message and interaction content

Discord sends interaction data to the bot so it can process commands and game actions.

- Prefix commands require Discord's Message Content privileged intent. When enabled, the bot parses prefix and arguments in memory.
- Slash commands deliver structured interaction payloads.
- A game input such as a guess, quiz answer, or move may be stored as part of active game state when necessary to validate or resume that game.
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
| Active session state | While active, with a maximum 24-hour recovery window after inactivity; expired rows are removed by the central cleanup task. |
| Cosmetic player profile | The theme preference remains until the player removes it with `.profile delete confirm` or a future full account-deletion flow. |
| Player settings, achievements, progression | Until profile deletion or a documented feature migration. |
| Guild settings | While the bot remains installed and settings are needed; delete or anonymise after removal, target maximum 30 days. |
| Routine operational logs | Target maximum 30 days, then delete or aggregate without direct identifiers. |
| Security incident records | Only as long as reasonably necessary; restrict access and document the purpose. |
| Backups | Rotate on a documented schedule; deleted profile data should expire from backups within a target maximum of 30 days. |

These are implementation targets. Before launch, maintainers must configure and test cleanup jobs and backup rotation. Data must not be kept longer merely because storage is cheap.

## 7. Privacy command and user controls

The current `.privacy` command is informational only. The `.profile delete confirm` action removes only the theme preference row. It does not delete session records or represent full account deletion. The planned `.privacy` command group is expected to provide:

| Command | Purpose |
| --- | --- |
| .privacy | Read a short privacy summary and open controls. |
| .privacy data | Show categories of data currently associated with the requesting user. |
| .privacy export | Request a copy of the user's stored profile and game data. |
| .privacy delete | Begin explicit, confirmed deletion of all user-owned Moonbyte data. Not implemented yet. |
| .privacy server | Let an authorised server administrator inspect guild-specific configuration and deletion controls. |

Slash-command subcommands should mirror these actions. User exports must be private and delivered only to the requesting user. If private delivery fails, the bot must not post the export publicly; it should explain the failure and allow a safe retry.

### Deletion flow

1. Explain exactly what will be removed and that Moonbyte cannot delete records held by Discord or other server members.
2. Require a separate confirmation interaction.
3. Re-check that the requester owns the profile.
4. End or detach active sessions safely; do not corrupt other players' sessions.
5. Delete or anonymise all user-owned records across profiles, sessions, results, achievements, inventory, quest progress, and indexes.
6. Remove user identifiers from retained aggregates where practical and permitted.
7. Record only minimal operational evidence that the deletion operation completed, without retaining deleted profile contents.
8. Report whether the database operation completed or failed. Never claim success before it is confirmed.

The current profile preference command only deletes the profile row; the complete deletion workflow above is planned, not implemented. The user may still appear in Discord messages or records retained by Discord or other server members. Moonbyte cannot delete Discord's own records.

## 8. Guild privacy

- Guild settings and server leaderboards must be keyed and queried by guild ID.
- A server administrator may inspect and delete that guild's configuration.
- Server administrators must not export or delete another user's global profile through guild controls.
- A guild leaderboard should expose only what its visibility setting permits.
- Private profile fields and exports must not be exposed in public game channels.
- Bot removal should trigger a documented cleanup path for guild-only configuration, subject to backup rotation.

## 9. Security protections

Planned safeguards:
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

## 11. Policy changes and contact

A change to data categories, purposes, retention, or external providers requires code review and an update to this policy. Significant changes should be announced in release notes before they take effect where practical.

For privacy questions that cannot be completed through the bot, use the repository's published support contact once one is configured. Do not post private personal data in public issues.

## 12. Production launch checklist

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
