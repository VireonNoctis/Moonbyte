# 0001. Moonbyte product identity and technology foundation

- Status: Superseded in part by ADR 0004 (database choice); other decisions remain accepted.
- Date: 2026-10-31
- Owners: Project maintainer

## Context

Moonbyte is a new Discord gaming bot based on selected game concepts and systems from Lunar. It needs a distinct identity and a foundation suited to efficient Discord event handling, concurrent game sessions, multiplayer state, and maintainability.

## Decisions

- Moonbyte is a separate application and repository. Lunar is only a source of audited game concepts and migration candidates.
- Rust is the primary application language.
- Tokio is the async runtime.
- Serenity is the Discord API library.
- Poise is the command framework for shared prefix and slash command definitions.
- The default prefix is a period (.).
- SQLite is the initial persistent database, accessed through SQLx configured for SQLite only.
- PostgreSQL and ScyllaDB are not selected.
- The initial deployment topology is one bot process with one durable SQLite database file.
- The visual direction is original full-colour pixel art.
- The game catalogue has five pillars: selected Lunar games, original mini-games, multiplayer, adventure/RPG, and puzzle/strategy.
- Progression is non-wagering and designed for healthy replayability.

## Alternatives considered

Python and TypeScript were not selected because the project prioritises Rust's performance characteristics, memory safety, and explicit state modelling. Go was considered credible but Rust was preferred. PostgreSQL and ScyllaDB were excluded because the initial workload does not justify a separate database service. A runtime dependency on Lunar was rejected to keep operation and data independent.

## Supersession note

The initial SQLite-only storage choice is superseded by ADR 0004 — PostgreSQL as the session and application store. This record is retained to explain the original trade-off and is not the current database instruction.

## Consequences

The project uses one application language and async runtime, a simple initial data topology, and a shared prefix/slash command framework. Rust has a steeper learning curve. SQLite's single-writer model constrains the initial deployment. Discord interactions are not suitable for frame-perfect real-time games. Prefix parsing requires the Message Content privileged intent.

Revisit these choices only when measurements or a concrete product requirement show that the current direction is no longer appropriate.
