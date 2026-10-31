# Moonbyte World Loom — Master Atlas

- Status: **Expanded product design proposal; not implemented.**
- Scope: **400 individual system ideas across 40 connected domains.**
- Documented idea volume: **48,295 lines** across the ten design volumes (before this index, mind map and machine-readable graph).
- System graph: [world_graph.json](./world_graph.json) — canonical node IDs, typed edges, rationales and shared invariants.
- Mermaid diagrams: [world_map.mmd](./world_map.mmd) — complete idea hierarchy and cross-domain map.

## One world, one graph

Moonbyte is designed as one connected world with several player-facing entrances. The Tavern is the social home and discovery layer. The Atlas and Expeditions make the world explorable. Quests and Vaults provide authored goals. Crafting converts known resources into useful objects. Collections preserve discoveries. The Bazaar circulates eligible earned items through fixed-price trades and transparent barter. Guilds and parties coordinate shared play. The Arcade supplies short sessions and skill practice. Character, mastery, homes and festivals turn the results into personal and community identity.

The graph is not a collection of unrelated command ideas. It is a contract map: each feature has a stable identifier, one owning domain, typed incoming/outgoing connections, an explicit reason for every cross-system relationship and a consistent implementation boundary.

## Ten design volumes

- [Hearth, Bazaar and Communities](./01_hearth_bazaar_and_communities.md) — 40 system proposals.
- [Atlas, Expeditions and Story](./02_atlas_expeditions_and_story.md) — 40 system proposals.
- [Crafting, Collections and Arcade](./03_crafting_collections_and_arcade.md) — 40 system proposals.
- [Characters, Mastery and Homesteads](./04_characters_mastery_and_homesteads.md) — 40 system proposals.
- [Creation, Competition and Trust](./05_creation_competition_and_trust.md) — 40 system proposals.
- [Ecology, Transit, Settlements and Professions](./06_ecology_transit_settlements_professions.md) — 40 additional proposals.
- [Creatures, Knowledge, Mysteries and Narrative](./07_creatures_knowledge_mysteries_narrative.md) — 40 additional proposals.
- [Building, Conservation, Governance and Live Operations](./08_building_conservation_governance_liveops.md) — 40 additional proposals.
- [Extensions, Platform, Economy and Security](./09_extensions_platform_economy_security.md) — 40 additional proposals.
- [Insights, Localisation, Onboarding and Accessibility](./10_insights_localisation_onboarding_accessibility.md) — 40 additional proposals.

## Domain index

1. **The Hearth and Tavern** (`hearth`) — The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
2. **The Bazaar and Exchange** (`bazaar`) — The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
3. **Guilds and Long-Form Cooperation** (`guild`) — Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
4. **Server Culture and Community Layer** (`server`) — Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
5. **Atlas, Regions and World Discovery** (`atlas`) — The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
6. **Expeditions and Exploration Sessions** (`expedition`) — Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
7. **Quests, Mystery and Story** (`quest`) — Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
8. **Vaults, Dungeons and Puzzle Spaces** (`vault`) — Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
9. **Gathering, Crafting and Workshops** (`craft`) — Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
10. **Collections, Bestiary and Museum** (`archive`) — The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
11. **Arcade and Original Mini-Games** (`arcade`) — The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
12. **Parties, Lobbies and Multiplayer Rules** (`multiplayer`) — Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
13. **Characters, Companions and Tactical Identity** (`character`) — Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
14. **Progression, Mastery and Recognition** (`mastery`) — Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
15. **Festivals, Chapters and World Events** (`festival`) — Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
16. **Homes, Gardens and Personal Spaces** (`homestead`) — Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
17. **Player Creation and Community Content** (`creator`) — Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
18. **Fair Play, Competition and Verification** (`fairness`) — Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
19. **Friends, Community Bonds and Social Play** (`fellowship`) — Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
20. **Accessibility, Moderation, Privacy and Operations** (`trust`) — Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.

## Core relationship loops

### The everyday loop
Tavern → Wayfinder → Arcade / Notice Wall / Party → result → Mastery and Archive → return to Tavern.

### The adventure loop
Rumour Circle → Atlas → Expedition → environmental puzzle / Vault → discovery → Archive → new Atlas routes and story context.

### The crafting and trade loop
Expedition or quest → known resources → Recipe Codex → Workshop → item provenance → Bazaar listing or work order → atomic escrow → new owner or completed commission.

