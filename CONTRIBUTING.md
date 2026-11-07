# Contributing to Moonbyte

Thanks for helping build Moonbyte.

## Read first

1. [Final vision](docs/MOONBYTE_FINAL_VISION.md)
2. [Implementation map](docs/IMPLEMENTATION.md)
3. [Privacy policy](docs/PRIVACY.md)
4. [Architecture](docs/ARCHITECTURE.md)
5. [Game catalogue](docs/GAME_CATALOGUE.md)
6. [Roadmap](docs/ROADMAP.md)
7. [Engineering instructions](AGENTS.md)

Moonbyte is a standalone Rust Discord bot. Its accepted stack is Rust, Tokio, Serenity, Poise and PostgreSQL through SQLx. Discord commands, messages, embeds, buttons, select menus and modals are the player interface; a website or browser client is outside the product scope.

## Development workflow

1. Inspect the current implementation, tests, issues and recent commits.
2. Describe the intended user-visible behavior and the smallest complete approach.
3. Keep game rules independent from Discord types.
4. Add tests for state transitions, validation, access checks, concurrency and recovery.
5. Update the canonical documents when public behavior, data handling or architecture changes.
6. Run formatting, linting, tests and builds relevant to the change.
7. Review the complete diff for unrelated edits and secrets.

## Quality expectations

- Rust code is formatted with rustfmt and passes Clippy without unexplained warnings.
- External GitHub Actions use full commit SHAs; Docker-based actions use full SHA-256 image digests.
- Game logic is deterministic under injected test clocks and random sources.
- Multiplayer actions validate membership, permissions and turn ownership.
- Persistence changes include PostgreSQL migrations, transaction tests and restart-recovery tests.
- Privacy changes include export/deletion, retention, guild-isolation and log-redaction tests as applicable.
- Discord operations respect rate limits, interaction expiry and permissions.
- Pixel-art assets are original or have documented rights for their intended use.
- No betting, wagering, staking, paid random rolls, loot-box monetisation or pay-to-win mechanics.
- Do not add manipulative engagement mechanics or obstruct a player's ability to stop playing.

## Pull requests

Explain the problem, design and behavior change. Include screenshots or sample Discord interactions when visual behavior changes. List the checks run and any checks not run. Call out database migrations, permission changes, configuration changes, privacy implications and compatibility considerations.

Keep changes focused. A separate follow-up issue is preferable to pulling unrelated ideas into a large change.
