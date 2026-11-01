# Moonbyte Architecture

- Status: Rust workspace, configuration bootstrap, Discord command adapters, a first transport-independent game/session slice, and a presentation-only Next.js catalogue foundation implemented; PostgreSQL-backed session persistence and restart recovery are implemented; player/profile persistence, live game API, Activity handshake, and authoritative visual rendering remain unimplemented.
- Last updated: 2026-10-31.

This document describes intended boundaries and constraints. It must be revised to match the implementation as it is built; do not treat planned components as existing services.

## 1. Technology baseline

| Concern | Selected technology |
| --- | --- |
| Language | Rust |
| Async runtime | Tokio |
| Discord API | Serenity |
| Command framework | Poise |
| Persistent storage | PostgreSQL |
| Database access | SQLx with PostgreSQL support only |
| Visual library | Next.js + React + TypeScript |
| Discord Activity bridge | Official Discord Embedded App SDK |
| Logging and diagnostics | Structured tracing |
| Art pipeline | Original pixel-art source assets and cached rendered output |

Poise is built on Serenity and supports prefix and slash commands from shared command definitions. Serenity manages Discord Gateway connections and supports sharding. Tokio is the async runtime. The executable uses Tokio, Serenity through Poise, and a committed dependency lockfile. The command adapter now exposes a game catalogue, start/join/action/session/leave operations in addition to help, about, ping and privacy. Four game modules run without Discord types. The session runtime loads and mutates versioned JSONB session snapshots in PostgreSQL. Mutations use short row-locked transactions; game rules remain independent of Discord and database types. `visual-library/` contains a real Next.js app for browsing only the implemented catalogue; it deliberately does not duplicate authoritative game logic or claim to host live sessions.

