# Moonbyte — Final Product Vision and World Loom Constitution

> **Version 4.0 · Authoritative product blueprint · 3 November 2026**
>
> This document consolidates the uploaded Final Vision v3.1, the ten World Loom design volumes, the live Rust/PostgreSQL foundation and the cross-system contracts needed to make Moonbyte one coherent product. It supersedes earlier vision drafts where they conflict. The ten volumes remain the detailed proposal catalogue; code, tests and the roadmap remain the source of truth for what is actually live.

## 1. The product promise

Moonbyte is a colourful, original game world inside Discord: quick games, thoughtful puzzles, shared adventures and a persistent world that remembers what players choose to do.

The product should feel like a connected handheld arcade and social adventure, not a command directory and not a feed designed to keep people scrolling. Players can enter through a game, the Tavern, an Atlas destination, a party, a quest or a friend, then see how that action connects to the rest of their world.

**Create together. Discover the world. Master games. Remember the journey.**

The design is expansive, but release is incremental. Four hundred proposals across forty domains are a map of possibilities, not a promise to ship every item. Seven integration contracts make the relationships explicit. A feature becomes live only after ownership, persistence, privacy, safety, accessibility and test requirements are met.

## 2. Non-negotiable foundations

### Technology and authority

- **Application and game rules:** Rust, Tokio, Serenity and Poise.
- **Persistence:** PostgreSQL through SQLx. PostgreSQL is the only accepted application database. SQLite and ScyllaDB are excluded; do not introduce Redis, a message broker, a separate service or distributed storage without measured need and a new accepted architecture decision.
- **Visual library:** Next.js, React and TypeScript. React is used for visual surfaces and the official Discord Embedded App SDK Activity, not injected into ordinary Discord embeds.
- **Authority boundary:** Rust owns state transitions, rules, randomness, permissions, progression, inventory, ledgers, trades, fairness transcripts and persistence. React receives authorised view models and submits actions for server-side validation.
- **Stable Rust:** CI follows the latest stable toolchain and records the compiler version used for each run. The current verified CI toolchain is Rust 1.99.1.
- **No Lunar runtime dependency:** Moonbyte owns its own source, game rules, storage, identity, commands, deployment and release lifecycle. Every reused concept or asset must pass source, dependency and licence review.

### Fairness and economy

- Moonbits are earned-only, have no real-world value and cannot be bought, withdrawn, transferred outside Moonbyte, staked or wagered.
- No gambling, betting, wagering, pooled stakes, paid random rolls, paid loot boxes, cash-out or chance-based tradable rewards.
- Casual random games are score-only. Randomness that affects a verifiable result uses OS-backed cryptographic entropy, the documented SHA-512 commit–reveal protocol and unbiased mapping. SHA-512 is a hash, not an entropy source.
- Crafting outputs and published rewards are deterministic. Vaults reveal exact rewards before entry. Bazaar trades use fixed prices or explicit barter, escrow and atomic settlement.
- Items carry stable IDs, provenance and eligibility. Discovery lore and account-bound progress are not market commodities. Only explicitly marked flavour duplicates may ever be tradable; they confer no power or progression advantage.

### Privacy, safety and healthy engagement

- Collect only fields necessary for the enabled feature and document their purpose, visibility, retention, export and deletion path.
- Separate personal, party, guild and server scopes. Guild configuration never grants access to another player's global profile or private history.
- Never retain raw command or message content for analytics. Do not log secrets, private payloads or unrevealed fairness seeds.
- Daily objectives are optional. Journey progress counts up and never resets after a missed day. Seasonal rewards return on a published reprise schedule.
- No artificial scarcity, punitive streak loss, nagging, deceptive countdown, forced presence, infinite reward loop or design intended to make stopping difficult.
- Leave, pause, mute, decline and deletion are real choices. A player can stop an activity without losing unrelated progress.
- Use original full-colour pixel art. Do not copy protected console branding, sprites, characters or third-party game art. All public content is rights-cleared, moderated and age-appropriate.

## 3. One world, one interconnection model

The **Wayfarer Thread** is the shared progression and continuity contract across every domain. It is not a new service or message broker. It is a set of typed Rust domain events, stable references and read models that let independently owned systems exchange verified outcomes.

The thread has four jobs:

1. **Identity continuity:** one Moonbyte profile and stable player-owned history, with explicitly scoped guild and party context.
2. **Progress continuity:** verified game, discovery, quest, expedition and crafting outcomes can update the appropriate profile, Chronicle, mastery, companion, collection or guild projection exactly once.
3. **Navigation continuity:** each system can point to the next relevant system without silently starting an activity or changing player state.
4. **Privacy continuity:** every projection inherits the source record's visibility and deletion rules; sharing is never broader than consent and the owning guild's permissions.

