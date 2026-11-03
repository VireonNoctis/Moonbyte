# Moonbyte

> A little world of games, right inside Discord.

Moonbyte is a Discord-first gaming bot built around an original, full-colour pixel-art arcade experience. It brings quick mini-games, puzzles, multiplayer sessions, collection systems, and longer adventure/RPG experiences into the servers people already use.

Moonbyte is a **Gaming Bot**. It has its own Rust codebase, game engine, storage, commands, identity, and release lifecycle. It does not depend on Lunar's running bot, website, account system, or database.

<p align="center"><strong>PIXEL WORLDS · QUICK GAMES · SHARED ADVENTURES</strong></p>

## Project status

**The player foundation, results loop, achievements, earned-only wallet, first Atlas slice, guild command routing and versioned policy-consent gate are implemented on this branch.** Alongside four transport-independent Rust games and durable PostgreSQL sessions, Moonbyte has persistent profiles, completed-game results, per-game statistics, five idempotent achievement milestones, private JSON export/deletion, and the initial Tavern Wayfinder. Each player's first completion of each game type earns a fixed 10 Moonbits once, with an immutable ledger entry and wallet update in the same transaction as the game result. The initial Atlas has five authored locations, a connected route graph, persistent personal discoveries and direct-neighbour travel; it offers no repeatable travel rewards. Spending, transfers, Vault rewards and Bazaar trading are not enabled.

## The vision

Moonbyte should feel like opening a living, colourful handheld arcade and connected social adventure world inside Discord: a place to discover something new, master familiar games, invite friends, solve challenges, and return to evolving worlds.

The product should be highly replayable without being manipulative. Depth comes from genuinely different games, skill mastery, discovery, cooperative goals, social play, and meaningful player choice—not from making play compulsive or difficult to leave. Players should always be able to stop, pause, mute reminders, and return without punishment.

## Five game pillars

| Category | Direction | Examples to explore |
| --- | --- | --- |
| Existing Lunar games | Rebuild selected games and useful mechanics from Lunar in Rust. | Akinator-style guessing, coin flip, dice, 8-ball, reviewed collection mechanics. |
| Original mini-games | Short, replayable experiences designed specifically for Moonbyte. | Memory match, word scramble, sequence recall, trivia, pattern recognition. |
| Multiplayer | Friends, parties, public lobbies, and cooperative play. | Tic-tac-toe, Connect Four, team quiz, co-op puzzles, turn-based party games. |
| Adventure / RPG | Persistent, story-led worlds with meaningful progression. | Exploration, quests, NPCs, inventory, crafting, collections, turn-based encounters. |
| Puzzle / strategy | Thoughtful solo and group challenges. | Sudoku, nonograms, logic grids, deduction, chess/checkers, daily puzzles. |

Every listed title is a candidate, not a promise that it already exists. The catalogue should grow through polished releases rather than a target number of shallow games.

## The player experience

- Discover games through a browsable library with categories, descriptions, player counts, estimated session length, and rules.
- Start solo play or invite friends into a lobby.
- Interact using Discord messages, buttons, select menus, and modals.
- See clear boards, expressive sprites, colourful scenes, and readable result screens.
- Keep appropriate progress, records, achievements, collections, and adventure saves.
- Leave or pause supported sessions without being nagged or punished.
- Find a different experience when a game is completed.

## Visual Library

The visual library now has a first **Next.js + React + TypeScript** catalogue foundation under `visual-library/`: responsive game cards, search/category filters, accessible rules disclosure, and copyable Discord commands for the four implemented games. It is a presentation-only browser fallback—not yet a live game client or Discord Activity.

- Discord Activity integration using the official Embedded App SDK remains planned and is not yet implemented.
- A normal HTTPS browser fallback remains available for catalogue discovery and supported views.
- The Rust/Tokio backend remains authoritative for game rules, session state, fairness, player progression, and PostgreSQL persistence.
- Standard bot commands still use Discord messages, embeds, buttons, select menus, and modals. React is not rendered inside an ordinary Discord embed.

## Command model

