# Moonbyte Project Setup

## Repository foundation

- [x] Repository named Moonbyte.
- [x] Product identity and independence from Lunar documented.
- [x] Rust, Tokio, Serenity, Poise, and PostgreSQL through SQLx selected.
- [x] Default prefix set to a period (.).
- [x] Full-colour pixel-art direction documented.
- [x] Product vision, architecture, catalogue, migration plan, and roadmap written.
- [x] Privacy stance, privacy command, data inventory, and retention targets documented.
- [x] Rust application workspace and committed dependency lockfile.
- [x] Rust formatting, PostgreSQL-backed tests, and Clippy checks in CI.
- [ ] Choose and add a project licence before external distribution.

## Discord application

Before running the bot:
- [ ] Create or configure the Moonbyte Discord application and bot user.
- [ ] Store the bot token in a local environment file or deployment secret store.
- [ ] Enable Message Content intent because the default prefix is a text prefix.
- [ ] Request only Gateway intents required by implemented features.
- [ ] Include the applications.commands OAuth scope when inviting the bot for slash commands.
- [ ] Test prefix and slash command registration in a development guild.

Never commit the token or share it in logs, screenshots, issues, or test fixtures.

## PostgreSQL setup

Moonbyte requires PostgreSQL before the bot starts. The application uses SQLx with PostgreSQL support only; SQLite is not a supported fallback.

For local development, start the included database service:

    docker compose up -d postgres

The development-only connection template in .env.example is:

    DATABASE_URL=postgres://moonbyte:moonbyte@localhost:5432/moonbyte

Replace the sample credentials for any shared or production environment. Configure the actual DATABASE_URL through a secret manager or process environment. Never commit a production connection URL or print it in logs.

## Local configuration

| Variable | Purpose |
| --- | --- |
| DISCORD_TOKEN | Bot token. Secret; never commit its real value. |
| DEFAULT_PREFIX | Default command prefix; a period. |
| DATABASE_URL | PostgreSQL connection URL. Secret; never log or commit production values. |
| RUST_LOG | Structured log filter. |

Moonbyte does not load .env files automatically. At startup it validates the required settings, opens a bounded SQLx connection pool, applies ordered schema migrations under a PostgreSQL advisory transaction lock, then starts the Discord client.

## Run the Discord bot

With PostgreSQL running and DISCORD_TOKEN set in the process environment:

    cargo run --locked

The runtime requests GUILDS, GUILD_MESSAGES, and MESSAGE_CONTENT. Enable Message Content in the Discord developer settings for prefix commands, and invite the application with the applications.commands OAuth scope for slash commands. Global slash-command registration can take up to an hour to propagate. See Command reference.

## Implemented checks

The workspace supports python3 scripts/validate.py, cargo fmt --all -- --check, cargo test --workspace --locked, and cargo clippy --workspace --all-targets --locked -- -D warnings. CI provisions an isolated PostgreSQL 17 service and runs the database integration tests against it.

## Remaining runtime checks

Production backup/restore, database monitoring, interaction idempotency, multi-instance load testing, structured logging, graceful shutdown, privacy export/deletion, and a development-guild Discord smoke test remain unimplemented or unverified.