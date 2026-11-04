# Moonbyte Architecture

> Technical source of truth. Live product behavior is listed in [Implementation Map](IMPLEMENTATION.md).

## System boundary

Moonbyte is one Rust application whose required player surface is Discord. Serenity/Poise translate Discord commands and interactions into application-service calls. The application services use PostgreSQL through SQLx. Game rules remain transport-independent; Discord does not calculate trusted outcomes or own player progress.

```text
Discord Gateway / Commands / Message Components
                    |
             Serenity + Poise
                    |
          Discord Adapter Layer
                    |
      Rust Application / Domain Services
        |      |       |        |
      Games  Players  World   Progression
        |      |       |        |
       PostgreSQL through SQLx
```

No separate web client, visual-library service, broker or microservice is required.

## Technology baseline

| Concern | Selection |
|---|---|
| Language | Rust 2024 |
| Async runtime | Tokio |
| Discord API | Serenity |
| Commands | Poise |
| Database | PostgreSQL |
| Database access | SQLx, PostgreSQL driver |
| Presentation | Discord messages, embeds, buttons, select menus and modals |
| Art | Original full-colour pixel art and custom emoji where useful |

The checked-in toolchain file is authoritative. CI records the version used by the run. Keep the initial deployment to one supported bot process until duplicate interaction delivery, concurrent-load behavior and failover are proven.

## Code layers and ownership

- **Domain:** pure game rules, player progression rules, Atlas graph, route definitions and state transitions. No Serenity, Poise or SQLx types.
- **Application/persistence:** player, game, wallet, quest, Atlas, Chronicle, guild and Expedition stores. PostgreSQL is authoritative.
- **Discord adapter:** command parsing, identity/guild extraction, response formatting and component routing. All user input is untrusted.
- **Composition root:** `src/main.rs` creates stores, registers commands, starts background retention work and owns graceful shutdown.
- **Configuration:** environment variables are validated before database/gateway startup. Secret values are never printed.

## Game session lifecycle

1. Start a session or guild-scoped lobby.
2. Persist the initial versioned JSONB snapshot.
3. For each mutation, select the session row for update, decode and validate its state, check guild/participant/phase/turn constraints, apply the pure game rule and persist the new snapshot in a short transaction.
4. Commit before sending or editing a Discord response.
5. Reconstruct state from PostgreSQL after process restart. A session ID is a locator, never an authentication token.
6. Expire waiting/active sessions after 24 hours without a successful mutation; one central cleanup task removes expired rows. Reads do not refresh expiry.

Expedition runs have their own durable state machine and checkpoint history. Unlike ordinary game sessions, a paused Expedition has no attendance deadline and is not removed for inactivity.

## PostgreSQL and consistency

- Ordered SQL migrations are recorded in `moonbyte_schema_migrations` and guarded by an advisory transaction lock at startup.
- Session mutations use row-level locks and short transactions. Do not hold a transaction over Discord HTTP calls.
- Game result, statistics, first-completion ledger credit and achievement changes commit with the completed session.
- Atlas travel, first discovery and the matching Chronicle fragment commit together.
- Expedition checkpoint history and each active participant's Atlas/Chronicle effects are written in the same transaction through owning service boundaries.
- Quest reward claims use a unique player/date/objective key and an immutable ledger entry.
- Guild settings use PostgreSQL notifications to invalidate per-process cache entries.
- State/content versions are explicit; unsupported persisted versions fail clearly instead of silently resetting.

## Privacy and security boundaries

- Every player-owned table has a clear owner and export/deletion path.
- Guild settings are not player profile data. Guild permissions do not grant access to private player history.
- Private profiles, statistics, quest state, Chronicle pages, rivalries and Expedition details are returned privately where required.
- Raw message text is not persisted as analytics. Tokens, authorization headers, private payloads, quest seeds and unnecessary player identifiers are not logged.
- No client-supplied score, winner, reward, currency balance or checkpoint is trusted.
- Discord delivery failure never rolls back a committed game or Expedition state change; retry behavior must avoid duplicate effects.

## Discord components

Tic-tac-toe, Connect Four and Memory Match boards expose numbered native Discord buttons. Each custom ID binds the session ID, expected action count and a bounded game-specific input. The global interaction handler rechecks current consent, server command-channel policy, participant identity, guild scope and session state before forwarding the input to the same GameRuntime used by text commands. If the action count has advanced, the old board is refreshed from PostgreSQL without applying the move. The expected count is checked again inside the same row-locked mutation transaction, closing the race between the initial read and concurrent clicks. Terminal controls are disabled. Word Scramble remains text-answer driven. The existing text command route remains available as an accessible fallback.

## Logging, shutdown and operations

The runtime uses `tracing` with an environment filter and a compact formatter. The default filter enables Moonbyte application events at info level and keeps Serenity/SQLx comparatively quiet. Events cover process startup/stopping, configuration progress, database readiness, gateway startup/stopping, shutdown signals, expired-session cleanup and guild cleanup.

Log fields are deliberately minimised: no Discord user or guild identifiers, raw messages, private payloads, token, database URL or secret quest/fairness seeds are emitted. Production operators should avoid enabling verbose SQLx query logging because database diagnostics may expose sensitive values.

The runtime handles Ctrl-C on all supported platforms and SIGTERM on Unix, then requests all Discord shards to shut down. Process exit drops background tasks and the SQLx pool cleanly.

The current supported deployment is single-process. Production readiness still requires database backup/restore automation, monitoring/alerts, secret rotation, capacity/load testing, interaction duplicate-delivery handling and a rehearsed recovery procedure. PostgreSQL/hosting encryption-at-rest properties must be verified with the operator; the application does not claim them by default.

## Verification

Run:

```bash
python3 scripts/validate.py
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

These checks do not replace a live Discord smoke test, production backup/restore rehearsal or multi-instance certification.
