# Moonbyte Project Setup

## Repository foundation

- [x] Repository named Moonbyte.
- [x] Product identity and independence from Lunar documented.
- [x] Rust, Tokio, Serenity, Poise, and SQLite selected.
- [x] Default prefix set to a period (.).
- [x] Full-colour pixel-art direction documented.
- [x] Product vision, architecture, catalogue, migration plan, and roadmap written.
- [x] Privacy stance, privacy command, data inventory, and retention targets documented.
- [ ] Choose and add a project licence before external distribution.
- [ ] Add Rust application workspace and commit Cargo.lock.
- [ ] Add Rust-specific CI checks.

## Discord application

Before running the bot:
- [ ] Create or configure the Moonbyte Discord application and bot user.
- [ ] Store the bot token in a local environment file or deployment secret store.
- [ ] Enable Message Content intent because the default prefix is a text prefix.
- [ ] Request only Gateway intents required by implemented features.
- [ ] Include the applications.commands OAuth scope when inviting the bot for slash commands.
- [ ] Test prefix and slash command registration in a development guild.

Never commit the token or share it in logs, screenshots, issues, or test fixtures.

## Local configuration

Environment variable names are listed in .env.example:

| Variable | Purpose |
| --- | --- |
| DISCORD_TOKEN | Bot token. Secret; never commit its real value. |
| DEFAULT_PREFIX | Default command prefix; a period. |
| DATABASE_PATH | Path to SQLite database file. |
| RUST_LOG | Structured log filter. |

The executable application and runtime-specific setup commands have not yet been added. The example environment file does not mean the bot already runs.

## First implementation checks

When the Rust workspace is introduced, add reproducible commands and CI jobs for rustfmt, Clippy, unit/integration tests, build with committed lockfile, SQLite migration validation, privacy export/deletion tests, and a local Discord smoke test that does not expose credentials.