### The community loop
Tavern → Guild → shared project / Guild Vault → contribution record → Guild Hall / Chronicle → server celebration.

### The creative loop
Puzzle or story creator → validation → moderation → private playtest → approved pack → Arcade / Quest board → feedback and versioned revision.

## World Loom invariants

1. **One source of truth:** Rust owns game state, rules, permissions, reward eligibility, trades, fairness transcripts and persistence.
2. **One presentation family:** Discord-native commands remain first-class; Next.js + React + TypeScript add rich views through the official Embedded App SDK where appropriate.
3. **One initial database target:** SQLite with versioned migrations, short transactions and tested recovery.
4. **One honest economy:** Moonbits are earned-only and non-redeemable. No betting, staking, cash-out, paid random rewards or chance-based tradable assets.
5. **One coherent identity:** original full-colour pixel art, common content IDs, shared world locations and consistent state labels.
6. **One engagement ethic:** players return for meaningful play, social choice, mastery and new content—not loss aversion, artificial scarcity or compulsive loops.
7. **One privacy boundary:** only necessary data is stored; public, party, guild and personal scopes are distinct and documented.
8. **One release truth:** every item in this atlas remains proposed until its own implementation and tests are complete.

## Relationship semantics

The graph uses explicit relationship types such as `hosts`, `routes_to`, `feeds`, `records`, `unlocks`, `coordinates`, `secured_by`, `belongs_to` and `contains_idea`. An edge includes a human-readable reason. A feature without a relationship is an incomplete design and should not be added to the graph as an orphan.

## How to use this atlas

- Start with the master graph, then open the feature's volume for its full contract.
- Treat stable IDs as references in future code, migrations, content packs and UI routes.
- Keep feature definitions separate from Discord command names; the latter are adapters, not the domain model.
- Implement vertical slices rather than attempting all 400 proposals at once.
- Add an implementation link and tests only when the feature actually exists.
- When a node changes, update both its volume card and graph record in the same commit.
- Do not represent all concepts as approved catalogue games; this is a wider product-system map, not a release promise.

## Recommended first connected vertical slice

1. Shared session and player identity contract.
2. Tavern home with the Wayfinder and Common Tables.
3. One polished mini-game and one small cooperative puzzle.
4. SQLite progress, idempotent rewards and a privacy-complete profile slice.
5. Personal Archive and a fixed-reward Puzzle Vault.
6. Known-resource crafting with provenance.
7. Fixed-price Bazaar listings only after ledger, escrow and double-spend tests.
8. Guild projects and server-scoped controls after isolation tests.

This sequence proves the core connections before the catalogue becomes large. The breadth of this atlas is a direction map, not permission to skip quality gates.


## Expansion II — the world beyond the first loop

The second layer adds twenty domains and two hundred distinct proposals. It covers ecology and climate; transport; settlements; professions; creature habitats; research and learning; mysteries; character continuity; architecture; restoration; civic governance; content operations; declarative extensions; Activity continuity; economy stewardship; security; privacy-preserving insights; localisation; onboarding; and advanced accessibility.

These systems make the world coherent and maintainable as it grows. They define stable manifests, content dependencies, rollback paths, deterministic service contracts, privacy-scoped reports, safe event subscriptions, accessible controls and reviewed releases. They connect to the first twenty domains through explained graph links rather than creating an unrelated second game.

### New relationship loops

- **Climate and exploration:** Atlas region → climate profile → forecast clue → Expedition route → observation → restoration map.
- **Travel and exchange:** route graph → caravan → cargo manifest → delivery contract → Bazaar receipt.
- **Settlements and professions:** civic project → blueprint → profession task → crafted result → permanent town service.
- **Creatures and knowledge:** habitat → field observation → Bestiary → research dossier → sanctuary story.
- **Mysteries and narrative:** case file → evidence wall → hypothesis → character memory flag → chapter recap.
- **Creation and operations:** creator draft → validation → review → staged preview → versioned release → evergreen archive.
- **Economy and security:** reward registry → immutable ledger → escrow → receipt → reconciliation and dispute review.
- **Accessibility and platform:** control preference → Discord/Activity view model → text alternative → consistent session recovery.

The second layer inherits all first-layer invariants. It does not authorise arbitrary executable extensions, real-money markets, behavioural surveillance or additional infrastructure by default.
