# Moonbyte Roadmap

> Product scope and cross-system relationships are governed by [Moonbyte Final Vision v4.0](MOONBYTE_FINAL_VISION.md) and the [World Loom Integration Contracts](atlas/INTEGRATION_CONTRACTS.md). Roadmap status below reflects implementation, not proposal count.

- Status: In progress.
- The Rust/Discord runtime, four playable game modules, PostgreSQL session persistence/recovery, player profiles/preferences, completed-game results/statistics, five achievement milestones, earned-only Moonbits wallet/ledger, the 11-destination Tavern Wayfinder, the five-location Atlas, optional daily quests, cumulative companion milestones and mutually accepted private rivalries are implemented. Inventory, crafting, bounded Expeditions, authored story quests, production recovery operations remain.
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
- [x] Default period prefix, guild-specific prefix overrides and global slash-command registration.
- [x] Manage Server command for setting the allowed command channel across prefix and slash invocations.
- [x] Versioned Terms of Service and Privacy Policy review/acceptance gate with privacy controls available before acceptance.
- [x] PostgreSQL LISTEN/NOTIFY invalidation for guild command-settings caches.
- [x] Remove guild-owned command settings and active game sessions after confirmed bot removal from a guild.
- [x] Help, about, privacy overview, and minimal diagnostics (`ping`).
- [x] Initial Tavern home and Wayfinder command, linked only to implemented games, profile, statistics, wallet, achievements and Atlas destinations.
- [x] Central custom-emoji registry with safe Unicode fallbacks for unconfigured upload IDs.
- [x] Five achievement milestones with idempotent unlocks.
- [x] Initial five-location Atlas, personal discovery state and direct-route travel with PostgreSQL/privacy coverage.
- [x] Wayfarer activity days, optional fixed-reward daily quests, SHA-512 rotation fingerprints, cumulative cosmetic companion milestones, and mutually accepted private rivalry records.
- [x] Game catalogue and play/session commands for the first four games.
- [x] Clear configuration and startup errors.
- [ ] Structured logging and graceful shutdown.
- [x] PostgreSQL connection pool, schema migration runner, durable game-session repository and JSONB state snapshots.
- [ ] Production backup/restore automation and a rehearsed recovery procedure.
- [x] CI for formatting, Clippy, compilation, and tests.
- [x] Discord-native commands, embeds and message components for the implemented game and player surfaces.
- [ ] Consistent presentation helpers and original full-colour art assets where a live bot feature materially benefits from them.

Exit criteria:
- Bot connects and reconnects without leaking tasks.
- Prefix and slash smoke tests work in a development guild.
- [x] Missing configuration fails clearly.
- [x] PostgreSQL connects, applies versioned migrations, and closes its pool safely.
- [x] CI is reproducible on a clean checkout.

## Phase 2 — Core game platform

Deliverables:
- [x] Stable game identifiers and metadata for the four starter games.
- [x] In-process session lifecycle with start, join, action, read and leave operations.
- [x] State-machine validation for the first four playable game modules.
- [x] Guild scope, participant membership and turn ownership checks.
- [x] Durable session lifecycle and state-machine recovery with PostgreSQL.
- [x] Player identity and basic profile persistence.
- [x] Completed-game result records and per-game aggregate statistics, transactionally tied to session completion.
- [x] Session creation, joining, leaving and game completion through the Rust runtime.
- [x] 24-hour inactivity expiry and one central cleanup task.
- [x] Versioned PostgreSQL session persistence and runtime reconstruction.
- [ ] Injectable clock and entropy interfaces for any future randomised game modules.
- Discord interaction adapter with membership and permission checks.

Exit criteria:
- [x] Four test games run through the domain/runtime API without embedding Discord types in their rules.
- [x] Invalid, occupied, wrong-turn, non-member and cross-guild actions are rejected.
- [ ] Stale interaction tokens, durable idempotency and expiry are implemented.
- [ ] A resumable session survives a bot restart.
- Independent sessions do not block each other.

