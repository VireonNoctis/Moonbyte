# Moonbyte Roadmap

- Status: In progress.
- The initial Rust workspace, configuration bootstrap, Rust CI checks, and first Discord command runtime are implemented; SQLite and game/session runtime work remains.
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
- [x] Cargo application workspace and committed lockfile.
- [x] Configuration loading and validation from environment variables.
- [x] Serenity client with Tokio runtime and Poise command framework.
- [x] Default period prefix and global slash-command registration.
- [x] Help, about, privacy overview, and minimal diagnostics (`ping`).
- [x] Clear configuration and startup errors.
- [ ] Structured logging and graceful shutdown.
- SQLite connection, migration runner, and backup path.
- [x] CI for formatting, Clippy, compilation, and tests.
- Next.js + React + TypeScript visual-library workspace.
- Discord Activity integration spike using the Embedded App SDK, with browser fallback.

Exit criteria:
- Bot connects and reconnects without leaking tasks.
- Prefix and slash smoke tests work in a development guild.
- [x] Missing configuration fails clearly.
- SQLite opens, migrates, and closes safely.
- [x] CI is reproducible on a clean checkout.

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


## Cross-cutting expansion — Moonbyte World Loom

**Status: Product design proposal only; these systems are not implemented.**

The [Moonbyte World Loom master atlas](atlas/README.md) expands the product into 400 connected system proposals across 40 domains. It is intentionally broader than the approved game catalogue: the Tavern, Bazaar, Guilds, Atlas, Expeditions, story, Vaults, crafting, collections, Arcade, multiplayer, characters, mastery, festivals, homes, player creation, fair play, social systems and trust/accessibility foundations form one graph.

### Integration order

1. Establish the authoritative Rust game/session contract, player identity and SQLite persistence.
2. Deliver the Tavern as a navigable entry surface with one polished solo game and one cooperative game.
3. Add Atlas locations, a bounded Expedition and one authored Quest/Vault route.
4. Add Archive records and deterministic progression so discoveries persist and can be exported/deleted.
5. Add known-input crafting and item provenance.
6. Add fixed-price Bazaar listings only after immutable ledger, escrow, idempotency and double-spend tests pass.
7. Add guild projects, community events and creator tooling only after guild isolation and moderation workflows are complete.
8. Expand content through validated vertical slices; do not attempt to implement all atlas proposals at once.

### World Loom exit criteria

- Every implemented node has a stable ID, owning domain, documented input/output contract and typed graph relationships.
- Every cross-system edge is backed by a real service contract or explicitly labelled as a future proposal.
- Rust remains the sole authority for state, outcomes, economy and permissions; the React Activity remains presentation-only.
- SQLite migration, recovery, privacy export/deletion and guild-isolation tests cover every new persisted record.
- Bazaar transactions are atomic and deterministic; no stake, wager, paid chance or cash-value mechanism is introduced.
- All new game sessions have visible rules, clear stop/pause controls, accessibility fallbacks and duplicate-action protection.
- Documentation distinguishes proposed, in-development and implemented status at all times.


## Expansion II — world depth and operating systems

**Status: Product design proposal only; not implemented.**

The [World Loom](atlas/README.md) contains 400 proposals across 40 connected domains. The second layer adds ecological context, transport, settlement services, optional professions, creature habitats, evidence-led research, mystery networks, authored character continuity, construction, restoration, civic governance, content operations, safe declarative extensions, Discord Activity continuity, economy stewardship, security, privacy-preserving insights, localisation, onboarding and advanced accessibility.

These systems must not become independent microservices. Shared contracts remain central: stable content IDs, typed domain events, narrow Rust application services, SQLite transactions, authenticated Discord identity, privacy-scoped view models and one versioned content pipeline.

### Additional integration order

1. Improve core identity, session, permissions, migrations and recovery before increasing persistent feature count.
2. Add Atlas region profiles, known travel routes and one settlement service slice.
3. Add one profession task, one creature observation and one evidence-led mystery to prove knowledge loops.
4. Add construction and conservation projects with validated blueprints and auditable contributions.
5. Add content registry, review, staging and rollback before community packs expand.
6. Add Activity handoff, deep links and cross-client resume without moving game authority into TypeScript.
7. Add market governance, security audits and aggregate insights only after ledger and privacy tests exist.
8. Add localisation and advanced accessibility gates before public content expands.

Do not implement the full breadth at once. Release one connected, testable vertical slice at a time.
