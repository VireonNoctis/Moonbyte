# Moonbyte Implementation Map

> **Document version 1.0.0 · Pre-launch source status · 5 November 2026**
>
> This map describes the Rust/PostgreSQL source in this repository. No Discord bot application has been registered or deployed. The Final Vision describes intended product direction; the Roadmap records implementation and release work.

## Runtime at a glance

| Area | Current implementation |
|---|---|
| Language/runtime | Rust 2024, Tokio |
| Discord | Serenity + Poise; shared prefix and slash handlers |
| Prefix | `.` by default; per-guild override supported |
| Storage | PostgreSQL through SQLx |
| Deployment contract | One supported bot process with PostgreSQL |
| Schema | Ordered migrations through `0013_atlas_crafting_materials.sql` |
| Player interface | Discord-native commands and messages; numbered board buttons for Tic-tac-toe, Connect Four and Memory Match |
| Toolchain | Pinned in `rust-toolchain.toml` |

## Live systems

### Command presentation

The command adapter now has a reusable Discord embed title-panel helper for substantial slash results. Craft uses **Wayfarer Workshop · Crafting**, Tavern uses **Tavern Wayfinder**, Games uses **Moonbyte Game Library**, and active game boards use their game name while retaining buttons. Craft uses an ephemeral slash result and DM-delivered prefix results when invoked in a server; Tavern and Games browse panels are public; game boards remain visible to participants. Other prefix commands retain normal message visibility. Unknown recipes, game IDs, Atlas destinations and finite Quest/Rival/Expedition action verbs receive conservative “Did you mean …?” guidance when a single option is clearly close. Suggestions never auto-correct or execute. Other private player views retain their existing private delivery path.


### Games and sessions

Four games currently run through the same transport-independent game engine:

- Tic-tac-toe — two-player guild lobby, legal moves, turn order, wins and draws.
- Connect Four — two-player guild lobby, gravity-based placement, legal columns, wins and draws.
- Memory Match — solo, eight fixed positions and four pairs.
- Word Scramble — solo, one curated starter puzzle and bounded attempts.

Sessions are stored as versioned PostgreSQL JSONB snapshots. Mutations lock one session row, validate state and membership, then commit before Discord delivery. Sessions survive runtime reconstruction. Waiting/active sessions expire after 24 hours without a successful mutation and are cleaned by one periodic task. Reads do not extend the inactivity window.

Tic-tac-toe, Connect Four and Memory Match expose numbered Discord buttons routed through the same authoritative GameRuntime as text commands. Each button carries the displayed action count; the expected count is checked again inside the PostgreSQL session-row lock, so concurrent stale clicks cannot both apply. Stale boards refresh from persisted state and terminal boards disable controls. Word Scramble remains answer-driven. Text commands remain an accessible fallback. Durable interaction receipt records beyond the action-version guard are not implemented.

### Tavern and world

The Tavern Wayfinder exposes fifteen registered destinations: Common Tables, player profile and record, Moonbits Ledger, Achievement Gallery, Atlas, Chronicle, Quest Board, Wayfarer Thread, Companion Journal, Rivalry Hall, Expedition Board, player Satchel, Postcard Wall and Wayfarer Workshop.

The Atlas contains five authored locations and reciprocal route edges. A player begins at Lantern Square and can travel only to a directly connected location. First discovery unlocks one private Chronicle fragment and one account-bound postcard in the same PostgreSQL transaction. Revisits do not duplicate pages, keepsakes or rewards.

The Lantern Road is one bounded authored route: Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory. Solo runs and same-guild groups of up to four are supported. Every participant opts in. The state machine supports recruiting, active, paused, completed and cancelled states; checkpoints are versioned and persisted, participant-specific Atlas/Chronicle/inventory effects use their owning services, and export/deletion includes each participant's own history. Runs can remain paused; there is no attendance deadline.

### Inventory and collection

- `.inventory` shows account-bound items with quantity, source and acquisition timestamp.
- `.collection` shows progress across five Atlas postcards.
- A postcard is granted once per location from a verified Atlas discovery. The record is written in the same transaction as Atlas/Chronicle progression.
- Migration 0012 backfills existing Atlas discoveries so existing players receive the matching keepsakes.
- Inventory items retain stable IDs and source provenance. These starter keepsakes cannot be bought, sold, transferred or spent.
- The Wayfarer Workshop supports one deterministic recipe: one Mosslight Silk plus one Tideglass yields one account-bound Wayfarer's Lantern. Input consumption and output creation share a transaction; a per-player advisory lock and unique recipe provenance prevent duplicate crafting.
- More recipes, gathering sources, trading and Bazaar listings are not live.

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
- Private data inventory and JSON export are implemented. Export schema is version 10 and includes item IDs, quantity, source type, source reference and acquisition timestamp.
- Confirmed deletion removes player-owned records across profiles, consent, sessions/results, statistics, achievements, wallet/ledger, Atlas/Chronicle, inventory, Wayfarer/rivalries and Expedition participation/history.
- A shared game-session snapshot is removed when it contains the deleting player; the receipt explains that co-participant session access is consequently ended. Independent player aggregates remain.

### Guild settings

Manage Server permissions protect guild prefix and optional command-channel configuration. Settings are stored separately from player profiles, cached for command dispatch and invalidated through PostgreSQL LISTEN/NOTIFY. Actual bot removal triggers cleanup of that guild's settings and guild-scoped active sessions; temporary guild unavailability does not.

### Moonbits

The first completion of each game type awards a fixed 10 Moonbits once per player/game type. Quest rewards are fixed and disclosed. Wallet updates and immutable ledger rows share the source transaction and idempotency keys. Currency cannot be purchased, cashed out, staked, wagered or transferred outside Moonbyte. Spending and trading are not live.

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
| Profiles, inventory, export and deletion inventory accounting | `src/profile.rs` |
| Inventory catalog and provenance records | `src/inventory.rs` |
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

- Durable interaction receipt records beyond the current action-version guard; development-guild interaction smoke tests and accessibility review of each board.
- Backup/restore scripts and a PostgreSQL readiness probe exist, but no scheduled backup service, production monitoring or rehearsed live restore has been configured.
- Discord application creation, bot invite configuration, secrets provisioning and live guild verification have not been performed.
- More original games, reviewed word/puzzle packs and additional authored Expedition routes.
- Known-input crafting, fixed-rule Puzzle Vaults, opt-in guild projects and a fixed-price Bazaar after ownership, moderation, idempotency and settlement contracts are proven.
- Public player directory/leaderboards, mastery, Almanac/seasonal content and homesteads remain proposals until their privacy and ownership contracts are implemented.

## Canonical documents

[Final Vision](MOONBYTE_FINAL_VISION.md) · [Architecture](ARCHITECTURE.md) · [Commands](COMMANDS.md) · [Game Catalogue](GAME_CATALOGUE.md) · [Privacy](PRIVACY.md) · [Fairness](FAIRNESS.md) · [Roadmap](ROADMAP.md) · [Operations](OPERATIONS.md) · [World Loom](atlas/README.md)