## Phase 3 — Lunar game migration and quick-play catalogue

Deliverables:
- Source, dependency, and licence audit of Lunar game candidates.
- Rebuilt quick-play games with tests.
- Akinator-style guessing flow reviewed for external service/API dependencies.
- Review of the existing card collection system and safe progression rules.
- Expand the game library beyond the first four games and add polished result screens. Per-game statistics for the current four games are implemented.

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
- [x] Durable per-game statistics for completed sessions.
- [x] Optional fixed-catalogue daily quests, cumulative Wayfarer activity and cosmetic companion milestones.
- Player inventory and additional cosmetic unlocks.
- Deterministic shops and earned rewards.
- [x] Profile and preference commands.
- [x] Current-schema data inventory, private JSON export and confirmed profile/session deletion.
- [x] Completed-result metadata and per-game statistics for the current game catalogue.
- Player inventory and authored long-form quest/story progression (the daily Wayfarer board is implemented separately).

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
- [x] Initial five-location Atlas map with persistent player discovery and connected-route travel.
- [x] Initial five-location Atlas map with persistent player discovery and connected-route travel.
- Original setting and expanded world map, authored regions and landmarks.
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

## Phase 7 — Discord-native visual presentation

Deliverables:
- Consistent Rust helpers for embeds, action rows, selects, modals and private/public response policy.
- Game-specific boards and result screens that use Discord-native components and remain understandable in text.
- Original full-colour pixel-art attachments or custom emoji only for features where artwork materially improves the experience.
- Reusable message layouts for lobbies, checkpoints, journey summaries and Tavern destinations.
- Interaction acknowledgement, edit/follow-up handling, rate-limit behavior and delivery-failure recovery.

Exit criteria:
- Every advertised action is backed by a registered Rust handler and server-side validation.
- Private information is delivered privately and public channel content respects feature visibility.
- Important state is readable without colour or image assets.
- Commands and games remain fully usable without any website, frontend workspace or Discord Activity.
- Discord interaction failures do not reverse committed PostgreSQL state.

## Phase 8 — Vaults, Bazaar and economy

Deliverables:
- [x] Initial earned-only Moonbits wallet and immutable first-completion ledger entry with idempotency key.
- Additional transparent reward definitions and deterministic award paths for later systems.
- Personal Vault and collectible archive.
- Puzzle Vault vertical slice with fixed, published rewards and save checkpoints.
- Crafting recipes and item provenance.
- Fixed-price Bazaar listings, barter, escrow, receipts, and trade history.
- Moderation, reporting, rate limits, and guild-scoped controls.
- [x] Persist guild prefix and command-channel settings independently of player profiles.

Exit criteria:
- [x] The current first-completion credit writes an immutable ledger entry and wallet balance in the same PostgreSQL transaction.
- Every future balance mutation must follow the same invariant.
- [x] Repeat-completion tests prove a player's first-completion reward cannot be claimed again for the same game type.
- [ ] Broader concurrent-credit, rollback and double-spend tests are required before any spending or transfers are enabled.
- No real-money value, wagering, paid random reward, or cash-out feature exists.
- [x] Profile inventory, export and deletion cover the wallet and ledger tables.
- Guild-isolation tests must cover future guild-owned economy records.
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

**Status: Partially implemented.** The initial Tavern Wayfinder and five-location Atlas starter map are implemented. The [Moonbyte World Loom master atlas](atlas/README.md) remains a design proposal for 400 connected system ideas across 40 domains. It is intentionally broader than the approved game catalogue: the Tavern, Bazaar, Guilds, Atlas, Expeditions, story, Vaults, crafting, collections, Arcade, multiplayer, characters, mastery, festivals, homes, player creation, fair play, social systems and trust/accessibility foundations form one graph.

### Integration order

