# Changelog

## Unreleased

- Switch the persistence target from the superseded SQLite decision to PostgreSQL through SQLx (PostgreSQL driver only).
- Add a versioned PostgreSQL migration runner, durable JSONB game-session snapshots, transactional row-locked mutations, restart recovery, 24-hour inactivity expiry and scheduled cleanup.
- Add PostgreSQL-backed runtime integration tests and a CI PostgreSQL service.

- Add transport-independent Rust rules for Tic-tac-toe, Connect Four, Memory Match, Minesweeper, Word Scramble, Sequence Recall and 2048.
- Add an in-process session runtime with guild-scoped lobbies, participant-only reads, turn validation, action validation, completion and leave controls.
- Add prefix and slash commands for games, play, join, action, session and leave.
- Add unit coverage for game outcomes, invalid actions, turn order, participant boundaries, guild isolation, and 2048 merging, scoring and random tile placement.
- Add Minesweeper regression coverage for first-click safety, exact mine placement, reversible flags, flood fill and terminal outcomes.
- Document the process-local persistence boundary and remaining runtime work.

