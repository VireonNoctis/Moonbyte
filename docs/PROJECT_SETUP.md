# Project Setup

## Requirements

- The pinned Rust toolchain in `rust-toolchain.toml` with rustfmt and Clippy.
- PostgreSQL. The included Compose file provides a local development service.
- A Discord application with a bot token.
- Docker Compose for the local database workflow (or an existing PostgreSQL instance).

## Start PostgreSQL

```bash
docker compose up -d postgres
```

The sample development connection string in `.env.example` is:

```text
DATABASE_URL=postgres://moonbyte:moonbyte@localhost:5432/moonbyte
```

Use different credentials and restricted network access outside local development. Never commit a real database URL.

## Configure Discord

Set the following environment variables in the process environment:

| Variable | Purpose |
|---|---|
| `DISCORD_TOKEN` | Bot token. Secret. |
| `DATABASE_URL` | PostgreSQL connection URL. Secret. |
| `DEFAULT_PREFIX` | Optional command prefix; defaults to a period. |
| `RUST_LOG` | Optional structured log filter. Avoid verbose SQLx logging in production. |

Moonbyte does not load `.env` automatically. Enable the Message Content privileged intent if prefix commands are required. Invite the bot with the `bot` and `applications.commands` scopes.

## Run

```bash
cargo run --locked
```

Startup validates configuration, connects to PostgreSQL, applies ordered migrations and starts the Discord gateway. Ctrl-C and Unix SIGTERM request a graceful shard shutdown.

## Validate

```bash
python3 scripts/validate.py
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

CI provisions an isolated PostgreSQL service and runs repository validation, formatting, tests and Clippy. These automated checks do not replace a development-guild smoke test or production backup/restore rehearsal.

## Operational checklist

- Keep the Discord token and database URL in a secret manager or protected process environment.
- Restrict PostgreSQL network access and database privileges.
- Schedule encrypted database backups with documented retention and restore testing.
- Verify host/backup encryption properties before making encryption-at-rest claims.
- Monitor process restarts, database health, Discord rate limits and backup completion.
- Review [Privacy Policy](PRIVACY.md) before public deployment.