### 3.1 Domain event envelope

An event is emitted only after its owning state transition commits. It has a stable event ID, event name, schema version, source aggregate ID, server timestamp, idempotency key and the minimum player/guild scope required by its consumer. Payloads contain stable IDs and typed outcomes, not raw Discord messages, arbitrary user text or secrets.

Consumers call their own application service and validate source, ownership and scope. Each consumer records or derives an idempotency key so retries cannot duplicate rewards, quest progress, archive entries, trades, mastery or notifications.

The first implementation uses in-process Rust application services and PostgreSQL transactions. Do not add a broker or distributed event platform. A transactional outbox is a future option only if measured multi-process delivery needs justify it and a separate architecture decision approves it.

### 3.2 Seven cross-cutting integration contracts

| Contract | Responsibility | Primary connections |
|---|---|---|
| **Wayfarer Thread** | Shared vocabulary for player activity, verified milestones and cross-domain references. | Games, Atlas, Expeditions, Quests, Mastery, Companions, Seasons and Guilds. |
| **Continuity Moments** | Small, one-time handoffs after a committed event; show what changed and the next relevant action. | Atlas → Chronicle; Expedition → Quest/Archive; Craft → Want Board; Rivalry → Rematch; Guild → Guildhall. |
| **Unified Player Card** | One authorised profile projection for identity, mastery, journey days, companion, Chronicle, rivals and permitted guild contributions. | Profile, Mastery, Archive, Character, Fellowship, Guild and Trust. |
| **Rival and Nemesis** | Honest, mutual-consent competition with private records and cosmetic recognition only. | Multiplayer, Fairness, Fellowship, Character, Hearth and Chronicle. |
| **Non-punitive Journey** | Cumulative active days and permanent milestone recognition; no reset or loss for absence. | Wayfarer, Quests, Mastery, Companion, Festivals and Profile. |
| **Seasonal Almanac** | Published event schedules, permanent catalogues and predictable reprise windows. | Festivals, Bazaar, Craft, Guild, Archive and Trust. |
| **Mastery Access** | Clear skill thresholds unlock optional difficulty tiers and variants without blocking the base game or main story. | Arcade, Atlas, Expeditions, Vaults, Fairness and Profile. |

## 4. Continuity Moments — useful ripples, not notification spam

A continuity moment is a small, contextual handoff shown after a successful action. It is not a second reward, background notification or instruction to keep playing.

| Committed event | Visible continuity moment | Systems updated |
|---|---|---|
| First-time Atlas discovery | The private lore page appears in the Chronicle; a guild discovery note appears only when shared discovery is enabled. | Atlas, Chronicle, Wayfarer, optional Guild Chronicle. |
| Expedition checkpoint | The checkpoint and route note are saved; the player can pause, continue or return to the Tavern. | Expedition, Atlas, Chronicle, Wayfarer. |
| Expedition completion | The run summary appears; an accepted daily Expedition objective progresses once. | Expedition, Wayfarer, Quests, Mastery and Archive. |
| Mastery rank change | The player sees the exact optional tiers or Realm variants now available, with a route link. | Mastery, Arcade, Atlas and Profile. |
| Craft completion | The item and provenance appear; a matching open Want Board request may be offered as one optional next action. | Craft, Inventory, Bazaar and Archive. |
| Bazaar fulfilment | Buyer and seller receive a private receipt; the Want Board and permitted reputation projection update. | Bazaar, Inventory, Economy and Profile. |
| Companion milestone | The cosmetic stage and eligible accessory appear together, ready to equip. | Wayfarer, Character, Inventory and Profile. |
| Guild milestone | A scoped milestone record and decoration unlock appear; contributor recognition follows published guild visibility rules. | Guild, Hearth, Archive, Homestead and Profile. |
| Rival rematch result | Both private records update; a rematch action is offered only to the participants. | Rivalry, Multiplayer, Fairness and Chronicle. |
| Seasonal set earned | The permanent catalogue marks the set as collected and displays its next published reprise window. | Festival, Archive, Profile and Bazaar eligibility. |

A handoff may be dismissed. It never silently joins a party, starts an expedition, lists an item, accepts a rivalry, posts to a server or sends an unsolicited DM. Quiet hours and notification preferences apply.

## 5. The Unified Player Card

The profile becomes the player's settings and progress home, with a private-first summary and clear shortcuts. It is a read model over owning systems, not a second place to store or mutate their private tables.

The personal view can show display name, optional bio, visibility and preferences; completed games, personal bests, achievements and Mastery ranks; cumulative Wayfarer journey days and the next non-resetting milestone; companion stage and equipped cosmetic; Chronicle and collection progress; active Rivalries and private head-to-head records; seasonal sets and the published Almanac; guild contributions scoped to each guild and shared only when enabled; and shortcuts to Games, Atlas, Chronicle, Quests, Expeditions, Companion and settings.

