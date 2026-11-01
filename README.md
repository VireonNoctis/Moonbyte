# Moonbyte

> A little world of games, right inside Discord.

Moonbyte is a Discord-first gaming bot built around an original, full-colour pixel-art arcade experience. It brings quick mini-games, puzzles, multiplayer sessions, collection systems, and longer adventure/RPG experiences into the servers people already use.

Moonbyte is a **Gaming Bot**. It has its own Rust codebase, game engine, storage, commands, identity, and release lifecycle. It does not depend on Lunar's running bot, website, account system, or database.

<p align="center"><strong>PIXEL WORLDS · QUICK GAMES · SHARED ADVENTURES</strong></p>

## Project status

**First playable runtime slice implemented.** Moonbyte connects through Tokio, Serenity and Poise, and now includes four Rust game modules: Tic-tac-toe, Connect Four, Memory Match and Word Scramble. The game rules are transport-independent; multiplayer lobbies are guild-scoped; participants, turn ownership and legal actions are validated. Sessions currently live in process memory and reset on restart. SQLite persistence, profiles, progression, Vaults, Bazaar and the visual frontend remain implementation work.

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
- The Rust/Tokio backend remains authoritative for game rules, session state, fairness, player progression, and SQLite persistence.
- Standard bot commands still use Discord messages, embeds, buttons, select menus, and modals. React is not rendered inside an ordinary Discord embed.

## Command model

The default prefix is a single period: **.**. Set `DEFAULT_PREFIX` to change it.

| Command | Status | Purpose |
| --- | --- | --- |
| `.help` / `/help` | Implemented | List commands available in this build. |
| `.about` / `/about` | Implemented | Describe Moonbyte and its current implementation boundary. |
| `.ping` / `/ping` | Implemented | Confirm command responsiveness and show process uptime. |
| `.privacy` / `/privacy` | Implemented | Explain current data handling and privacy-control limitations. |
| `.games` / `/games` | Implemented | Browse the four currently playable games and their controls. |
| `.play <game>` / `/play` | Implemented | Start a solo game or create a two-player lobby. |
| `.join <session>` / `/join` | Implemented | Join a two-player lobby in the same Discord server. |
| `.action <session> <input>` / `/action` | Implemented | Submit a validated move or answer. |
| `.session <session>` / `/session` | Implemented | Inspect a session you participate in. |
| `.leave <session>` / `/leave` | Implemented | End a session you participate in. |
| `.daily` / `/daily` | Planned | Open the optional daily challenge. |
| `.profile` / `/profile` | Planned | View a player profile and statistics. |
| `.collection` / `/collection` | Planned | View collected cards, discoveries, and items. |
| `.inventory` / `/inventory` | Planned | View usable items and cosmetics. |
| `.quests` / `/quests` | Planned | Browse active and available quests. |
| `.party` / `/party` | Planned | Create, invite, or manage a party. |
| `.leaderboard` / `/leaderboard` | Planned | View supported rankings. |
| `.achievements` / `/achievements` | Planned | Browse earned achievements. |
| `.privacy data` / `/privacy data` | Planned | Inspect categories of data associated with the requester. |
| `.privacy export` / `/privacy export` | Planned | Request a private copy of stored profile data. |
| `.privacy delete` / `/privacy delete` | Planned | Request confirmed deletion of Moonbyte profile data. |
| `.settings` / `/settings` | Planned | Configure player and server preferences. |
| `.stop` / `/stop` | Planned | Leave or end an eligible active session. |

Poise defines prefix and slash commands from shared Rust handlers. Global slash commands are registered at startup and may take up to an hour to appear everywhere. Prefix commands require the Discord Message Content privileged intent to be enabled for the application and requested by the bot. The current runtime requests only `GUILDS`, `GUILD_MESSAGES`, and `MESSAGE_CONTENT`.

The current `.privacy` command is an overview only. It does not provide a data inventory, export, or deletion flow. Active sessions are held in process memory and disappear on restart; they are not persisted to SQLite. See [Command reference](docs/COMMANDS.md) and [Game Runtime](docs/GAME_RUNTIME.md) for exact behavior and limitations.

## Privacy is a product feature

Moonbyte's stance is **privacy by design and data minimisation**. We should collect only the information required to operate enabled game features, explain what is stored, avoid selling or sharing player data for advertising, and provide in-bot privacy controls.

- No advertising trackers, behavioural profiling, or sale of personal data.
- No AI training on player messages or game activity.
- No raw message-content analytics. Prefix text is processed to identify the command and arguments, then discarded unless a specific game needs a value for its active session.
- No cross-server exposure of guild settings or server-only leaderboards.
- Profile export and deletion controls are part of the planned product, not an afterthought.
- No blanket end-to-end-encryption claim: Discord and the bot must process commands and game actions for the bot to work. The bot's database is not considered end-to-end encrypted.

