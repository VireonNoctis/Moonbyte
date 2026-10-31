# Moonbyte Product Vision

- Status: Accepted product direction; implementation is not complete.
- Last updated: 2026-10-31.

## 1. One-sentence vision

**Moonbyte turns Discord into a colourful pocket arcade where people can discover, play, and share quick games, puzzles, multiplayer sessions, and longer adventures.**

Tagline: **A little world of games, right inside Discord.**

## 2. Product identity

Moonbyte is a separate gaming bot built using Lunar as its foundation. Lunar provides source material: selected game ideas, useful mechanics, and implementation lessons. Moonbyte owns its own repository, Rust application, game engine, storage, Discord identity, configuration, and roadmap.

The two projects must remain operationally independent. Moonbyte must not require Lunar to be online, require users to link a Lunar website account, or read Lunar's production database. Reuse is an audited migration of specific game behavior, not a dependency on the old bot's architecture.

The product should feel like opening an original handheld arcade in a Discord server. A lightweight console metaphor—directional controls, A/B actions, Start, Select, game cards, and playful status panels—can shape the interaction design without copying any existing console's protected branding or artwork.

## 3. The player experience

A player should be able to discover a game through a visual library, start solo play or a lobby, understand rules before play, use Discord buttons/menus/modals, leave or resume supported sessions, earn appropriate progression, and find something else to play after completing a game.

Discord is the client surface. A Moonbyte game should not assume a continuously animated canvas, frame-perfect input, or a 60-FPS connection. Fast turn-based interactions and short timed rounds are the natural fit.

## 4. Product pillars

### A. A broad, curated game library

The catalogue has five required pillars:
- Existing Lunar games and mechanics that pass migration review.
- Original mini-games designed for quick repeat play.
- Multiplayer games for friends and server communities.
- Adventure and RPG experiences with persistent progression.
- Puzzle and strategy games for thoughtful solo or group play.

A large catalogue is not a launch requirement. A small number of polished, well-tested games is better than many shallow or unreliable ones.

### B. A consistent game engine

All games should use shared lifecycle, session, player-action, persistence, rendering, and result interfaces. Each game owns its rules and state transitions. The platform owns Discord delivery, player identity, session coordination, common persistence, and cross-game progression.

### C. Reliable multiplayer

Multiplayer sessions need a creator/host and participant list, lobby capacity and invitations, turn order or simultaneous-action windows, server-side action validation, deadlines, inactivity handling, cancellation, recovery, and explicit view/join/control/end permissions.

The MVP should begin with turn-based games. Real-time action games are not a target for Discord's interaction model.

### D. A living player profile

Profiles can include display preferences, games played and completed, per-game records, achievements, collection progress, adventure checkpoints, and earned cosmetics. Store only what enabled features require. Provide export and deletion controls.

### E. A colourful visual identity

The visual direction is **original full-colour pixel art**:
- Distinctive environments, characters, icons, item sprites, and boards.
- Balanced palettes, clear foreground/background separation, crisp edges, consistent sprite proportions.
- Colour used to establish world identity and communicate game state.
- Text and shapes remain readable without relying on colour alone.
- Reusable sprite sheets and UI assets.

Do not use a monochrome-only theme. Do not copy Nintendo/Game Boy logos, device frames, proprietary sprites, characters, or third-party game art. External art must have documented rights for intended use.

## 5. Replayability without compulsion

Moonbyte should be a product players return to because it contains genuinely good games and evolving content. The goal is long-term enjoyment, not maximising minutes online or exploiting habit formation.

Healthy engagement requirements:
- Variety across short, medium, and long session types.
- Skill mastery, optional difficulty levels, unlockable challenges, and personal bests.
- Cooperative goals and friendly competition that do not require constant availability.
- Story arcs and quests that can be paused and resumed.
- Optional daily or rotating challenges; missing a day never destroys progress or resets a punitive streak.
- Predictable rewards earned through play.
- Notification opt-ins, quiet hours, frequency limits, and easy opt-out.
- Clear leave, pause, mute, and account-deletion controls.

Do not implement fake scarcity, manipulative FOMO, loss-aversion streaks, infinite reward loops, nagging notifications, paid random rewards, or mechanics designed to make stopping difficult. No reward should require a player to remain online for an unhealthy or unreasonable length of time.

## 6. Game categories

