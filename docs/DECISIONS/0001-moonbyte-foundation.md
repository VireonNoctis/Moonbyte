# 0001. Product identity and original technology foundation

- Status: Superseded in part by ADR 0004 for persistence; the remaining product and runtime decisions remain accepted.
- Date: 2026-10-31
- Owners: Project maintainer

## Context

Moonbyte is a Discord gaming bot that needs its own identity, codebase, game rules, persistent player state and release lifecycle. It must support turn-based games, durable multiplayer sessions and maintainable cross-system progression.

## Decisions

- Moonbyte is a standalone application with its own repository, runtime, database schema and deployment.
- Rust is the primary application language.
- Tokio is the asynchronous runtime.
- Serenity is the Discord API library.
- Poise provides shared prefix and slash-command definitions.
- The default prefix is a period (.).
- SQLite was selected for the earliest prototype; this persistence choice is superseded by ADR 0004, which selects PostgreSQL.
- The initial deployment topology is one bot process with one PostgreSQL database.
- The visual direction is original full-colour pixel art.
- The product pillars are original mini-games, multiplayer, adventure/RPG, and puzzle/strategy.
- Progression is non-wagering and designed for healthy replayability.

## Alternatives considered

Rust was preferred for memory safety, performance and explicit state modelling. Go was considered credible but not selected. Discord interactions are event-driven and are not a suitable target for frame-perfect games. A single Rust application with clear domain and adapter boundaries is preferred over premature microservices.

## Supersession note

The early SQLite decision is historical only. [ADR 0004](0004-postgresql-storage.md) is the current source for database selection and storage behavior.

## Consequences

The application uses one primary language, one async runtime and one PostgreSQL-backed system of record. Prefix parsing requires Discord's Message Content privileged intent. The deployment remains single-process until interaction idempotency, failover and concurrent-load behavior are proven.
