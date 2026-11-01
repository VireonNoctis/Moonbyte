# Moonbyte Game Runtime — first durable slice

- Status: Implemented Rust domain, Discord adapter, PostgreSQL session persistence and restart recovery.
- Persistence: PostgreSQL JSONB snapshots with row-locked mutation transactions and versioned schema.
- Retention: Sessions expire after 24 hours without a successful mutation; expired rows are centrally deleted every 15 minutes.
- Authority: Rust owns game rules, participants, phases, turn order and outcomes.

This document describes the implementation that currently exists. It is intentionally narrower than the World Loom product specification.

## Current game modules

| Stable ID | Mode | Rules currently implemented |
| --- | --- | --- |
| tic-tac-toe | Two players | 3×3 board, legal placement, occupied-cell rejection, turn order, horizontal/vertical/diagonal wins and draw detection. |
| connect-four | Two players | Seven columns, six rows, gravity-based placement, full-column rejection, four-direction win detection and draw detection. |
| memory-match | Solo | Eight positions, four fixed pairs, duplicate/matched-position rejection and completion after all pairs are found. |
| word-scramble | Solo | One original five-letter starter puzzle, case-insensitive answer checking and three bounded attempts. |

Memory Match currently uses a fixed starter arrangement so the first implementation does not imply an unverified random shuffle. Word Scramble currently contains one starter puzzle. Neither limitation should be described as a complete production content catalogue.

## Session lifecycle

1. Start: a solo game enters Active; a two-player game enters WaitingForPlayers. A PostgreSQL sequence generates durable session IDs.
2. Join: the first distinct player from the same guild fills the two-player lobby.
3. Act: the runtime selects the row for update, validates phase, guild scope, participant membership, turn owner, action range and game-specific legal move, then commits the new snapshot.
4. Complete: game rules produce a winner, draw or completed solo result; the final session snapshot is persisted.
5. Leave: any participant may end an unfinished session; the resulting phase is persisted.
6. Read: the runtime loads from PostgreSQL and only exposes a snapshot to a participant in the matching guild context.
7. Recover: a new GameRuntime instance can read the same session after the old process/runtime is gone, until its inactivity expiry.

A session ID is a reference, not a security credential. Access still requires participant identity and guild scope. Reads do not extend the 24-hour inactivity window; successful mutations do.

## Persistence contract

The initial migration creates a schema migration ledger, a PostgreSQL sequence for session IDs and a game_sessions table. Each row stores a stable session ID, game ID, optional guild ID, lifecycle phase, state schema version, action count, JSONB snapshot containing game state and player membership, timestamps and an inactivity expiry.

Every mutating operation runs in a short SQL transaction. Session rows are selected FOR UPDATE, state is deserialised and validated by the transport-independent domain, then a new JSONB snapshot is written and the transaction commits. Rejected domain actions are not persisted. No database transaction is held across Discord HTTP calls.

The migration runner serialises concurrent process startup with a PostgreSQL advisory transaction lock. One central Tokio cleanup task removes expired rows every 15 minutes; expired rows are also excluded from read and mutation queries. Backup/restore, production database provisioning and deployment encryption-at-rest remain operator responsibilities.

## Ownership boundaries

- The game engine contains pure rule transitions and does not import Serenity, Poise or SQLx.
- The runtime coordinates session lifecycle and uses PostgreSQL as its durable source of truth.
- The command layer converts Discord identity and guild context to the runtime contract.
- The frontend, when introduced, must submit intent to Rust and must never calculate trusted outcomes or ownership.
- No game state is written directly by a command handler.

## Error and abuse handling

The runtime rejects unknown games, malformed or out-of-range actions, occupied cells, full columns, already-matched positions, repeated positions, wrong turns, non-members, incorrect guild context, non-joinable lobbies and actions after a session ends or expires. A rejected domain action is not persisted and does not advance the action count. The runtime does not accept client-supplied winner, score, balance or completion fields.

## Tests

Pure domain tests cover parser bounds, win detection, invalid moves, game completion and bounded attempts. PostgreSQL-backed runtime tests cover distinct durable IDs, guild-scoped lobby joins, DM multiplayer rejection, private session reads, membership and turn ownership, transactional rejection, leave/completion and runtime reconstruction against persisted state. CI starts an isolated PostgreSQL service for these tests.

These are automated tests, not a Discord live-guild smoke test, multi-process load test, backup/restore rehearsal, or production deployment certification.

## Known gaps before the next release gate

- Durable player identity, profiles, preferences, results, achievements, inventory and progression.
- Privacy data inventory, private export, confirmed deletion and retention across all future user-owned tables.
- Interaction idempotency keys and protection against duplicate Discord delivery after a process crash.
- Multi-instance deployment, reconnect behavior and concurrent load/soak testing.
- Backup/restore automation, database monitoring and production secret provisioning.
- Discord button/select interactions and message edits for board updates; current input still uses commands.
- A deterministic content registry and multiple reviewed Memory Match / Word Scramble puzzles.
- Structured logging, graceful shutdown and service health reporting.
- A visual library and Discord Activity using Next.js, React, TypeScript and the official Embedded App SDK.

Do not mark these items complete until code, integration tests and operational documentation prove them.