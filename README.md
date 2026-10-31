# Moonbyte

> A little world of games, right inside Discord.

Moonbyte is a Discord-first gaming bot built around an original, full-colour pixel-art arcade experience. It brings quick mini-games, puzzles, multiplayer sessions, collection systems, and longer adventure/RPG experiences into the servers people already use.

Moonbyte is a **separate bot built from selected game concepts and systems in Lunar**. It has its own Rust codebase, game engine, storage, commands, identity, and release lifecycle. It does not depend on Lunar's running bot, website, account system, or database.

<p align="center"><strong>PIXEL WORLDS · QUICK GAMES · SHARED ADVENTURES</strong></p>

## Project status

**Product specification and repository foundation.** Product direction, technology decisions, architecture, game catalogue, privacy stance, Lunar migration inventory, and staged roadmap are documented. The executable Discord bot and playable catalogue have not yet been implemented. Commands and features below describe intended behavior, not currently working functionality.

## The vision

Moonbyte should feel like opening a living, colourful handheld arcade inside Discord: a place to discover something new, master familiar games, invite friends, solve challenges, and return to evolving worlds.

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

The visual library is a dedicated **Next.js + React + TypeScript** frontend for browsing the catalogue, viewing detailed game cards, creating/joining richer lobbies, and displaying game interfaces that benefit from a full web UI.

- In Discord, the frontend can be launched as a Discord Activity using the official Embedded App SDK, subject to application configuration and Activity requirements.
- A normal HTTPS browser fallback remains available for catalogue discovery and supported views.
- The Rust/Tokio backend remains authoritative for game rules, session state, fairness, player progression, and SQLite persistence.
- Standard bot commands still use Discord messages, embeds, buttons, select menus, and modals. React is not rendered inside an ordinary Discord embed.

## Command model

The default prefix is a single period: **.**

| Prefix command | Intended purpose |
| --- | --- |
| .help | Browse commands and get started. |
| .games | Browse and filter the game library. |
| .play <game> | Start a game or join a compatible session. |
| .daily | Open the optional daily challenge. |
| .profile | View a player profile and statistics. |
| .collection | View collected cards, discoveries, and items. |
| .inventory | View usable items and cosmetics. |
| .quests | Browse active and available quests. |
| .party | Create, invite, or manage a party. |
| .leaderboard | View supported rankings. |
| .achievements | Browse earned achievements. |
| .privacy | Read Moonbyte's privacy stance and available controls. |
| .privacy data | See categories of data associated with your profile. |
| .privacy export | Request a copy of your stored profile data. |
| .privacy delete | Request deletion of your Moonbyte profile data. |
| .settings | Configure preferences and privacy controls. |
| .stop | Leave or end an eligible active session. |

Slash command equivalents should accompany prefix commands where Discord supports the same interaction. Poise is the chosen command framework over Serenity. Prefix commands require Discord's privileged Message Content intent to be enabled for the application and requested by the bot.

The planned privacy command should provide overview, data, export, and delete flows. Deletion must require explicit confirmation and must not be hidden behind a support ticket.

## Privacy is a product feature

Moonbyte's stance is **privacy by design and data minimisation**. We should collect only the information required to operate enabled game features, explain what is stored, avoid selling or sharing player data for advertising, and provide in-bot privacy controls.

- No advertising trackers, behavioural profiling, or sale of personal data.
- No AI training on player messages or game activity.
- No raw message-content analytics. Prefix text is processed to identify the command and arguments, then discarded unless a specific game needs a value for its active session.
- No cross-server exposure of guild settings or server-only leaderboards.
- Profile export and deletion controls are part of the planned product, not an afterthought.
- No blanket end-to-end-encryption claim: Discord and the bot must process commands and game actions for the bot to work. The bot's database is not considered end-to-end encrypted.

Read the detailed [Privacy Policy and Data Practices](docs/PRIVACY.md). The policy describes intended production behavior; the bot is not yet implemented, so it is not a claim that a running service currently collects only these fields.

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
- [Game catalogue](docs/GAME_CATALOGUE.md) — approved game selections, inherited Lunar baseline, shared contracts, and release quality gates.\n- [Economy, Vaults & Bazaar](docs/ECONOMY_AND_VAULTS.md) — player inventory, vault experiences, fixed-price trading, and non-wagering economy rules.\n- [Fairness and randomness](docs/FAIRNESS.md) — SHA-512 commit–reveal protocol, unbiased result derivation, and verification requirements.
- [Lunar migration plan](docs/MIGRATION_FROM_LUNAR.md) — what may be reused conceptually and what must be rebuilt or audited.
- [Roadmap](docs/ROADMAP.md) — phased implementation, deliverables, and exit criteria.
- [Architecture decisions](docs/DECISIONS/README.md) — accepted decisions and trade-offs.
- [Engineering instructions](AGENTS.md) — repository rules for contributors and coding agents.

## Development status

The repository currently contains the product specification and repository-quality scaffold. A Cargo workspace, executable Discord bot, game engine, migrations, and runtime-specific build commands are future implementation milestones.

The currently available baseline repository check is:

~~~bash
python3 scripts/validate.py
~~~

Once the Rust workspace is added, CI must validate formatting, Clippy, compilation, unit and integration tests, and the committed lockfile. Do not document commands as available until they exist and have been run successfully.

## Data and privacy

The intended data model contains only information needed to provide enabled features: Discord identifiers, player and guild settings, game progress, achievements, collections, results, and recoverable session state. Raw message content should not be retained as gameplay analytics. Players should be able to inspect and request deletion of their stored profile data. Guild-specific configuration and rankings must remain isolated.

## Licensing

A project license has not yet been selected. Until one is added, do not assume that code or assets may be reused, modified, or redistributed. Lunar source material and all third-party assets must have their provenance and license checked before migration.