A public or guild-facing card is a separately authorised projection. It honours profile visibility, show-statistics settings, guild scope and each feature's own opt-in. Never expose another player's private rivalry, location, quest state or export through a public profile. If a section is disabled or unavailable, explain that boundary rather than inventing data.

## 6. Rivals and the Nemesis

Rivalry means genuine competition between consenting players. It is never a wager and never changes normal game rules.

### Eligibility and consent

- Three or more completed matches against the same opponent can qualify a pair for a quiet eligibility suggestion.
- Eligibility is not a declaration, notification or public label. No player is automatically enrolled. Suggestions are disabled unless the player permits them.
- Either participant may send a formal invitation. A rivalry becomes active only after the other participant accepts.
- Declining, ignoring, blocking or leaving has no effect on currency, mastery, journey, access or unrelated history.

### Active rivalry

- The Rivalry Log records honest completed-match totals and timestamps for both participants.
- A Grudge Match is an ordinary session on the same validated game engine, rules version, turn ownership and fairness protocol as every other match.
- Grudge Trophies are optional cosmetic indicators of the current head-to-head lead. They can change hands and have no economic value, power or permanent penalty.
- A rematch is an optional request. It does not start automatically and cannot bypass the opponent's consent.
- Rivalry records are private by default. Server display requires both participants' explicit sharing choice and a guild permission check.

### Nemesis NPC

The Nemesis is a skippable flavour character in Emberveil Wastes. Dialogue can reference a rivalry only when the participant has enabled that feature. It never mocks, shames or targets a player, and it has no mechanical effect, reward modifier or loss memory by default.

## 7. Mastery, Journey and Companion

### Mastery

Mastery is earned through clear, reproducible criteria: completed games, demonstrated skill, personal bests, puzzle solutions and authored milestones. Each rank explains its threshold and what it unlocks.

Ranks may open optional Realm variants, harder Waypoints, Mastery Vaults and advanced challenge sets. They never gate the base catalogue, privacy controls, the central story or a player's ability to join friends. Competitive leaderboards compare equivalent rules and timing profiles.

### The Wayfarer Journey

The current implementation records unique active calendar days and optional daily objectives. The final design extends that same non-punitive model across eligible systems.

- At most one journey-day record per player per calendar day, regardless of how many systems they touch.
- A missed day does not subtract a day, erase a milestone or reduce a companion stage.
- Qualifying actions are meaningful completed activity, not command spam, idle presence or repeatable trivial clicks.
- Daily objectives are optional, fixed-reward and disclosed before acceptance. Claiming is idempotent.
- A player can inspect the source of each journey milestone and choose whether to share it.

### Companion Sprite

The Companion is original, cosmetic-first and expressive. It can appear in the Tavern, private profile and an expedition party view when the player allows it. Journey and story milestones unlock permanent stages and accessories. Companions do not create combat advantage, production boosts or pay-to-win effects.

## 8. Seasons and the Moonlit Tideway

Seasonal content is a publishing and celebration layer, not a scarcity engine.

- A typical event chapter lasts around three to four weeks; the exact calendar is published before it starts.
- Seasonal sets, decorations and story entries are recorded permanently in the Almanac.
- Each set has a published reprise window or an evergreen route for players who missed the original event.
- Tide Tokens, where used, are account-bound event progress marks with no cash value. They cannot be bought, transferred, staked, wagered or sold.
- Seasonal challenges are optional and have known outcomes. Missing an event never erases progress from another system.
- Seasonal content may add safe weather, art, NPC dialogue, optional routes and community projects, but it cannot remove a core location or lock the main story.

## 9. How all ten volumes connect

The ten World Loom volumes remain the detailed source catalogue for forty domains and four hundred original ideas. Each volume begins with a Wayfarer Thread handoff contract. The graph adds seven integration-contract nodes without changing the original count of 400 ideas or 40 domains.