The process default prefix is a single period: **.**. Set `DEFAULT_PREFIX` to change it. Server moderators can override it per guild and optionally restrict bot commands to one channel with `.prefix`.

| Command | Status | Purpose |
| --- | --- | --- |
| `.help` / `/help` | Implemented | List commands available in this build. |
| `.about` / `/about` | Implemented | Describe Moonbyte and its current implementation boundary. |
| `.ping` / `/ping` | Implemented | Confirm command responsiveness and show process uptime. |
| `.terms` / `/terms` | Implemented | Receive the current Terms of Service document. |
| `.consent accept` / `/consent` | Implemented | Explicitly agree to the current Terms and Privacy Policy versions. |
| `.prefix` / `/prefix` | Implemented | Manage Server moderators configure the guild prefix and optional command channel. |
| `.tavern` / `/tavern` | Implemented | Open the Tavern home and Wayfinder for live destinations. |
| `.privacy` / `/privacy` | Implemented | Receive the full Privacy Policy document and summary. |
| `.privacy_data` / `/privacy_data` | Implemented | Inspect your current Moonbyte data inventory privately. |
| `.privacy_export` / `/privacy_export` | Implemented | Receive a private JSON export of current stored records. |
| `.privacy_delete` / `/privacy_delete` | Implemented | Begin and explicitly confirm deletion of your Moonbyte data. |
| `.profile` / `/profile` | Implemented | View your private player profile. |
| `.profile_edit` / `/profile_edit` | Implemented | Set a display name and short bio. |
| `.settings` / `/settings` | Implemented | Save profile visibility and player preferences. |
| `.stats` / `/stats` | Implemented | View completed-game totals and per-game wins/draws. |
| `.wallet` / `/wallet` | Implemented | View the earned-only Moonbits balance and ledger-entry count privately. |
| `.achievements` / `/achievements` | Implemented | View the five fixed achievement milestones earned through play. |
| `.atlas` / `/atlas` | Implemented | View your current location, known places and directly connected routes privately. |
| `.travel <location>` / `/travel` | Implemented | Travel to a directly connected Atlas location and record its discovery. |
| `.games` / `/games` | Implemented | Browse the four currently playable games and their controls. |
| `.play <game>` / `/play` | Implemented | Start a solo game or create a two-player lobby. |
| `.join <session>` / `/join` | Implemented | Join a two-player lobby in the same Discord server. |
| `.action <session> <input>` / `/action` | Implemented | Submit a validated move or answer. |
| `.session <session>` / `/session` | Implemented | Inspect a session you participate in. |
| `.sessions` / `/sessions` | Implemented | List your 10 most recently updated active sessions in this server or DM scope. |
| `.leave <session>` / `/leave` | Implemented | End a session you participate in. |

The Wayfinder currently exposes six live destinations: Common Tables (`.games`), Your Place (`.profile`), the Chronicle (`.stats`), The Ledger (`.wallet`), The Gallery (`.achievements`) and The Atlas (`.atlas`). The Atlas destination opens a five-location starter map; Expeditions, story routes, Vaults, parties and Bazaar trading remain planned.

### Planned commands

| Command | Status | Purpose |
| --- | --- | --- |
| `.daily` / `/daily` | Planned | Open an optional daily challenge without attendance penalties. |
| `.collection` / `/collection` | Planned | View collected cards, discoveries and items. |
| `.inventory` / `/inventory` | Planned | View usable items and cosmetics. |
| `.quests` / `/quests` | Planned | Browse active and available quests. |
| `.party` / `/party` | Planned | Create, invite or manage a party. |
| `.leaderboard` / `/leaderboard` | Planned | View supported rankings. |
| `.stop` / `/stop` | Planned | Leave or end an eligible active session. |

Poise defines prefix and slash commands from shared Rust handlers. Global slash commands are registered at startup and may take up to an hour to appear everywhere. Prefix commands require the Discord Message Content privileged intent to be enabled for the application and requested by the bot. The current runtime requests only `GUILDS`, `GUILD_MESSAGES`, and `MESSAGE_CONTENT`.

