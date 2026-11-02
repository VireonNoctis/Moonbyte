# Changelog

## Unreleased

- Add PostgreSQL-backed player profiles with validated display names, short bios, private-by-default visibility and saved preferences.
- Add private profile commands, a current-schema data inventory, on-demand JSON export and explicit two-step profile deletion.
- Add atomic player deletion that removes the profile and every game-session snapshot containing that player ID.
- Add a PostgreSQL migration for player profiles and extend the ordered migration runner.
- Add PostgreSQL integration coverage for profile creation, editing, preferences, export and deletion.
- Add migration 0003 for completed-game result records and per-player/per-game aggregate statistics.
- Persist results and update every participant's counters atomically with the completed session snapshot, using the session ID to prevent duplicate awards.
- Include result metadata and aggregate statistics in privacy inventory/export/deletion.

- Switch the persistence target from the superseded SQLite decision to PostgreSQL through SQLx (PostgreSQL driver only).
- Add a versioned PostgreSQL migration runner, durable JSONB game-session snapshots, transactional row-locked mutations, restart recovery, 24-hour inactivity expiry and scheduled cleanup.
- Add PostgreSQL-backed runtime integration tests and a CI PostgreSQL service.

- Add transport-independent Rust rules for Tic-tac-toe, Connect Four, Memory Match and Word Scramble.
- Add an in-process session runtime with guild-scoped lobbies, participant-only reads, turn validation, action validation, completion and leave controls.
- Add prefix and slash commands for games, play, join, action, session and leave.
- Add unit coverage for game outcomes, invalid actions, turn order, participant boundaries and guild isolation.
- Document the process-local persistence boundary and remaining runtime work.

