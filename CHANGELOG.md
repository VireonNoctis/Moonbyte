# Changelog

## Unreleased

- Switch the persistence target from the superseded SQLite decision to PostgreSQL through SQLx (PostgreSQL driver only).
- Add a versioned PostgreSQL migration runner, durable JSONB game-session snapshots, transactional row-locked mutations, restart recovery, 24-hour inactivity expiry and scheduled cleanup.
- Add PostgreSQL-backed runtime integration tests and a CI PostgreSQL service.

- Add transport-independent Rust rules for Tic-tac-toe, Connect Four, Memory Match and Word Scramble.
- Add an in-process session runtime with guild-scoped lobbies, participant-only reads, turn validation, action validation, completion and leave controls.
- Add prefix and slash commands for games, play, join, action, session and leave.
- Add unit coverage for game outcomes, invalid actions, turn order, participant boundaries and guild isolation.
- Document the process-local persistence boundary and remaining runtime work.

