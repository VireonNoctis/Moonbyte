#!/usr/bin/env bash
set -Eeuo pipefail
umask 077

: "${DATABASE_URL:?DATABASE_URL must be set to a PostgreSQL connection URL}"
command -v pg_dump >/dev/null 2>&1 || { echo "pg_dump is required." >&2; exit 1; }
command -v pg_restore >/dev/null 2>&1 || { echo "pg_restore is required." >&2; exit 1; }
command -v sha256sum >/dev/null 2>&1 || { echo "sha256sum is required." >&2; exit 1; }

backup_dir="${MOONBYTE_BACKUP_DIR:-./backups}"
mkdir -p -- "$backup_dir"
chmod 700 "$backup_dir"

timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
backup_file="$backup_dir/moonbyte-$timestamp.dump"
temporary_file="$(mktemp "$backup_dir/.moonbyte-backup.XXXXXX")"
cleanup() {
    rm -f -- "$temporary_file"
}
trap cleanup EXIT

if [[ -e "$backup_file" || -e "$backup_file.sha256" ]]; then
    echo "A backup with this timestamp already exists; refusing to overwrite it." >&2
    exit 1
fi

pg_dump --format=custom --no-owner --no-acl --file="$temporary_file" "$DATABASE_URL"
pg_restore --list "$temporary_file" >/dev/null
chmod 600 "$temporary_file"
mv -- "$temporary_file" "$backup_file"
sha256sum "$backup_file" | awk '{print $1}' > "$backup_file.sha256"
chmod 600 "$backup_file.sha256"

printf 'Backup created: %s\n' "$backup_file"
printf 'Checksum: %s.sha256\n' "$backup_file"
