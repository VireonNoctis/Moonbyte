# Moonbyte Implementation Map

> **Status source of truth · 4 November 2026**
>
> The Final Vision describes the intended product. This document describes what this branch actually implements. The Roadmap records what comes next.

## Runtime at a glance

| Area | Current implementation |
|---|---|
| Language/runtime | Rust 2024, Tokio |
| Discord | Serenity + Poise; shared prefix and slash handlers |
| Prefix | `.` by default; per-guild override supported |
| Storage | PostgreSQL through SQLx |
| Deployment contract | One supported bot process with PostgreSQL |
| Schema | Ordered migrations through `0011_expeditions.sql` |
| Player interface | Discord-native commands/messages; numbered buttons for Tic-tac-toe, Connect Four and Memory Match |
| Toolchain | Pinned in `rust-toolchain.toml` |

## Live systems

### Games and sessions

Four games currently run through the same transport-independent game engine:

- Tic-tac-toe — two-player guild lobby, legal moves, turn order, wins and draws.
- Connect Four — two-player guild lobby, gravity-based placement, legal columns, wins and draws.
- Memory Match — solo, eight fixed positions and four pairs.
- Word Scramble — solo, one curated starter puzzle and bounded attempts.

Sessions are stored as versioned PostgreSQL JSONB snapshots. Mutations lock one session row, validate state and membership, then commit before Discord delivery. Sessions survive runtime reconstruction. Waiting/active sessions expire after 24 hours without a successful mutation and are cleaned by one periodic task. Reads do not extend the inactivity window. Discord button/select controls and duplicate-interaction idempotency are not yet implemented; commands remain the reliable action path.

### Tavern and world

The typed Tavern registry contains twelve live destinations:

1. Common Tables — `.games`
2. Your Place — `.profile`
3. Your Record — `.stats`
4. The Ledger — `.wallet`
5. The Gallery — `.achievements`
6. The Atlas — `.atlas`
7. The Chronicle — `.chronicle`
8. The Quest Board — `.quests`
9. The Wayfarer Thread — `.wayfarer`
10. The Companion Journal — `.companion`
11. The Rivalry Hall — `.rivals`
12. The Expedition Board — `.expedition`

The Atlas contains five authored locations and six reciprocal route edges. A player begins at Lantern Square and can travel only to a directly connected location. First discovery unlocks one private Chronicle fragment in the same transaction; revisits do not duplicate pages or grant repeatable rewards.

The Lantern Road is implemented as one bounded, authored route: Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory. Solo runs and same-guild groups of up to four are supported. Every participant opts in. The state machine supports recruiting, active, paused, completed and cancelled states; checkpoints are versioned and persisted, participant-specific Atlas/Chronicle writes use their owning services, and export/deletion includes the participant's own history. Runs can remain paused; there is no attendance deadline.

### Wayfarer and social progress

- Three optional daily objectives are selected from a fixed five-objective catalogue.
- Quest acceptance is explicit; rewards are fixed, disclosed and claimed through the immutable wallet ledger.
- Rotation uses a PostgreSQL UUID seed and domain-separated SHA-512 derivation with unbiased selection. The private seed is not exported.
- Journey days are cumulative and never reset after absence.
- Companion milestones are cosmetic and do not change game rules.
- Rivalries are private and become active only through mutual acceptance. Only later completed two-player sessions count.

### Profiles, consent and privacy

- Profiles store a player-selected display name, optional short bio, visibility and preference flags.
- The global command gate requires acceptance of current Terms and Privacy versions before ordinary commands. Legal/privacy commands remain available.
- Private data inventory and JSON export are implemented. Export schema version is 9.
- Confirmed deletion removes player-owned records across profiles, consents, sessions/results, statistics, achievements, wallet/ledger, Atlas/Chronicle, Wayfarer/rivalries and Expedition participation/history.
- A shared game-session snapshot is removed when it contains the deleting player; the receipt explains that co-participant session access is consequently ended. Independent player aggregates remain.

### Guild settings

Manage Server permissions protect guild prefix and optional command-channel configuration. Settings are stored separately from player profiles, cached for command dispatch and invalidated through PostgreSQL LISTEN/NOTIFY. Actual bot removal triggers cleanup of that guild's settings and guild-scoped active sessions; temporary guild unavailability does not.

### Moonbits

The first completion of each game type awards a fixed 10 Moonbits once per player/game type. Quest rewards are fixed and disclosed. Wallet updates and immutable ledger rows share the source transaction and idempotency keys. Currency cannot be purchased, cashed out, staked, wagered or transferred outside Moonbyte. Spending, player trading, Vaults and Bazaar are not live.

## Code ownership

| Concern | Owner |
|---|---|
| Discord lifecycle, gateway and shutdown | `src/main.rs` |
| Shared data/services | `src/lib.rs` |
| Configuration | `src/config.rs` |
| Ordered PostgreSQL migrations and cleanup | `src/database.rs`, `migrations/` |
| Pure game rules | `src/game/engine.rs`, `src/game/model.rs` |
| Durable game sessions | `src/game/runtime.rs` |
| Discord commands/adapters | `src/commands.rs` |
| Profiles, inventory, export and deletion | `src/profile.rs` |
| Consent | `src/consent.rs` |
| Economy ledger | `src/wallet.rs` |
| Achievements | `src/achievement.rs` |
| Atlas and travel | `src/atlas.rs` |
| Chronicle | `src/chronicle.rs` |
| Wayfarer, quests and rivalries | `src/wayfarer.rs` |
| Expedition route and run state | `src/expedition.rs` |
| Tavern destinations | `src/tavern.rs` |
| Guild configuration | `src/guild_settings.rs` |

## Known gaps

- Durable per-interaction receipt records beyond the current row-locked action-version guard, if later delivery evidence justifies the additional state.
- Development-guild interaction smoke tests and accessibility review of each interactive board.
- Production database backup/restore automation, monitoring and a rehearsed recovery procedure.
- More game content, validated word/puzzle packs, further Expedition routes, inventory, crafting and a fixed-price, non-wagering Bazaar.
- A public player directory, public leaderboards and shared guild projects only after consent and guild-isolation contracts are implemented.

## Canonical documents

[Final Vision](MOONBYTE_FINAL_VISION.md) · [Architecture](ARCHITECTURE.md) · [Commands](COMMANDS.md) · [Game Catalogue](GAME_CATALOGUE.md) · [Privacy](PRIVACY.md) · [Fairness](FAIRNESS.md) · [Roadmap](ROADMAP.md) · [World Loom](atlas/README.md)