1. Establish the authoritative Rust game/session contract, player identity and PostgreSQL persistence.
2. Deliver the Tavern as a navigable entry surface with one polished solo game and one cooperative game.
3. Expand the implemented five-location Atlas with region profiles, then add a bounded Expedition and one authored Quest/Vault route.
4. [x] Add the first private Chronicle archive records and deterministic first-discovery unlocks with export/deletion coverage. Broader Archive and collection systems remain.
5. Add known-input crafting and item provenance.
6. Add fixed-price Bazaar listings only after immutable ledger, escrow, idempotency and double-spend tests pass.
7. Add guild projects, community events and creator tooling only after guild isolation and moderation workflows are complete.
8. Expand content through validated vertical slices; do not attempt to implement all atlas proposals at once.

### World Loom exit criteria

- Every implemented node has a stable ID, owning domain, documented input/output contract and typed graph relationships.
- Every cross-system edge is backed by a real service contract or explicitly labelled as a future proposal.
- Rust remains the sole authority for state, outcomes, economy and permissions; Discord-native messages are the presentation layer.
- PostgreSQL migration, session-recovery, privacy export/deletion and guild-isolation tests cover every new persisted record.
- Bazaar transactions are atomic and deterministic; no stake, wager, paid chance or cash-value mechanism is introduced.
- All new game sessions have visible rules, clear stop/pause controls, accessibility fallbacks and duplicate-action protection.
- Documentation distinguishes proposed, in-development and implemented status at all times.


## Expansion II — world depth and operating systems

**Status: Product design proposal only; not implemented.**

The [World Loom](atlas/README.md) contains 400 proposals across 40 connected domains. The second layer adds ecological context, transport, settlement services, optional professions, creature habitats, evidence-led research, mystery networks, authored character continuity, construction, restoration, civic governance, content operations, safe declarative extensions, Discord-native interaction continuity, economy stewardship, security, privacy-preserving insights, localisation, onboarding and advanced accessibility.

These systems must not become independent microservices. Shared contracts remain central: stable content IDs, typed domain events, narrow Rust application services, PostgreSQL transactions, authenticated Discord identity, privacy-scoped view models and one versioned content pipeline.

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


## Current implementation increment — player foundation, Tavern and Atlas starter map

The current branch has completed the initial player-foundation and Wayfarer progression vertical slices. The next planned runtime slice is Expedition Foundation v1; its scope and acceptance gates are specified in [EXPEDITION_FOUNDATION.md](EXPEDITION_FOUNDATION.md).

The implementation combines four transport-independent Rust game rules, a PostgreSQL-backed session runtime, player profiles/preferences, atomic completed-game results, per-game statistics, five idempotent achievements, an earned-only Moonbits wallet/ledger and private data controls. The Tavern Wayfinder routes to eleven live destinations: Common Tables, player profile, game record, earned-only ledger, Achievement Gallery, Atlas, private Chronicle, Quest Board, Wayfarer Thread, Companion Journal and Rivalry Hall.

The Atlas slice is a five-location, reciprocal route graph with a persistent current location and per-player discovery/visit history. The `.atlas` and `.travel` commands are private in guild contexts; travel is restricted to direct connected routes and produces no repeatable reward. Atlas, Chronicle and Wayfarer data are part of the current privacy inventory, version 8 export and confirmed deletion flow. Versioned consent records are included in the same export and deletion flow.

This increment does **not** mark the full player platform or World Loom complete. Bounded Expeditions and authored story-quest routes, inventory, crafting, Vaults, Bazaar, community maps, guild rooms, more games, production recovery operations remain follow-up gates. The optional daily Wayfarer quest board is already implemented.


## Completed increment — Wayfarer progression slice

**Status: Implemented on PR #3.** The code head `895f47f6c2bf17a685d1d32840a7c1ef1604e93e` passed GitHub Actions CI run #418, including repository validation, formatting, PostgreSQL-backed workspace tests and Clippy. A subsequent documentation-only commit must pass its own PR checks before merge.

