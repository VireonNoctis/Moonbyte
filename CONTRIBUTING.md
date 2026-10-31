# Contributing to Moonbyte

Thanks for helping build Moonbyte.

## Read first

1. [Product vision](docs/PRODUCT_VISION.md)
2. [Privacy policy](docs/PRIVACY.md)
3. [Architecture](docs/ARCHITECTURE.md)
4. [Game catalogue](docs/GAME_CATALOGUE.md)
5. [Lunar migration plan](docs/MIGRATION_FROM_LUNAR.md)
6. [Roadmap](docs/ROADMAP.md)
7. [Engineering instructions](AGENTS.md)

Moonbyte is a separate Rust Discord bot. Lunar is a source of audited game concepts, not a runtime dependency. The accepted stack is Rust, Tokio, Serenity, Poise, and SQLite through SQLx. PostgreSQL and ScyllaDB are excluded.

## Development workflow

1. Inspect the current implementation, tests, issues, and recent commits.
2. Describe the intended user-visible behavior and the smallest complete approach.
3. Keep game rules independent from Discord types.
4. Add tests for state transitions, validation, access checks, concurrency, and recovery.
5. Update docs when public behavior, data handling, or architectural decisions change.
6. Run formatting, linting, tests, and builds relevant to the change.
7. Review the complete diff for unrelated edits and secrets.

## Quality expectations

- Rust code is formatted with rustfmt and passes Clippy without unexplained warnings.
- Game logic is deterministic under injected test clocks and random sources.
- Multiplayer actions validate membership, permissions, and turn ownership.
- Persistence changes include SQLite migrations and migration tests.
- Privacy changes include export/deletion, retention, guild-isolation, and log-redaction tests as applicable.
- Discord operations respect rate limits, interaction expiry, and permissions.
- Pixel-art assets are original or have documented rights for their intended use.
- No betting, wagering, staking, paid random rolls, loot-box monetisation, or pay-to-win mechanics.
- Do not add manipulative engagement mechanics or obstruct a user's ability to stop playing.

## Pull requests

Explain the problem, design, and behavior change. Include screenshots or sample Discord interactions when visual behavior changes. List the checks run and any checks not run. Call out database migrations, permission changes, configuration changes, privacy implications, and compatibility considerations.

Keep changes focused. A separate follow-up issue is preferable to pulling unrelated ideas into a large change.
