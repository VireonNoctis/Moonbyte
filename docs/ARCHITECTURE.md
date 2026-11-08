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
- **Application/persistence:** player, game, wallet, inventory, quest, Atlas, Chronicle, Vault, guild and Expedition stores. PostgreSQL is authoritative.
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
- Atlas travel, first discovery, the matching Chronicle fragment, account-bound postcard and any authored material grant commit together; inventory writes are idempotent and retain the Atlas source ID.
- A fixed recipe consumes source-backed inventory rows and inserts its account-bound output in one transaction. A per-player advisory lock serialises recipe attempts.
- Expedition checkpoint history and each active participant's Atlas/Chronicle/inventory/Wayfarer effects are written in one transaction. Advancement locks every active participant's account in stable ID order before the run row, then rechecks party membership; leave and account deletion use the same user-to-run lock order.
- Profile creation, game-session entry and account deletion share a per-player advisory lock and re-check both current policy versions inside the transaction. Game actions acquire participant locks in stable order before taking the session row; Expedition checkpoint actions acquire all active participant locks before the run row. Domain events cannot recreate deleted profiles.
- Player JSON export runs every profile, consent, game, progress, wallet and ledger read through one read-only `REPEATABLE READ` transaction. The snapshot prevents a concurrent commit from producing mixed before/after records in one export.
- Puzzle Vault mutations use the same per-player advisory lock as privacy deletion, row-lock the open run and commit each correct save stone atomically. Completion, Chronicle unlock, Wayfarer activity and the one-time keepsake share the final checkpoint transaction.
- Quest reward claims use a unique player/date/objective key and an immutable ledger entry.
- Guild settings use PostgreSQL notifications to invalidate per-process cache entries.
- Guild Project contributions are inserted and aggregated inside the originating shared Expedition transaction. A project-row lock serialises join, leave and contribution; a unique `(project, run, checkpoint)` key prevents duplicate credit.
- Guild Project membership is player-owned and belongs in inventory/export/deletion. Contribution rows deliberately omit contributor IDs; account deletion clears the Expedition actor reference while preserving anonymous server progress. Actual guild removal cascades the server-owned board.
- State/content versions are explicit; unsupported persisted versions fail clearly instead of silently resetting.

## Privacy and security boundaries

- Every player-owned table has a clear owner and export/deletion path. Inventory records retain stable item IDs, quantity, source type, source reference and acquisition timestamp; export covers postcard, material, crafted-item and Vault-keepsake provenance, and account deletion removes all inventory rows. Vault run/checkpoint tables are included in private inventory, JSON export and deletion; incorrect guesses are never persisted.
- Guild settings and project aggregates are server-owned, not player profile data. Project membership is personal and exportable/deletable; the board never exposes enrollee names. Guild permissions do not grant access to private player history.
- Private profiles, statistics, quest state, inventory, Chronicle pages, Vault progress, rivalries and Expedition details are returned privately where required.
- Raw message text is not persisted as analytics. Tokens, authorization headers, private payloads, quest seeds and unnecessary player identifiers are not logged.
- No client-supplied score, winner, reward, currency balance or checkpoint is trusted.
- Discord delivery failure never rolls back a committed game or Expedition state change; retry behavior must avoid duplicate effects.

## Command presentation and input guidance

The Discord adapter owns response formatting without moving domain rules into Serenity/Poise. Substantial slash results use the shared `send_title_panel` helper to render a compact embed title and description; it is used by Craft, Tavern and Games. Active game boards use the game title in their embed while retaining native controls. Panel visibility is selected per command: Craft results are ephemeral for slash invocations and DM-delivered for prefix invocations in a server; browse panels are public and game boards remain visible to participants. Other prefix output retains normal message visibility. Use title panels for meaningful browse/workbench/board views, not every acknowledgement, and preserve private delivery for profile, inventory and other personal data. Finite command arguments use a conservative edit-distance suggestion helper for recipes, game IDs, Atlas destinations and finite action verbs. A suggestion is never an automatic correction and must not execute a command on the player's behalf.

## Discord command presentation

Slash-command fields with finite choices use Poise autocomplete callbacks. Suggestions are built from the owning catalogue (crafting recipes, game definitions, Atlas locations) or the relevant fixed action vocabulary (including Vault actions and chamber IDs), filtered against the partial text and capped at Discord's 25-choice/100-character limits. Craft labels include the output and ingredients, but values are canonical recipe IDs; the handler also accepts the exact display title for prefix convenience. Autocomplete is discovery only and never performs a craft, travel, Vault entry/solve, quest or social action.

The self-only Player Card is a read-model composition of Profile, game statistics, Achievements, Wayfarer/Companion, Rivalry, Chronicle, Inventory and Vault services. The Vault command uses the same title-panel helper, with a separate optional title field so prefix solve answers remain canonical and unmodified. It does not duplicate their state or initialise Atlas progress as a side effect. Public and guild-facing profile projections remain deferred until their visibility and sharing contracts are implemented.

Substantial browse and overview responses use a shared embed title-panel builder. Relevant slash commands expose an optional `title` string; matching prefix commands accept trailing title text after required arguments. The adapter removes duplicate leading Markdown headings, escapes user-supplied titles to prevent mentions/Markdown injection, normalises whitespace and respects Discord's title/description limits. Player-owned slash views and active-session lists use ephemeral embeds; prefix invocations in a server are sent by DM. Public catalogues keep their intended visibility. The title changes presentation only and cannot alter a recipe, destination, player record or game outcome. Expedition catalogue/status panels reuse the same standard visual treatment without changing their multi-part argument grammar.

Finite closed-vocabulary inputs use conservative suggestion text. A close match is shown as a hint; Moonbyte does not silently replace a recipe, game, Atlas destination, Vault action or chamber ID. Incorrect Vault answers return authored hints without storing the submitted answer.

## Discord components

Tic-tac-toe, Connect Four and Memory Match boards expose numbered native Discord buttons. Each custom ID binds the session ID, expected action count and a bounded game-specific input. The global interaction handler rechecks current consent, server command-channel policy, participant identity, guild scope and session state before forwarding the input to the same GameRuntime used by text commands. If the action count has advanced, the old board is refreshed from PostgreSQL without applying the move. Terminal controls are disabled. Word Scramble remains text-answer driven. The existing text command route remains available as an accessible fallback.

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