References: [Poise documentation](https://docs.rs/poise/latest/poise/), [Serenity documentation](https://docs.rs/serenity/latest/serenity/), [Tokio documentation](https://docs.rs/tokio/latest/tokio/).

## 2. Logical system

~~~mermaid
flowchart TD
    U[Discord users] --> D[Discord Gateway and interactions]
    D --> A[Discord adapter: Serenity + Poise]
    A --> C[Command and interaction handlers]
    C --> S[Application services]
    S --> G[Game registry and session runtime]
    G --> R[Game rules and state machines]
    S --> P[Player, guild, progression and privacy services]
    G --> DB[(PostgreSQL)]
    P --> DB
    G --> V[Pixel-art renderer and asset cache]
    V --> A
    S --> O[Structured logs and metrics]
~~~

The runtime should be one deployable bot process first. Separate services, queues, distributed caches, and multi-region infrastructure are not part of the initial architecture.

## 3. Component boundaries

| Component | Responsibility | Must not own |
| --- | --- | --- |
| Discord adapter | Gateway events, prefix/slash parsing, interaction acknowledgements, message edits, buttons, menus, modals. | Game rules or direct SQL queries. |
| Application services | Orchestrate start game, join lobby, submit action, finish game, update profile, export, and delete use cases. | Discord transport details. |
| Game registry | Discover validated game definitions and metadata. | Persistent player data or gateway connections. |
| Session manager | Create, find, authorise, transition, expire, and recover sessions. | Individual game's domain rules. |
| Game modules | Rules, legal actions, state transitions, outcomes, and game-specific presentation data. | Directly sending Discord messages or bypassing session validation. |
| Player/progression services | Profiles, statistics, achievements, inventory, and reward policy. | Guild-private configuration. |
| Privacy service | Data inventory, export, deletion orchestration, retention requests, and privacy command responses. | Bypassing repositories or claiming deletion before it completes. |
| Persistence layer | PostgreSQL repositories, versioned migrations, short transactions, JSONB state snapshots and expiry cleanup. | Discord API calls. |
| Rendering layer | Convert game view models and assets into cached image output or frontend-ready view models. | Mutating game state. |
| Visual library frontend | Next.js pages, React components, catalogue, lobbies, and Activity UI. | Authoritative rules, balances, permissions, or direct database access. |
| Activity bridge | Embedded App SDK handshake and Discord client capabilities. | Owning player identity or trusted game state. |
| Operations layer | Configuration, structured diagnostics, graceful shutdown, and health signals. | Game-specific decisions. |

The primary dependency direction is from adapters toward application services and domain rules. Game logic must remain testable without a live Discord connection.

## 4. Game contract and lifecycle

Every game should provide a stable ID, display metadata, category, player limits, session mode, rules, initial state, validated actions, state transition function, view model, end conditions, results, and a versioned representation when state must be saved.

The platform owns the shared lifecycle:

~~~mermaid
stateDiagram-v2
    [*] --> Created
    Created --> WaitingForPlayers
    Created --> Active: solo start
    WaitingForPlayers --> Active: lobby starts
    WaitingForPlayers --> Expired: lobby timeout
    Active --> Paused: supported interruption
    Paused --> Active: resume
    Active --> Completed: game ends
    Active --> Aborted: permitted cancellation
    Paused --> Aborted: cancellation
    Completed --> [*]
    Aborted --> [*]
    Expired --> [*]
~~~

Every transition must be validated. Reject actions from non-members, the wrong player, stale interactions, duplicate submissions, and actions invalid for the current phase.

## 5. Concurrency model

The product is event-driven, not frame-driven.
- Tokio handles Discord networking, asynchronous storage operations, and coordination.
- Incoming interactions become validated game actions; state changes only on meaningful events.
- Use bounded channels and explicit backpressure for internal queues.
- Scope synchronisation to an individual session or resource. Avoid one global mutex for all games.
- Never hold a lock or transaction across a Discord HTTP request.
- Never block Tokio worker threads with CPU-heavy rendering or synchronous I/O.
- Use cancellation tokens/timeouts for sessions and shutdown paths.
- Do not create a busy polling loop for every idle session. Use event deadlines or a central expiry scheduler.
- Use per-session tasks only where they simplify ownership and lifecycle.
- Treat rate limits and interaction expiry as normal control flow.

A slow game or render must not stall unrelated sessions. Isolate CPU-heavy rendering from async I/O workers and cache output by stable render inputs.

## 6. Persistence: PostgreSQL

PostgreSQL is Moonbyte's persistent system of record. SQLx is compiled with its PostgreSQL driver only; SQLite and ScyllaDB are not part of the runtime.

The first migration creates a monotonic session-ID sequence, a schema migration ledger, a game_sessions table with stable ID, game ID, optional guild ID, lifecycle phase, state version, action count, JSONB session snapshot, timestamps and expiry, plus indexes for expiry and guild/phase queries.

Every session mutation runs inside a short database transaction. Mutating actions select the session row FOR UPDATE, deserialize and validate domain state, serialize the new versioned snapshot, update the row and commit before the Discord adapter sends a response. A rejected action drops the transaction without persisting a partial state change. Session reads verify participant and guild scope.

The initial retention rule expires a session after 24 hours without a successful mutation. One central task removes expired rows every 15 minutes; the runtime also refuses to read or mutate an expired row. Do not retain expired snapshots indefinitely.

The current supported deployment remains one bot process with one PostgreSQL database. PostgreSQL row locking provides atomic per-session mutations, but do not claim multi-instance production support until Discord interaction idempotency, duplicate delivery, process failover, pool sizing and concurrent load tests are complete.

Backup, restore, managed-database selection, connection-secret provisioning and production encryption-at-rest configuration remain operator/deployment work. Never log the connection URL or claim that selecting PostgreSQL automatically proves encryption at rest.

Likely future data groups include player profiles, settings, guild settings, results, achievements, inventory, quest progress, ledger entries and privacy-operation metadata. Add each through reviewed migrations and update the privacy inventory before enabling the feature.
## 7. Discord command and interaction adapter

Default prefix: **.** Poise should expose shared command definitions as prefix and slash commands where appropriate.

Prefix commands require Discord's Message Content privileged intent. Enable it in developer settings and request it in code. Request only intents the implementation needs.

Interaction rules:
- Acknowledge or defer interactions before Discord's response deadline.
- Edit an existing game message for state changes where possible instead of flooding channels.
- Use ephemeral responses for private information, exports, and validation feedback where supported.
- Check that the clicking user's identity and permissions match the action.
- Treat deleted messages, expired interactions, user departure, and API errors as recoverable failure paths.
- Keep public multiplayer boards separate from personal inventory and profile details.

Reference: [Discord Gateway intents](https://discord.com/developers/docs/topics/gateway#privileged-intents).

## 8. Privacy service and data boundaries

The privacy service should provide overview, data inventory, private export, confirmed deletion, and guild-configuration deletion for authorised administrators.

- Repositories must expose explicit user-owned data queries so export/deletion cannot silently omit a table.
- Deletion must be one coordinated, auditable operation with idempotent retries.
- A deletion confirmation must be bound to the requesting Discord user and expire.
- Do not store export files permanently. Generate them on demand and deliver them privately.
- Ensure logs record operation outcome and correlation metadata, not export contents or deleted values.
- Retention cleanup must be implemented as bounded, observable jobs.
- Guild-scoped configuration and server-only records must remain isolated by guild ID.
- Tests must prove that a user's data cannot be retrieved by another user and one guild cannot inspect another guild's records.

See [PRIVACY.md](PRIVACY.md) for data categories, purposes, retention targets, and encryption limitations.

## 9. Pixel-art rendering

Pixel art is a first-class part of the product.
- Maintain an original art direction and documented palette per game/world.
- Store source assets in a predictable, licensed asset directory.
- Prefer reusable sprite sheets and atlases over duplicated one-off images.
- Separate source sprites from runtime render output.
- Cache render results by game, state version, theme, and relevant viewport inputs.
- Re-render only when visible state changes.
- Keep dimensions and file sizes suitable for Discord.
- Ensure visual state is also conveyed through text or accessible labels; colour alone must not encode critical information.

Do not commit generated caches. Do not use third-party assets unless their licence allows the intended distribution.

## 10. Visual library and Discord Activity boundary

The visual library is a separate **Next.js + React + TypeScript** application. It can run as a normal HTTPS web app and, where configured, as a Discord Activity embedded in the Discord client.

- Use Discord's official Embedded App SDK for Activity handshake and supported client integration.
- Keep Discord-native bot commands and ordinary embeds available as the baseline/fallback interface.
- The frontend requests authorised view models and sends intent/action requests to the Rust application API.
- The Rust backend validates the Discord identity/session context, guild, membership, permissions, game phase, and turn ownership for every state-changing action.
- Never trust client-side validation, client-supplied balances, client-supplied item ownership, or client-generated game outcomes.
- Do not expose the bot token, database URL/credentials, server seed before reveal, or backend credentials to browser code.
- Use HTTPS, strict origin/CORS policy, secure session handling, Content Security Policy appropriate to Discord Activity hosting, and bounded API requests.
- Treat Activity startup, reload, participant changes, and disconnects as normal lifecycle events. The backend remains authoritative if the frontend reloads.
- Do not duplicate game rules in TypeScript. Share schemas/types where useful, but keep game outcomes and state transitions in Rust.

Official references: [Discord Activities](https://discord.com/developers/docs/activities/overview) and [Embedded App SDK](https://discord.com/developers/docs/developer-tools/embedded-app-sdk).

## 11. Security and trust boundaries

- Discord user and guild identifiers are untrusted until checked against interaction context.
- Authorise every action server-side.
- Check session ownership, membership, current turn, and guild scope for every mutation.
- Validate string lengths, enum values, numeric ranges, and payload sizes before persistence or rendering.
- Apply command and game-action rate limits per user and guild.
- Treat external API responses and imported legacy data as untrusted.
- Keep secrets in environment variables or deployment secret stores.
- Avoid logging raw message content, tokens, private export contents, or unnecessary user data.
- Make profile deletion and retention explicit.

## 12. Reliability and observability

Initial operational signals:
- Gateway connection and reconnect state.
- Command and interaction success/failure counts.
- Active sessions and transitions by game.
- Session timeout and recovery counts.
- PostgreSQL query latency, pool saturation, migration version and expired-session cleanup.
- Render duration and cache hit rate.
- Rate-limit and interaction-expiry errors.
- Privacy export/deletion outcomes and retention-job status, without logging the data itself.

Use structured logs with correlation identifiers for command and session flows. Do not log secrets or full user-generated message content.

Shutdown should stop new session creation, cancel or drain active work within a defined policy, persist resumable state, close the database pool, and exit. Startup should validate configuration and migrations before accepting play.

## 13. Fairness transcript boundary

Random outcomes requiring verification use the commit–reveal protocol in [FAIRNESS.md](FAIRNESS.md). Generate secrets using OS-backed CSPRNG entropy; use SHA-512 only as the commitment hash and documented derivation primitive. Store append-only transcript events, do not permit hidden rerolls, and make aborts verifiable. The web UI may display verification state but cannot generate or certify results.

The economy ledger, Bazaar transactions, and Vault rewards are deterministic and cannot depend on a random draw.

## 14. Scaling policy

1. Start with one bot process, one PostgreSQL database, and bounded connection-pool concurrency.
2. Measure latency, database contention, memory use, and active-session volume.
3. Optimise query plans, cache repeated renders, and reduce redundant Discord edits.
4. Use Serenity's supported sharding when required by Discord's operational limits.
5. Consider separate workers or a different persistence topology only when measurements show a concrete bottleneck.

Do not distribute the system merely to appear enterprise-grade.


## 15. Current implementation snapshot

The current playable slice is deliberately small and honest about its limits:

- Rust domain rules implement Tic-tac-toe, Connect Four, Memory Match and Word Scramble.
- An in-process session registry provides start, guild-scoped join, validated action, participant-only read and leave operations.
- Poise exposes the same operations as prefix and slash commands.
- The registry uses a short synchronous mutex only around in-memory operations. It performs no await, Discord request or disk I/O while holding the lock.
- Session IDs are process-local references, not durable IDs or authentication credentials.
- PostgreSQL session persistence, migrations, and restart recovery are implemented. Player profiles, achievements, inventory, economy, privacy export/deletion and the live web game API remain unimplemented.

This is a functional vertical slice for domain rules, Discord commands, PostgreSQL persistence and single-process restart recovery—not yet a production-ready multi-instance deployment. The next gate is player/profile persistence, end-to-end privacy workflows, idempotent Discord interaction handling and concurrent-session load tests.
