# Moonbyte Operations Runbook

> **Document version: 1.0.0 · Pre-launch source project**
>
> This repository contains bot source, not a registered or deployed Discord application. No production PostgreSQL service, scheduled backup job, monitoring target or live restore has been configured. Do not treat CI as deployment verification.

## 1. Runtime contract

- One Rust process using Tokio, Serenity and Poise.
- PostgreSQL is the only supported application database.
- The process runs ordered migrations at startup under a PostgreSQL advisory transaction lock.
- A single cleanup task removes expired game sessions every 15 minutes. This is not a general database retention or backup service.
- The supported deployment shape remains one bot process plus one PostgreSQL database. Do not horizontally scale the bot until session locking, interaction delivery and guild event behavior have been load-tested.

## 2. Required configuration

Set these values in the deployment secret manager, never in a committed file:

| Variable | Purpose |
|---|---|
| `DISCORD_TOKEN` | Token for the registered Discord bot application. |
| `DATABASE_URL` | PostgreSQL connection URL. Use TLS and a least-privilege database role in deployment. |
| `DEFAULT_PREFIX` | Optional default prefix; defaults to `.`. |
| `RUST_LOG` | Optional tracing filter. The default is intentionally conservative. |

The application cannot start without `DISCORD_TOKEN` and `DATABASE_URL`. Create the Discord application, enable only the required intents and configure the invite permissions before attempting a live test. Message Content intent is required by the current prefix-command setup.

## 3. Database backups

The repository provides a manual custom-format backup script:

```sh
DATABASE_URL='postgresql://…' MOONBYTE_BACKUP_DIR='/secure/backups' ./scripts/backup-postgres.sh
```

The script:
- Uses `pg_dump` custom format with owner/ACL metadata excluded.
- Writes a restrictive-permission file and a SHA-256 sidecar.
- Validates that the archive can be listed before publishing it.
- Refuses to overwrite a same-timestamp backup.

Store backups off the bot host, encrypt them at rest and restrict read access. The script does not schedule backups or upload them to remote storage. Before public deployment, choose a schedule and retention policy; a daily backup is a reasonable initial baseline, with an additional backup before schema changes. Define the actual recovery point and recovery time objectives from the hosting provider's capabilities rather than promising them in advance.

## 4. Restore drill

Restore only into a newly created, empty database. The script verifies the checksum and refuses to run when the target's `public` schema already contains tables, views or sequences.

```sh
DATABASE_URL='postgresql://…' ./scripts/restore-postgres.sh /secure/backups/moonbyte-YYYYMMDDTHHMMSSZ.dump --confirm-empty-database
```

Use a database with the same supported PostgreSQL major version as the source. Keep the original database untouched while validating the restored copy. After restoration, start a test process against the restored database and verify migrations, profile reads, inventory, game sessions and privacy deletion. Record the measured restore duration and any data loss relative to the backup timestamp.

## 5. Health checks and logs

- The `.ping` command measures command responsiveness and executes `SELECT 1` through the application pool. It reports database reachability and a coarse latency; it does not prove Discord API health, restore readiness or full gameplay correctness.
- Startup, shutdown, migration readiness and session cleanup emit minimised structured tracing events.
- Do not log Discord tokens, database URLs, private player payloads, raw message content or fairness seeds.
- Production log retention, alert routing, uptime monitoring and an incident contact are not configured by this repository.

## 6. Pre-launch checklist

- [ ] Register the Discord application and bot; keep the token in a secret manager.
- [ ] Invite it to a private development server using least-privilege permissions and required intents.
- [ ] Provision PostgreSQL with TLS, restricted network access, backups and a named owner.
- [ ] Configure scheduled off-host backups and alert on failures.
- [ ] Restore a backup into a new empty database and measure recovery.
- [ ] Verify Terms/Privacy consent, prefix and slash commands, game buttons, restart recovery, guild isolation and account deletion in Discord.
- [ ] Review legal documents, data processors, retention, support contact and incident process before inviting public users.

## 7. Incident handling

1. Stop or isolate the bot if state integrity or credential exposure is suspected.
2. Preserve minimised logs and the current database snapshot; do not publish secrets or private records in an issue.
3. Rotate exposed Discord/database credentials through their respective providers.
4. Restore only into a separate empty database first. Validate the restored state before any production cutover.
5. Document the cause, affected systems, recovery point and user-facing impact.
