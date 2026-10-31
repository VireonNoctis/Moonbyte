# Moonbyte Roadmap

- Status: Planned.
- This roadmap defines deliverables and exit criteria, not promised release dates.
- A milestone is complete only when its acceptance criteria are met and documented.

## Phase 0 — Product and repository foundation

**Status: Documented in this repository update.**

Deliverables:
- Product vision and clear project boundary from Lunar.
- Accepted stack and data-storage direction.
- Target architecture and game-session lifecycle.
- Game catalogue and legacy migration inventory.
- Privacy policy, privacy-command design, data inventory, and retention targets.
- Phased implementation plan and engineering instructions.

Exit criteria:
- README accurately identifies the product and implementation status.
- Accepted architecture decisions are recorded.
- Planned behavior is distinguished from working behavior.
- Privacy claims are explicitly described as target policy until verified in code.

## Phase 1 — Rust and Discord runtime

Deliverables:
- Cargo application workspace and committed lockfile.
- Configuration loading from environment variables.
- Serenity client with Tokio runtime and Poise command framework.
- Default period prefix and slash-command registration.
- Help, about, privacy overview, and minimal diagnostics.
- Structured logging, graceful shutdown, and clear startup errors.
- SQLite connection, migration runner, and backup path.
- CI for formatting, Clippy, compilation, and tests.
- Next.js + React + TypeScript visual-library workspace.
- Discord Activity integration spike using the Embedded App SDK, with browser fallback.

Exit criteria:
- Bot connects and reconnects without leaking tasks.
- Prefix and slash smoke tests work in a development guild.
- Missing configuration fails clearly.
- SQLite opens, migrates, and closes safely.
- CI is reproducible on a clean checkout.

## Phase 2 — Core game platform

Deliverables:
- Game identifiers, metadata, categories, and registry.
- Shared session lifecycle and state-machine validation.
- Player identity and basic profiles.
- Session creation, joining, leaving, completion, cancellation, and expiry.
- Versioned session persistence and restart recovery.
- Injectable clock and RNG interfaces for deterministic tests.
- Discord interaction adapter with membership and permission checks.

Exit criteria:
- A test game runs end-to-end without embedding Discord types in its rules.
- Invalid, duplicate, stale, and unauthorised actions are rejected.
- A resumable session survives a bot restart.
- Independent sessions do not block each other.

## Phase 3 — Lunar game migration and quick-play catalogue

Deliverables:
- Source, dependency, and licence audit of Lunar game candidates.
- Rebuilt quick-play games with tests.
- Akinator-style guessing flow reviewed for external service/API dependencies.
- Review of the existing card collection system and safe progression rules.
- Game library browsing, per-game statistics, and result screens.

Candidate first ports:
- Coin flip and dice as short, non-wagering games.
- 8-ball as a casual party game.
- Akinator-style guessing if underlying service terms support the use.
- Card collection only after a fairness and privacy review.

Exit criteria:
- Every port has source provenance and a licence decision.
- Core rules are reimplemented or isolated from legacy Discord code.
- No game requires a Lunar account, website, database, or bot process.
- Outcomes and cleanup are covered by tests.

## Phase 4 — Original games and player progression

Deliverables:
- Moonbyte-original mini-games.
- Puzzle/strategy foundation and optional daily challenge.
- Player statistics, achievements, inventory, and cosmetic unlocks.
- Deterministic shops and earned rewards.
- Profile, privacy data, export, and deletion commands.

Suggested first originals:
- Memory match.
- Word scramble.
- Sequence recall.
- A daily logic or pattern puzzle.

Exit criteria:
- Progress persists appropriately.
- Achievements are awarded idempotently.
- Exports are complete and private.
- Deletion covers all user-owned tables and indexes.
- Retention and log-redaction tests pass.
- No wagering or manipulative streak mechanics exist.

## Phase 5 — Multiplayer

Deliverables:
- Public and private lobbies.
- Host controls, invitations, player limits, and ready state.
- Turn-based multiplayer games.
- Turn ownership, action validation, deadlines, leave/rejoin behavior.
- Group statistics and optional guild leaderboards.

Suggested first multiplayer games:
- Tic-tac-toe.
- Connect Four.
- A team quiz or co-op puzzle.

Exit criteria:
- Lobbies recover or close cleanly after disconnects and restarts.
- A non-member cannot control a session.
- Turn order and duplicate actions are correct under concurrency tests.
- Guild data remains isolated.
- One slow or invalid action does not stall other games.

## Phase 6 — Adventure and RPG

Deliverables:
- Original setting and world map.
- Quest definitions and progression flags.
- NPC interactions and dialogue.
- Inventory, collection, crafting, and character progression.
- Turn-based encounters with age-appropriate presentation.
- Party play and save checkpoints.

Exit criteria:
- Save data is versioned and migratable.
- Quest progression cannot be duplicated through retries.
- Players can recover saves after restarts.
- Content and reward rules are documented and fair.
- Story progress can be paused without penalty.

## Phase 7 — Visual library and richer interaction

Deliverables:
- Next.js + React + TypeScript visual library for the approved catalogue.
- Responsive game pages, lobby views, player-facing game view models, and pixel-art asset integration.
- Discord Activity integration through the official Embedded App SDK.
- HTTPS browser fallback for catalogue discovery and compatible game views.
- Authenticated Rust API boundary; the frontend never owns game outcomes, inventory, balances, or persistence.

Exit criteria:
- Ordinary bot embeds and commands remain functional without the frontend.
- Activity launch, refresh, reconnect, participant changes, and close behavior are tested.
- Frontend cannot mutate state without backend authorisation.
- Bot credentials and private seeds never enter frontend bundles.
- Mobile, web, and desktop Activity behavior is tested against current Discord requirements.

## Phase 8 — Vaults, Bazaar and economy

Deliverables:
- Earned-only Moonbits ledger with idempotent entries and transparent reward definitions.
- Personal Vault and collectible archive.
- Puzzle Vault vertical slice with fixed, published rewards and save checkpoints.
- Crafting recipes and item provenance.
- Fixed-price Bazaar listings, barter, escrow, receipts, and trade history.
- Moderation, reporting, rate limits, and guild-scoped controls.

Exit criteria:
- Every balance mutation has an immutable ledger entry in the same SQLite transaction.
- Double-spend, duplicate interaction, race, rollback, and self-trade tests pass.
- No real-money value, wagering, paid random reward, or cash-out feature exists.
- Profile export/deletion and guild-isolation tests cover the new records.
- Every Vault reward and Bazaar price/fee is shown before confirmation.

## Phase 9 — Production readiness and measured scale

Deliverables:
- Load and soak tests for concurrent sessions.
- Database backup and restore drills.
- Operational dashboards or structured alerting only where justified.
- Rate-limit, failure, and recovery testing.
- Privacy retention, export, and deletion drills.
- Moderation and server-configuration controls.
- Sharding or additional workers only when justified by measured load.

Exit criteria:
- Performance limits are measured and published rather than guessed.
- Recovery and privacy workflows have been exercised.
- No known cross-guild data isolation failures remain.
- Operators can diagnose degradation without logging private message content.
- Public privacy claims match deployed behavior.

## Ideas backlog

Potential additions, deliberately outside the initial commitment:
- Seasonal events with no punitive missed-event mechanics.
- Community-created puzzle packs with moderation and validation.
- Spectator mode for supported games.
- Cooperative guild milestones.
- Cosmetic themes and profile frames.
- A web companion for public game records, only if Discord UX demonstrates a real need.

Backlog ideas must not delay the runtime, game engine, privacy foundation, or first polished games.
