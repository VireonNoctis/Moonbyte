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
- [x] Add Rust application workspace and commit Cargo.lock.
- [x] Add Rust formatting, test, build, and Clippy checks in CI.

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

The executable now connects to Discord through Tokio, Serenity, and Poise. It registers global slash commands at startup and supports the default period prefix. Set variables in the process environment; Moonbyte does not load .env files automatically. `DATABASE_PATH` is reserved for the future SQLite adapter and is not opened by the current runtime.

## Run the Discord bot

With `DISCORD_TOKEN` set in the process environment, run:

```bash
cargo run --locked
```

The runtime requests `GUILDS`, `GUILD_MESSAGES`, and `MESSAGE_CONTENT`. Enable Message Content in the Discord developer settings for prefix commands, and invite the application with the `applications.commands` OAuth scope for slash commands. Global slash-command registration can take up to an hour to propagate. See [Command reference](COMMANDS.md).

## Implemented checks

The workspace supports `python3 scripts/validate.py`, `cargo fmt --all -- --check`, `cargo test --workspace --locked`, and `cargo clippy --workspace --all-targets --locked -- -D warnings`. CI runs these checks with the committed lockfile.

## Remaining runtime checks

SQLite, migrations, structured logging, graceful shutdown, privacy export/deletion, and a development-guild Discord smoke test remain unimplemented or unverified. Add integration tests for reconnect, shutdown, command registration, and recoverable startup failures as those capabilities are built.
