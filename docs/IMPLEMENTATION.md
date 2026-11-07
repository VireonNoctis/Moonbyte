# Moonbyte Implementation Map

> **Document version 1.0.0 · Pre-launch source status · 6 November 2026**
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
| Schema | Ordered migrations through `0016_guild_projects.sql` |
| Player interface | Discord-native commands and messages; numbered board buttons for Tic-tac-toe, Connect Four and Memory Match |
| Toolchain | Pinned in `rust-toolchain.toml` |

## Live systems

### Command presentation

The command adapter uses a shared Discord embed title-panel builder for Craft, Tavern, Games, Expedition catalogue/status views and player-facing overview/browse screens. Relevant slash commands expose an optional `title` field, and matching prefix commands accept trailing title text after required arguments. Titles are trimmed, whitespace-normalised, mention-safe and bounded to Discord's 256-character embed-title limit. Player-owned slash views and active-session lists use ephemeral embeds; matching prefix invocations in a server are delivered by DM. Public catalogue panels preserve their configured visibility. Game boards retain canonical titles and native controls.

Slash autocomplete is generated from the authoritative local catalogues, not a second hard-coded UI list. Craft options show the output keepsake and ingredient requirements while their selected values remain canonical recipe IDs. Game, Atlas destination, Quest action, Rivalry action, Expedition action, Vault action/room and profile-visibility fields use the same shared matcher and Discord response builder. Autocomplete returns at most 25 choices, with labels/values bounded to Discord's 100-character choice limit. Prefix syntax and the existing typo suggestion path remain available.

Unknown recipes, game IDs, Atlas destinations, Vault blueprint/chamber IDs and finite Quest/Rival/Expedition/Vault action verbs receive conservative “Did you mean …?” guidance when a single option is clearly close. Suggestions never auto-correct or execute.

### Games and sessions

Four games currently run through the same transport-independent game engine:

- Tic-tac-toe — two-player guild lobby, legal moves, turn order, wins and draws.
- Connect Four — two-player guild lobby, gravity-based placement, legal columns, wins and draws.
- Memory Match — solo, eight fixed positions and four pairs.
- Word Scramble — solo, one curated starter puzzle and bounded attempts.

Sessions are stored as versioned PostgreSQL JSONB snapshots. Mutations lock one session row, validate state and membership, then commit before Discord delivery. Sessions survive runtime reconstruction. Waiting/active sessions expire after 24 hours without a successful mutation and are cleaned by one periodic task. Reads do not extend the inactivity window.

Tic-tac-toe, Connect Four and Memory Match expose numbered Discord buttons routed through the same authoritative GameRuntime as text commands. Each button carries the displayed action count; the expected count is checked again inside the PostgreSQL session-row lock, so concurrent stale clicks cannot both apply. Stale boards refresh from persisted state and terminal boards disable controls. Word Scramble remains answer-driven. Text commands remain an accessible fallback. Durable interaction receipt records beyond the action-version guard are not implemented.

### Tavern and world

The Tavern Wayfinder exposes sixteen registered destinations: Common Tables, player profile and record, Moonbits Ledger, Achievement Gallery, Atlas, Chronicle, Quest Board, Wayfarer Thread, Companion Journal, Rivalry Hall, Expedition Board, player Satchel, Postcard Wall, Wayfarer Workshop and the Puzzle Vault.

The Atlas contains five authored locations and reciprocal route edges. A player begins at Lantern Square and can travel only to a directly connected location. First discovery unlocks one private Chronicle fragment and one account-bound postcard in the same PostgreSQL transaction. Revisits do not duplicate pages, keepsakes or rewards.

The Lantern Road is one bounded authored route: Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory. Solo runs and same-guild groups of up to four are supported. Every participant opts in. The state machine supports recruiting, active, paused, completed and cancelled states; checkpoints are versioned and persisted, participant-specific Atlas/Chronicle/inventory effects use their owning services, and export/deletion includes each participant's own history. Runs can remain paused; there is no attendance deadline.

### Puzzle Vault

Mosslight Resonance is the first fixed-rule, solo Puzzle Vault. It is unlocked by discovering Mosslight Grove in the Atlas and can be opened from the Tavern Wayfinder or Atlas panel. The three authored chambers are Waystone Order, Tideglass Code and Keeper's Seal; the final chamber verifies the first two answers together. Rules, clues and accepted answers are fixed source data, not generated content.