| Category | Product role | Candidate experiences |
| --- | --- | --- |
| Existing Lunar games | Preserve suitable parts of the foundation. | Akinator-style guessing, coin flip, dice, 8-ball, reviewed card collection. |
| Original mini-games | Give Moonbyte an identity. | Memory match, word scramble, sequence recall, pattern recognition, quick trivia. |
| Multiplayer | Make the bot social. | Tic-tac-toe, Connect Four, quiz rooms, co-op puzzles, turn-based party games. |
| Adventure / RPG | Provide story and persistent progress. | Exploration, quests, NPC dialogue, inventory, crafting, collections, party progression, turn-based encounters. |
| Puzzle / strategy | Deliver thoughtful challenges. | Sudoku, nonograms, logic grids, word puzzles, chess/checkers, optional daily challenges. |

Candidates are not promises that every title will ship. See [GAME_CATALOGUE.md](GAME_CATALOGUE.md).

## 7. Visual library and rich Discord experiences

Moonbyte's visual library uses **Next.js + React + TypeScript**. It provides a browsable catalogue, detailed game pages, richer lobby views, and interfaces for games that benefit from a full web surface.

The visual library is a frontend, not a second game engine:
- Rust/Tokio remains authoritative for game rules, validation, randomness, session state, progression, economy, and SQLite persistence.
- The frontend receives authorised view models and submits player actions to the backend.
- Discord-native commands continue to use messages, embeds, buttons, menus, and modals.
- When running inside Discord, the frontend can integrate the official Embedded App SDK as an Activity. An HTTPS browser fallback supports discovery and compatible views.
- React is not injected into normal Discord embeds. The Activity is a separate web app embedded within the Discord client.

## 8. Progression, Vaults and Bazaar

Progression recognises participation and mastery through experience, achievements, cosmetic collections, profile customisation, optional challenges, and administrator-enabled server leaderboards.

### Vaults

Vaults are replayable challenge spaces, not random-reward boxes. Planned types include Puzzle Vaults, Expedition Vaults, Relic Vaults, Guild Vaults, Mastery Vaults, Archive Vaults, Crafting Vaults, and optional Seasonal Vaults. Each Vault shows its difficulty, expected session length, rules, and exact rewards before entry. Rewards are fixed for the vault version and completion condition. Players can save at checkpoints and leave without losing completed progress.

A Personal Vault stores earned collectibles, discovered relics, recipes, blueprints, cosmetics, and saved loadouts. Keys, where used, are earned through visible progression and are never sold or tied to hidden outcomes.

### Bazaar

The Bazaar is a controlled player marketplace for eligible earned items. It supports player stalls, fixed-price listings, transparent barter, wanted boards, crafting orders, item provenance, escrow, trade receipts, and private trade history. Every trade is explicitly confirmed and atomically executed. Listings reserve items in escrow; cancellation and expiry return them exactly once. Self-trading, duplicate claims, double-spending, and trade-farming are blocked.

### Economy boundaries

Moonbits are earned through published activities and have no real-world value. They cannot be purchased, withdrawn, exchanged outside Moonbyte, or staked. Rewards, prices, recipes, and fees are disclosed in advance.

Moonbyte will not implement gambling, betting, wagering, stakes, cash-out, paid random rewards, or chance-based tradable rewards. Casual random games are score-only: they never risk or award currency or items. Vault rewards and Bazaar prices are deterministic.

See [ECONOMY_AND_VAULTS.md](ECONOMY_AND_VAULTS.md) for the complete system specification and [FAIRNESS.md](FAIRNESS.md) for the verification protocol. SHA-512 is a cryptographic hash, not an entropy source; verified outcomes also require OS-backed entropy and unbiased derivation.

## 9. Discord command model

Default prefix: **.**

Primary commands:
- .help — usage and command groups.
- .games — browse the library.
- .play — start a game or join a session.
- .daily — open the optional daily challenge.
- .profile — view a player profile.
- .collection and .inventory — view collections and items.
- .quests — browse adventure content.
- .party — create and manage a party.
- .leaderboard — supported rankings.
- .achievements — earned achievements.
- .privacy — privacy overview and controls.
- .privacy data — data inventory for the requesting player.
- .privacy export — request a copy of stored profile data.
- .privacy delete — confirm deletion of Moonbyte profile data.
- .privacy server — guild configuration controls for authorised admins.
- .settings — player and server preferences.
- .stop — leave or end an eligible session.

Slash-command equivalents should be available where practical. Prefix parsing uses Discord's Message Content privileged intent, which must be enabled in developer settings and requested by the bot. Request no privileged intents that the product does not need.

Public game boards are suitable for multiplayer. Personal information, profile data, and exports should be delivered privately where supported. Participants must be told whether a session is public or private.

## 10. Server and community experience

Moonbyte should work in multiple Discord guilds without mixing server configuration. Planned server features include enabled game categories, channel preferences, guild-scoped leaderboards, optional server challenges, independent sessions, moderation controls, and reasonable per-user/per-guild rate limits.

