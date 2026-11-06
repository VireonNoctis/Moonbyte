#!/usr/bin/env bash
set -Eeuo pipefail
umask 077

usage() {
    echo "Usage: $0 BACKUP_FILE --confirm-empty-database" >&2
    exit 2
}

[[ $# -eq 2 ]] || usage
backup_file="$1"
confirmation="$2"
[[ "$confirmation" == "--confirm-empty-database" ]] || usage

: "${DATABASE_URL:?DATABASE_URL must be set to the target PostgreSQL connection URL}"
command -v psql >/dev/null 2>&1 || { echo "psql is required." >&2; exit 1; }
command -v pg_restore >/dev/null 2>&1 || { echo "pg_restore is required." >&2; exit 1; }
command -v sha256sum >/dev/null 2>&1 || { echo "sha256sum is required." >&2; exit 1; }

[[ -f "$backup_file" && -r "$backup_file" ]] || {
    echo "Backup file is missing or unreadable." >&2
    exit 1
}
checksum_file="$backup_file.sha256"
[[ -f "$checksum_file" && -r "$checksum_file" ]] || {
    echo "Checksum manifest is required; create backups with backup-postgres.sh." >&2
    exit 1
}

expected_checksum="$(awk 'NR == 1 { print $1 }' "$checksum_file")"
actual_checksum="$(sha256sum "$backup_file" | awk '{print $1}')"
[[ -n "$expected_checksum" && "$expected_checksum" == "$actual_checksum" ]] || {
    echo "Backup checksum verification failed." >&2
    exit 1
}
pg_restore --list "$backup_file" >/dev/null

existing_objects="$(psql --no-psqlrc --tuples-only --no-align --set=ON_ERROR_STOP=1 \
    --dbname="$DATABASE_URL" \
    --command="SELECT count(*) FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace WHERE n.nspname = 'public' AND c.relkind IN ('r', 'p', 'v', 'm', 'S', 'f')")"

[[ "$existing_objects" == "0" ]] || {
    echo "Target database is not empty; restore refused. Use a newly created empty database." >&2
    exit 1
}

pg_restore --single-transaction --exit-on-error --no-owner --no-acl \
    --dbname="$DATABASE_URL" "$backup_file"

echo "Restore completed into the verified empty database."