The `.terms` command provides the full Terms of Service and `.privacy` provides the full Privacy Policy. New users must review both, then run `.consent accept` before ordinary commands are unlocked. A material policy update requires renewed acceptance. Use `.privacy_data`, `.privacy_export`, and `.privacy_delete` for private account controls; when invoked in a server, account details, exports and deletion receipts are delivered by DM. Deletion requires the explicit second command `.privacy_delete DELETE` and removes the profile, per-game statistic rows, Moonbits wallet and ledger entries, and every session snapshot plus linked result containing the requester’s Discord ID, together with personal Atlas location/discovery records. This intentionally ends shared sessions whose snapshot contains multiple participant IDs. See [Player Profiles](docs/PLAYER_PROFILES.md), [Command reference](docs/COMMANDS.md), and [Privacy Policy](docs/PRIVACY.md) for exact coverage and limits.

## Privacy is a product feature

Moonbyte's stance is **privacy by design and data minimisation**. We should collect only the information required to operate enabled game features, explain what is stored, avoid selling or sharing player data for advertising, and provide in-bot privacy controls.

- No advertising trackers, behavioural profiling, or sale of personal data.
- No AI training on player messages or game activity.
- No raw message-content analytics. Prefix text is processed to identify the command and arguments, then discarded unless a specific game needs a value for its active session.
- No cross-server exposure of guild settings or server-only leaderboards.
- In-bot export and deletion controls cover the current persisted data; every new player-owned system must extend them before release.
- No blanket end-to-end-encryption claim: Discord and the bot must process commands and game actions for the bot to work. The bot's database is not considered end-to-end encrypted.

Read the detailed [Privacy Policy and Data Practices](docs/PRIVACY.md). The current runtime persists profiles/preferences, session snapshots, completed results, per-game statistics, achievements, earned-only Moonbits wallet/ledger rows, personal Atlas state and discovery history in PostgreSQL. Session-level results expire with their parent session; aggregate statistics, achievements, wallet milestones and Atlas discoveries remain until account deletion. Inventory, quests, Expeditions, Vaults and Bazaar trades are not persisted yet. Version 6 export includes the current profile, Terms/Privacy acceptance record, session metadata, results, statistics, achievements, wallet/ledger and Atlas state; deletion covers all ten current player-owned tables plus any shared session snapshots containing the requester. It cannot delete Discord’s own records or independent infrastructure backups.

## Technology decisions

| Area | Decision | Purpose |
| --- | --- | --- |
| Language | **Rust** | Performance, memory safety, and explicit game-state modelling. |
| Async runtime | **Tokio** | Efficient asynchronous Discord networking and concurrent session coordination. |
| Discord API | **Serenity** | Gateway, HTTP, interactions, and sharding support. |
| Command framework | **Poise** | Shared prefix and slash command definitions. |
| Persistent storage | **PostgreSQL** | Transactional database for session persistence, recovery and future player systems. |
| Database access | **SQLx with PostgreSQL only** | Async connection pooling, migrations, JSONB state snapshots and transactional row locking. |
| Visual library | **Next.js + React + TypeScript** | Catalogue browsing, responsive game UI, and Activity-capable views. |
| Discord embedded UI | **Discord Embedded App SDK** | Connects the web frontend to Discord when running as an Activity. |
| Art direction | **Original full-colour pixel art** | Distinctive characters, worlds, boards, and interface elements. |

PostgreSQL uses versioned migrations, indexes, bounded connection pooling, JSONB session snapshots, and short row-locked transactions. The supported deployment remains one bot process until cross-process interaction idempotency and load behavior are tested.

## Economy, Vaults and Bazaar

Moonbyte's planned progression system includes a personal Vault, cooperative Puzzle/Expedition/Guild Vaults, collectible Archives, crafting, and a player Bazaar with fixed-price listings, transparent barter, escrow, and trade history. Rewards and prices are disclosed and deterministic.

Moonbyte will not implement gambling, betting, wagering, staking, paid random rewards, cash-out, or chance-based tradable rewards. Casual random games are score-only and never risk or award currency/items. See [Economy, Vaults & Bazaar](docs/ECONOMY_AND_VAULTS.md) and [Fairness and Randomness](docs/FAIRNESS.md).