- [x] Daily board assigns three stable objectives per player/day from a fixed catalogue.
- [x] PostgreSQL UUID seed plus domain-separated SHA-512 selection with unbiased index mapping.
- [x] Game completions and first-time Atlas discoveries update active objectives in their source transaction.
- [x] Fixed quest rewards use the existing idempotent Moonbits ledger.
- [x] Cumulative activity days unlock cosmetic companion stages; missed days never subtract progress.
- [x] Private rival invitations require mutual acceptance; only later completed two-player games count.
- [x] Export, inventory and confirmed deletion cover every Wayfarer/rivalry table.
- [ ] Matchmaking/rematch invitations, companion collection, expedition content and richer Discord-native game presentation remains a future milestone.


## Next major implementation slice — Expedition Foundation v1

**Status: Planned next; no Expedition runtime, tables or commands are implemented yet.**
**Specification:** [Expedition Foundation v1](EXPEDITION_FOUNDATION.md).

The next slice should turn the existing five-location Atlas into one bounded, replayable journey without prematurely implementing the full Atlas/Quest/Vault/Bazaar catalogue. The first authored route is **The Lantern Road**:

`Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory`.

The route uses only existing reciprocal Atlas edges and authored Chronicle content. It is versioned and deterministic. Solo play is supported; a shared run supports one to four consenting players in one Discord guild. A player must be at the route's starting location before joining. The run records its own progress, while each participant's Atlas/Chronicle effects are applied through their owning services.

### Delivery sequence

1. **Definition and state machine:** introduce a stable expedition ID and content version; define recruiting, active, paused, completed and cancelled states; specify participant/host transitions and optimistic row-version checks.
2. **PostgreSQL persistence:** add the next ordered migration for runs, participants and checkpoint history. Add foreign keys, unique constraints and indexes for one active run per player, one checkpoint completion per run/version/checkpoint, guild scope and safe cleanup.
3. **Transactional application service:** validate membership, route adjacency, participant scope, run version and checkpoint order. Commit each checkpoint once. Reuse Atlas, Chronicle and Wayfarer domain operations through their service boundaries; do not write directly to another domain's tables.
4. **Discord interaction layer:** add `.expedition list|start|join|status|advance|pause|resume|leave` prefix and slash surfaces. Explain the route, group scope and personal Atlas effects before joining. Keep responses private where they reveal personal progress.
5. **Privacy and lifecycle:** add export/inventory coverage and confirmed deletion for Expedition-owned data. Removing one participant must not erase another participant's history; transfer host ownership to the earliest remaining participant or pause safely.
6. **Reliability and accessibility:** test restart recovery, duplicate and concurrent advances, cancellation, participant departure, host departure, permission changes, Discord delivery failure, text-only fallback and reduced-motion behavior.

### Release acceptance gate

- [ ] One authored route is fully traversable; every route edge exists in the current Atlas graph.
- [ ] Solo runs and one-to-four-player guild runs use the same authoritative Rust state machine.
- [ ] Players explicitly consent to join; no one is added to a run or moved on the Atlas without confirmation.
- [ ] Start, advance, checkpoint, pause, resume, leave and completion are durable and idempotent.
- [ ] Checkpoint writes and each participant's Atlas discovery/Chronicle unlock are atomic or have a documented retry-safe hand-off.
- [ ] Repeated commands, concurrent requests and expired Discord interactions cannot duplicate progress or effects.
- [ ] A run can be paused indefinitely and resumed later. Missing a day, leaving a run or taking a break never removes progress or rewards.
- [ ] Export, inventory, deletion, guild isolation and log-redaction tests cover all new records.
- [ ] The route is usable without colour, animation or a browser Activity; Discord-native controls remain functional.
- [ ] README, architecture, command reference, privacy policy, final vision, Atlas and roadmap all distinguish shipped behavior from the planned Expedition slice.

### Explicitly out of scope for v1

No inventory, gathering, crafting, Vaults, Bazaar listings, barter, trade escrow, new currency rewards, random loot, public matchmaking, public leaderboards, creator-authored routes, dynamic world events or Discord Activity game client. No attendance deadlines, punitive streaks, decay, FOMO, forced notifications or gameplay advantage for spending. These can be considered only after the core Expedition contract is proven.