Read the detailed [Privacy Policy and Data Practices](docs/PRIVACY.md). The current runtime does not connect to SQLite or persist player profiles, game sessions or results. Active game state is process-local, guild-scoped and participant-readable; it disappears when the process restarts. Data inventory, export and deletion commands are not implemented; the policy remains a target that must be verified as storage and game features are added.

## Technology decisions

| Area | Decision | Purpose |
| --- | --- | --- |
| Language | **Rust** | Performance, memory safety, and explicit game-state modelling. |
| Async runtime | **Tokio** | Efficient asynchronous Discord networking and concurrent session coordination. |
| Discord API | **Serenity** | Gateway, HTTP, interactions, and sharding support. |
| Command framework | **Poise** | Shared prefix and slash command definitions. |
| Persistent storage | **SQLite** | Embedded database for the initial single-instance deployment. |
| Database access | **SQLx with SQLite only** | Async database access and migration support. |
| Visual library | **Next.js + React + TypeScript** | Catalogue browsing, responsive game UI, and Activity-capable views. |
| Discord embedded UI | **Discord Embedded App SDK** | Connects the web frontend to Discord when running as an Activity. |
| Art direction | **Original full-colour pixel art** | Distinctive characters, worlds, boards, and interface elements. |

SQLite will use migrations, indexes, short transactions, and write-ahead logging where appropriate.

## Economy, Vaults and Bazaar

Moonbyte's planned progression system includes a personal Vault, cooperative Puzzle/Expedition/Guild Vaults, collectible Archives, crafting, and a player Bazaar with fixed-price listings, transparent barter, escrow, and trade history. Rewards and prices are disclosed and deterministic.

Moonbyte will not implement gambling, betting, wagering, staking, paid random rewards, cash-out, or chance-based tradable rewards. Casual random games are score-only and never risk or award currency/items. See [Economy, Vaults & Bazaar](docs/ECONOMY_AND_VAULTS.md) and [Fairness and Randomness](docs/FAIRNESS.md).

## Documentation

- [Product vision](docs/PRODUCT_VISION.md) — detailed product goals, engagement principles, systems, safety, and non-goals.
- [Privacy policy and data practices](docs/PRIVACY.md) — data categories, purposes, retention, controls, limits, and security stance.
- [Architecture](docs/ARCHITECTURE.md) — runtime boundaries, game engine, concurrency, persistence, rendering, and operations.
- [Game catalogue](docs/GAME_CATALOGUE.md) — approved game selections, inherited Lunar baseline, shared contracts, and release quality gates.
- [Economy, Vaults & Bazaar](docs/ECONOMY_AND_VAULTS.md) — player inventory, vault experiences, fixed-price trading, and non-wagering economy rules.
- [Moonbyte World Loom](docs/atlas/README.md) — 400 connected system ideas across 40 domains, ten design volumes, a 40,000+ line specification and the machine-readable mind-map graph.
- [Fairness and randomness](docs/FAIRNESS.md) — SHA-512 commit–reveal protocol, unbiased result derivation, and verification requirements.
- [Lunar migration plan](docs/MIGRATION_FROM_LUNAR.md) — what may be reused conceptually and what must be rebuilt or audited.
- [Roadmap](docs/ROADMAP.md) — phased implementation, deliverables, and exit criteria.
- [Architecture decisions](docs/DECISIONS/README.md) — accepted decisions and trade-offs.
- [Engineering instructions](AGENTS.md) — repository rules for contributors and coding agents.

## Development status

The repository contains a Rust 2024 workspace, committed lockfile, validated environment configuration, and a working Tokio/Serenity/Poise Discord runtime. Startup validates the Discord token and optional prefix/database-path settings without printing secret values, connects to Discord, registers global slash commands, and exposes the ten commands documented in [Command reference](docs/COMMANDS.md). The first four game rules and in-memory session lifecycle are implemented and unit-tested. SQLite, migrations, durable saves, player profiles and result persistence are not connected yet.

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

The intended data model contains only information needed to provide enabled features: Discord identifiers, player and guild settings, game progress, achievements, collections, results, and recoverable session state. Raw message content should not be retained as gameplay analytics. Players should be able to inspect and request deletion of their stored profile data. Guild-specific configuration and rankings must remain isolated.

## Licensing

A project license has not yet been selected. Until one is added, do not assume that code or assets may be reused, modified, or redistributed. Lunar source material and all third-party assets must have their provenance and license checked before migration.
