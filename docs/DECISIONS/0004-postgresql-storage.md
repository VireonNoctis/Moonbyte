# 0004. PostgreSQL as the session and application store

- Status: Accepted
- Date: 2026-11-01
- Owners: Project maintainer
- Supersedes: The SQLite-only database selection in ADR 0001.

## Context

Moonbyte has moved beyond a command-only shell into a stateful multiplayer game runtime. Sessions must survive process restarts, mutations must be atomic under concurrent actions, and the next product stages include player profiles, collections, guild state, progression, and a transactional earned-only ledger. The original single-file SQLite choice no longer matches the selected product direction.

## Decision

- PostgreSQL is Moonbyte's persistent system of record.
- Rust accesses PostgreSQL through SQLx with the PostgreSQL driver only.
- SQLite and ScyllaDB are not runtime database targets.
- Schema changes are ordered, versioned migrations.
- Game-session snapshots use JSONB with an explicit state version and indexed metadata columns.
- Every session mutation uses a short transaction and row-level lock; validation and serialization happen before the transaction commits.
- Expired sessions are excluded at query time and physically removed by one central cleanup task.
- The initial supported deployment remains one bot process and one PostgreSQL database. Do not claim production multi-instance support until Discord interaction idempotency, duplicate delivery, failover and concurrent load behavior have been tested.
- The frontend never connects directly to PostgreSQL and never owns authoritative game state.

## Alternatives considered

- SQLite: simpler deployment and a strong option for one local process, but it is no longer the desired primary store for the planned transactional and concurrent application workload.
- ScyllaDB: unnecessary operational and data-model complexity for the current application.
- Redis or a message broker: no measured requirement justifies introducing another state system or service.
- Microservices: rejected; the application remains one Rust process with explicit domain, application, persistence and Discord-adapter boundaries.

## Consequences

The bot now requires a reachable PostgreSQL database and a secret DATABASE_URL before startup. Local development includes a PostgreSQL Compose service. CI provisions a disposable PostgreSQL service and runs persistence integration tests against it. Operators must separately provision backups, restore rehearsals, monitoring, secret rotation, network policy and encryption-at-rest controls.

PostgreSQL does not automatically provide end-to-end encryption or prove encryption at rest. The project must not make those claims until the selected host and its backup/key-management configuration are verified.

The database choice can be revisited only through a new accepted decision based on measured operational or workload requirements.