A user's core Moonbyte identity can remain consistent across servers, while guild-specific settings and rankings stay scoped to the relevant guild.

## 11. Privacy and trust

Privacy is a core product feature. The complete intended policy is in [PRIVACY.md](PRIVACY.md).

Core stance:
- Collect the minimum data required for enabled game features.
- Explain the purpose and retention of every stored category.
- Do not sell or rent personal data, or share it for advertising.
- No advertising trackers, behavioural profiling, third-party analytics, or AI training on player messages.
- Do not retain raw prefix messages or command content as analytics.
- Provide an in-bot privacy command for overview, data inventory, export, and deletion.
- Separate user profile data from guild-specific configuration and server-only rankings.
- Restrict access to the database, logs, backups, and deployment credentials.
- Be honest about encryption: Discord and the bot must process interactions; the bot does not provide end-to-end encryption for stored game data.

The policy is a target commitment and must be implemented and verified before production release. Documentation must change if actual behavior differs.

## 12. What Moonbyte is not

- Not a replacement for Lunar's anime/manga or general community features.
- Not a wrapper that calls Lunar at runtime.
- Not a frame-based real-time game engine disguised as a Discord bot. Rich game views use a separate Next.js/React Activity-capable frontend.
- Not a monochrome console clone or copy of protected Nintendo branding.
- Not a betting, wagering, or paid chance platform.
- Not designed to make players unable to stop.
- Not a promise to ship every game idea in the catalogue.
- Not a reason to add distributed infrastructure before workload requires it.

## 13. First-release definition

A first public release is ready when the bot connects, recovers, and shuts down cleanly; prefix and slash commands work; a curated set of solo and multiplayer games completes reliably; sessions enforce membership and state transitions; profiles persist in SQLite; pixel-art UI is readable; privacy controls accurately describe data, export it, and delete it; retention and log redaction are tested; and documentation matches production behavior.


## 14. The connected world: Moonbyte World Loom

The long-range product design connects the Tavern, Bazaar, Atlas, Expeditions, Quests, Vaults, crafting, collections, Arcade, guilds, parties, characters, mastery, homes, festivals and community creation into one world graph.

The Tavern is the social home and discovery surface. The Atlas gives every region and landmark a stable identity. Expeditions turn routes into bounded sessions. Quests and Vaults supply authored goals. Gathering and crafting create known, provenance-bearing outputs. The Bazaar circulates eligible earned items through disclosed fixed-price trades, barter and escrow. The Archive preserves discoveries. Guilds and parties coordinate group work. Arcade games provide short sessions and mastery practice. Homes, characters and festivals express the results.

This is specified in the [Moonbyte World Loom master atlas](atlas/README.md): 200 individual proposals across 20 domains, five detailed volumes, a Mermaid map and a machine-readable graph with typed edges and an explanation for every cross-system relationship. The atlas is a product design proposal, not an implementation claim or a promise to ship every idea.

The graph has one governing rule: systems exchange stable domain events and references rather than directly manipulating one another's private state. Rust remains authoritative for rules, sessions, permissions, progression, economy, fairness and SQLite persistence. Next.js + React + TypeScript remain presentation-only. The first release should prove a small connected vertical slice before expanding the catalogue.

The expanded world remains subject to the existing boundaries: earned-only non-redeemable currency, no gambling or paid chance, transparent fixed rewards, original or rights-cleared content, privacy by design, accessible text fallbacks, and replayability without manipulative engagement mechanics.


## 15. Expansion II — world depth, operations and continuity

The second design layer covers the capabilities that support a deep world over time: regional ecology and climate; travel and logistics; settlements; professions; creature habitats and sanctuaries; libraries and evidence-led research; community mysteries; narrative character continuity; construction; restoration; fictional civic governance; content operations; declarative extensions; Activity session continuity; economy governance; security; privacy-preserving insights; localisation; onboarding; and advanced accessibility.

It adds 200 proposals across 20 further domains, all connected to the first graph. Its purpose is to make the product coherent and maintainable at scale, not to create disconnected mechanics. Stable content IDs, service ownership, dependency validation, privacy-scoped view models and reversible releases keep the world consistent.

The [Moonbyte World Loom master atlas](atlas/README.md) documents both layers: 400 proposals across 40 domains and 48,295 lines of feature specifications. All proposals remain design work until implemented and tested. Rust owns authoritative rules and persistence, Next.js/React/TypeScript present authorised state, SQLite remains the initial database, and the game remains earned-only, non-wagering, privacy-conscious and playable without attendance pressure.
