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

## 7. Progression and economy

Progression should recognise participation and mastery through experience, achievements, cosmetic collection, profile customisation, optional rotating challenges, and server leaderboards where administrators enable them.

The economy must remain non-wagering. No real-money gambling, betting, staking, paid random rolls, or pay-to-win mechanics. Do not let players risk earned currency on uncertain outcomes. Prefer deterministic shops and transparent, earn-only rewards.

The existing Lunar card-collection/gacha system requires a separate fairness review before migration. Any retained collection mechanic must not become a paid chance system or a way to wager value.

## 8. Discord command model

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

## 9. Server and community experience

Moonbyte should work in multiple Discord guilds without mixing server configuration. Planned server features include enabled game categories, channel preferences, guild-scoped leaderboards, optional server challenges, independent sessions, moderation controls, and reasonable per-user/per-guild rate limits.

A user's core Moonbyte identity can remain consistent across servers, while guild-specific settings and rankings stay scoped to the relevant guild.

## 10. Privacy and trust

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

## 11. What Moonbyte is not

- Not a replacement for Lunar's anime/manga or general community features.
- Not a wrapper that calls Lunar at runtime.
- Not a frame-based real-time game engine disguised as a Discord bot.
- Not a monochrome console clone or copy of protected Nintendo branding.
- Not a betting, wagering, or paid chance platform.
- Not designed to make players unable to stop.
- Not a promise to ship every game idea in the catalogue.
- Not a reason to add distributed infrastructure before workload requires it.

## 12. First-release definition

A first public release is ready when the bot connects, recovers, and shuts down cleanly; prefix and slash commands work; a curated set of solo and multiplayer games completes reliably; sessions enforce membership and state transitions; profiles persist in SQLite; pixel-art UI is readable; privacy controls accurately describe data, export it, and delete it; retention and log redaction are tested; and documentation matches production behavior.