| Volume | Domains | Main interconnection chain |
|---|---|---|
| I — Hearth, Bazaar and Communities | Hearth, Bazaar, Guild, Server | Discover → gather → trade or cooperate → record a scoped community milestone. |
| II — Atlas, Expeditions and Story | Atlas, Expedition, Quest, Vault | Discover route → prepare → checkpoint → resolve authored objective → preserve lore. |
| III — Crafting, Collections and Arcade | Craft, Archive, Arcade, Multiplayer | Play or explore → earn known inputs → craft → catalogue → share or trade eligible items. |
| IV — Characters, Mastery and Homesteads | Character, Mastery, Festival, Homestead | Demonstrate skill → unlock a cosmetic or optional challenge → display or celebrate it. |
| V — Creation, Competition and Trust | Creator, Fairness, Fellowship, Trust | Create → validate → review → publish; or invite → consent → play a fair rivalry. |
| VI — Ecology, Transit, Settlements and Professions | Climate, Transit, Settlement, Profession | Observe a region → plan a route → gather supplies → build or serve a settlement. |
| VII — Creatures, Knowledge, Mysteries and Narrative | Creature, Knowledge, Mystery, Narrative | Discover evidence → research → collaborate on a case → unlock a reviewed story branch. |
| VIII — Building, Conservation, Governance and Live Operations | Building, Conservation, Governance, LiveOps | Propose → review → fund with known resources → build → audit → preserve history. |
| IX — Extensions, Platform, Economy and Security | Extensions, Platform, Economy Governance, Security | Author a capability → review permissions → publish a safe interface → audit every transaction. |
| X — Insights, Localisation, Onboarding and Accessibility | Insights, Localisation, Onboarding, Accessibility Plus | Explain the world → let players choose controls/language → measure reliability with minimised aggregates. |

## 10. The connected player journeys

- **Quick play:** Tavern → Common Tables → completed game → first-completion ledger milestone → achievement/quest progress → private Player Card → optional next game or exit.
- **Explorer:** Tavern → Atlas → directly connected route → first discovery → private Chronicle page → Expedition eligibility → checkpoint → optional Vault → permanent Archive entry.
- **Maker:** Expedition or quest → known materials → Recipe Codex → predictable craft → personal collection → optional Want Board fulfilment → private trade receipt.
- **Guild organiser:** Tavern → guild-scoped project → optional party/Expedition → verified contributions → Guild Chronicle → public decoration only under guild settings → member-controlled profile recognition.
- **Competitive player:** Normal match → quiet eligibility after repeated meetings → mutual rivalry invitation → ordinary fair Grudge Match → private record update → optional rematch.
- **Returning player:** Open Tavern → optional welcome-back recap → resume a checkpoint or choose a new activity. No guilt language, lost journey count, expiring core reward or forced catch-up.

## 11. Scope and truth of implementation

The current repository contains working Rust/PostgreSQL vertical slices for the runtime, four games, sessions, profiles, game results/statistics, achievements, first-completion Moonbits, guild command configuration, versioned policy consent, data inventory/export/deletion, Tavern/Wayfinder, the five-location Atlas, private Chronicle pages, optional daily quests, cumulative journey days, companion milestones and mutual private rivalries.

The rest of the World Loom remains a design proposal until code, migrations, commands, tests, documentation and privacy coverage are merged. In particular: larger Atlas regions, multi-stage Expeditions, authored story networks, Vaults, inventory, crafting, homesteads, Bazaar trading, guild projects, seasonal releases, expanded mastery, the full Rival/Nemesis presentation and the unified Player Card are not made live by this document.

## 12. Release order and acceptance gates

1. **[Expedition foundation](EXPEDITION_FOUNDATION.md):** one authored, bounded route; durable checkpoint state; pause/resume/leave; Atlas and Chronicle handoff; optional Wayfarer activity projection; export/deletion coverage.
2. **Inventory and provenance:** stable item IDs, earned source and account ownership before crafting or trade.
3. **Known-input crafting:** fixed recipes, validated ingredients, transactional output, no random yield.
4. **Puzzle Vault:** clear rules, published rewards, save checkpoints and verifiable completion.
5. **Cooperative party and Guild projects:** explicit invitations, roles, contribution scope and guild isolation.
6. **Bazaar:** fixed-price listing and explicit barter only after escrow, ledger, idempotency, anti-farming and concurrency tests.
7. **Mastery and profile projection:** clear rank thresholds, authorised unified Player Card and optional Realm variants.
8. **Seasonal Almanac and continuity polish:** published calendar, reprises, player-controlled handoffs and content archive.
9. **Discord Activity and production readiness:** official SDK, mobile/accessibility, recovery, backup/restore, rate-limit and privacy drills.

A slice is complete only when its owning Rust module controls every state transition; PostgreSQL constraints prevent duplicate or partial writes; Discord and Activity surfaces share the same application service; events have stable schemas and idempotency rules; player and guild data are scoped with complete privacy coverage; tests cover normal, invalid, duplicate, concurrent, rollback, pause/leave and recovery paths; and README, roadmap, volumes, graph and decision records agree with the code.

**The central rule:** a new system is not finished when it has a command. It is finished when its state, consequences, privacy, presentation and relationships work together as one part of Moonbyte.

## Current next-slice reference

The first delivery boundary for the Expedition foundation is defined in [Expedition Foundation v1](EXPEDITION_FOUNDATION.md). It narrows the much larger Atlas/Expedition/Quest proposals to one authored route and explicit release gates; no Expedition behavior should be considered live until its Rust service, migration, commands, privacy coverage and tests are merged.
