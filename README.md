# Moonbyte

> A little world of games, right inside Discord.

Moonbyte is a standalone, Discord-first gaming bot built in Rust. Its world combines quick games, puzzles, shared adventures and persistent player progress through Discord commands and native message interactions.

**The bot is the product.** Discord is the player interface; Rust owns game rules and authoritative state; PostgreSQL stores durable progress. No separate client is required.

## Current build

This branch contains a working Rust/Tokio/Serenity/Poise runtime and PostgreSQL persistence:

- Four playable games: Tic-tac-toe, Connect Four, Memory Match and Word Scramble.
- Durable sessions, transactional moves, restart recovery and 24-hour inactive-session retention.
- Player profiles, preferences, completed results, statistics and five achievements.
- An earned-only Moonbits ledger: a fixed first-completion milestone per game type, with no purchase, transfer or wagering features.
- A 12-destination Tavern Wayfinder.
- A five-location Atlas with direct-route travel and private Chronicle pages.
- Optional daily quests, cumulative journey days, cosmetic companion milestones and mutually accepted private rivalries.
- The Lantern Road Expedition, playable solo or with up to three consenting companions, with persistent checkpoints and Atlas/Chronicle hand-offs.
- Versioned Terms/Privacy consent, private data inventory, version 9 export and confirmed account deletion.
- Per-server command prefixes and optional command-channel routing.

Three starter games now have Discord-native numbered board controls, with text commands retained as a fallback. Button payloads carry the displayed action version; stale controls refresh from PostgreSQL instead of replaying a move. Structured tracing records minimised lifecycle and cleanup events. Durable duplicate-interaction records and production backup/restore drills remain in the roadmap. A green CI run is not a substitute for live Discord deployment testing.

## Technology

| Layer | Choice |
|---|---|
| Application and game rules | Rust, edition 2024 |
| Async runtime | Tokio |
| Discord API | Serenity |
| Commands | Poise, shared prefix and slash handlers |
| Storage | PostgreSQL through SQLx |
| Player interface | Discord messages, embeds, buttons, select menus and modals |
| Art | Original full-colour pixel art and custom emoji where useful |

The default prefix is `.`. Prefix commands require the Message Content privileged intent. Slash commands are also available.

## Run locally

1. Start PostgreSQL: `docker compose up -d postgres`.
2. Set `DISCORD_TOKEN` and `DATABASE_URL` in the process environment. Set `DEFAULT_PREFIX` only when changing the default period prefix.
3. Invite the bot with the `bot` and `applications.commands` scopes. Enable Message Content in the Discord developer settings when using prefix commands.
4. Run `cargo run --locked`.

Moonbyte does not automatically load `.env` files. Never commit real credentials or log tokens/database URLs.

## Validation

```bash
python3 scripts/validate.py
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

CI provisions PostgreSQL and runs repository validation, formatting, tests and Clippy.

## Documentation — one source per concern

- [Final Vision](docs/MOONBYTE_FINAL_VISION.md) — product principles, connected world and long-term design.
- [Implementation Map](docs/IMPLEMENTATION.md) — what is live, module ownership, data boundaries and current gaps.
- [Architecture](docs/ARCHITECTURE.md) — Rust layers, PostgreSQL transactions, recovery and operational constraints.
- [Command Reference](docs/COMMANDS.md) — every registered prefix/slash command and current game flow.
- [Game Catalogue](docs/GAME_CATALOGUE.md) — approved original game concepts and release gates.
- [Roadmap](docs/ROADMAP.md) — completed work and next engineering slices.
- [Privacy Policy](docs/PRIVACY.md) and [Terms of Service](docs/TERMS_OF_SERVICE.md) — the current player-facing policies.
- [Fairness and Randomness](docs/FAIRNESS.md) — the target verification protocol and current randomness boundary.
- [Project Setup](docs/PROJECT_SETUP.md) — development and deployment prerequisites.
- [World Loom volumes and integration contracts](docs/atlas/README.md) — the detailed proposal catalogue and cross-system ownership rules.
- [Architecture decisions](docs/DECISIONS/README.md) — accepted decisions and their rationale.

## Product boundaries

Moonbyte is a game and community experience, not a betting or gambling product. There are no stakes, wagers, paid random rewards, cash-out or pay-to-win mechanics. Daily objectives are optional; missed days never remove progress. Players can leave or pause supported activities without punishment.