Vault panels use the shared private title-panel builder. Slash invocations expose an optional `title` field; for prefix list/status/pause/resume/leave actions, trailing text becomes the panel title without changing command behavior. Solve answers remain a single canonical token and optional title text is parsed separately. `.vault list` reviews access and the fixed reward; `.vault enter mosslight-resonance confirm` requires explicit confirmation. `.vault solve <room-id> <answer>` records only a correct chamber's canonical ID and timestamp as a PostgreSQL save stone. Wrong guesses are checked in memory and are not persisted; the command returns the authored hint. Runs can be paused, left and resumed without a deadline. Completion grants one account-bound Mosslight Seal, with provenance tied to the completing run and blueprint version; the one-time guard prevents replay farming. Completion also unlocks a private Chronicle page and records a Wayfarer activity event. There are no Moonbits, repeatable rewards, leaderboards or group mode.
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
- `.profile` is a private, read-only Player Card composed from owning services: game totals, achievements, Wayfarer journey days, Companion stage, Rivalry counts, Chronicle pages, postcards, Satchel items and latest Puzzle Vault progress. It also provides command shortcuts. It does not initialise Atlas state, alter progress, or expose public/guild profile projections.
- The global command gate requires acceptance of current Terms and Privacy versions before ordinary commands. Legal/privacy commands remain available.
- Profile creation uses the same per-user PostgreSQL advisory lock as confirmed account deletion and re-checks both current policy versions inside the transaction. A deterministic PostgreSQL regression test queues a stale creation behind deletion and verifies it is rejected after deletion commits.
- Game-session creation and joining take the relevant player lock and re-check consent/profile existence. Moves lock participant IDs in stable order before the session row, then verify the participant set is unchanged. Domain progress handlers require an existing profile and do not create one.
- Private data inventory and JSON export are implemented. Export schema is version 12 and includes item IDs, quantity, source type, source reference and acquisition timestamp, plus Vault runs and solved chamber checkpoints, and opt-in Guild Project memberships. Export reads all categories through one read-only PostgreSQL `REPEATABLE READ` transaction, keeping related profile, progress, balance and ledger records on a single database snapshot.
- Confirmed deletion removes player-owned records across profiles, consent, sessions/results, statistics, achievements, wallet/ledger, Atlas/Chronicle, inventory, Wayfarer/rivalries, Expedition participation/history, Puzzle Vault runs/checkpoints and Guild Project memberships. Anonymous Guild Project contribution aggregates remain.
- A shared game-session snapshot is removed when it contains the deleting player; the receipt explains that co-participant session access is consequently ended. Independent player aggregates remain.

### Guild settings and cooperative projects

Manage Server permissions protect guild prefix and optional command-channel configuration. Settings are stored separately from player profiles, cached for command dispatch and invalidated through PostgreSQL LISTEN/NOTIFY. Actual bot removal triggers cleanup of that guild's settings, guild-scoped active sessions and project board; temporary guild unavailability does not.

The first Guild Project is the fixed **Lantern Road Survey**. A member with Manage Server or Administrator can open it once; opening again returns the existing board without resetting progress. Players explicitly join or leave. Its private board displays aggregate progress, enrolment count and the invoking player's own enrolment state, never member names. Only a checkpoint completed by an enrolled player in an active, same-server shared Lantern Road Expedition counts. Contribution rows contain no contributor ID; account deletion removes personal membership and anonymises retained Expedition checkpoint actor links without rolling back server progress.

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
| Puzzle Vault rules and persistent runs | `src/vault.rs` |
| Guild configuration | `src/guild_settings.rs` |
| Guild Projects | `src/guild_project.rs` |

## Known gaps

- Durable interaction receipt records beyond the current action-version guard; development-guild interaction smoke tests and accessibility review of each board.
- Backup/restore scripts and a PostgreSQL readiness probe exist, but no scheduled backup service, production monitoring or rehearsed live restore has been configured.
- Discord application creation, bot invite configuration, secrets provisioning and live guild verification have not been performed.
- More original games, reviewed word/puzzle packs and additional authored Expedition routes.
- Additional reviewed Vault blueprints/chambers, opt-in shared collections and a fixed-price Bazaar after ownership, moderation, idempotency and settlement contracts are proven.
- Public player directory/leaderboards, mastery, Almanac/seasonal content and homesteads remain proposals until their privacy and ownership contracts are implemented.

## Canonical documents

[Final Vision](MOONBYTE_FINAL_VISION.md) · [Architecture](ARCHITECTURE.md) · [Commands](COMMANDS.md) · [Game Catalogue](GAME_CATALOGUE.md) · [Privacy](PRIVACY.md) · [Fairness](FAIRNESS.md) · [Roadmap](ROADMAP.md) · [Operations](OPERATIONS.md) · [World Loom](atlas/README.md)