## Documentation

- [Product vision](docs/PRODUCT_VISION.md) — product goals, engagement principles, systems, safety and non-goals.
- [Tavern and Wayfinder](docs/TAVERN.md) — the implemented home navigation and its live destination boundary.
- [Player profiles](docs/PLAYER_PROFILES.md) — identity, preferences, game records, achievements, Moonbits and privacy coverage.
- [Guild command configuration](docs/GUILD_CONFIGURATION.md) — per-server prefixes, moderator permissions and command-channel routing.
- [Terms of Service](docs/TERMS_OF_SERVICE.md) — rules for using Moonbyte and the current agreement version.
- [Atlas starter map](docs/ATLAS.md) — the five authored locations, direct-route travel and personal discovery contract.
- [Economy, Vaults & Bazaar](docs/ECONOMY_AND_VAULTS.md) — the implemented first-completion ledger and planned Vault/trading boundaries.
- [Privacy policy and data practices](docs/PRIVACY.md) — data categories, purposes, retention, controls, limits and security stance.
- [Architecture](docs/ARCHITECTURE.md) — runtime boundaries, game engine, concurrency, persistence, rendering and operations.
- [Game catalogue](docs/GAME_CATALOGUE.md) — approved game selections, inherited Lunar baseline, shared contracts and release quality gates.
- [Moonbyte World Loom](docs/atlas/README.md) — 400 connected system proposals across 40 domains and the machine-readable system graph.
- [Fairness and randomness](docs/FAIRNESS.md) — SHA-512 commit–reveal protocol, unbiased result derivation and verification requirements.
- [Lunar migration plan](docs/MIGRATION_FROM_LUNAR.md) — reuse boundaries and source/licence audit requirements.
- [Roadmap](docs/ROADMAP.md) — implementation phases and acceptance criteria.

## Development status

The repository contains a Rust 2024 workspace, committed lockfile, validated environment configuration, and a working Tokio/Serenity/Poise Discord runtime with guild-specific prefix and command-channel configuration. Startup validates the Discord token, prefix and required PostgreSQL URL without printing secret values, connects to PostgreSQL, applies versioned migrations, then starts Discord and registers global slash commands. The first four game rules, PostgreSQL-backed session lifecycle, player profiles/preferences, atomic completed-game results, per-game statistics, five achievement milestones, the first-completion Moonbits ledger, the six-destination Tavern Wayfinder, the five-location Atlas, private data inventory/export/deletion and the custom-emoji registry are implemented with unit and PostgreSQL integration coverage. The Atlas only allows direct-route travel and issues no travel rewards. Inventory, quests, Expeditions, authored story, spending, full Hearth behavior, Vaults and Bazaar remain future work.

Set configuration in the process environment before launching the executable with `cargo run --locked`. The `.env.example` file is a reference template; Moonbyte does not parse `.env` files automatically.

The repository validation and Rust checks are:

~~~bash
python3 scripts/validate.py
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
~~~

CI runs the repository validator plus Rust formatting, tests, and Clippy. Do not describe the Discord gateway or database as operational until those adapters are implemented and exercised.

## Data and privacy

The current data model stores Discord identifiers in player profiles, session snapshots, result rows, per-game aggregate statistics, wallet balances and wallet ledger entries, plus a display name, optional bio, visibility, two profile preference flags and recoverable session state. The Tavern Wayfinder is navigation-only and adds no persisted data. Achievements, collections and inventory are not persisted yet. Raw message content should not be retained as gameplay analytics. Players should be able to inspect and request deletion of their stored profile data. Guild-specific configuration and rankings must remain isolated. Command routing stores the guild ID, custom prefix and optional command-channel ID; it is not part of an individual player's export or profile deletion.

## Licensing

A project license has not yet been selected. Until one is added, do not assume that code or assets may be reused, modified, or redistributed. Lunar source material and all third-party assets must have their provenance and license checked before migration.
