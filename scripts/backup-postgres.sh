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
checksum_file="$backup_file.sha256"
temporary_file="$(mktemp "$backup_dir/.moonbyte-backup.XXXXXX")"
temporary_checksum="$(mktemp "$backup_dir/.moonbyte-checksum.XXXXXX")"
published=false

cleanup() {
    rm -f -- "$temporary_file" "$temporary_checksum"
    if [[ "$published" != true ]]; then
        rm -f -- "$backup_file" "$checksum_file"
    fi
}
trap cleanup EXIT

if [[ -e "$backup_file" || -e "$checksum_file" ]]; then
    echo "A backup with this timestamp already exists; refusing to overwrite it." >&2
    exit 1
fi

pg_dump --format=custom --no-owner --no-acl --file="$temporary_file" "$DATABASE_URL"
pg_restore --list "$temporary_file" >/dev/null
sha256sum "$temporary_file" | awk '{print $1}' > "$temporary_checksum"

checksum="$(cat "$temporary_checksum")"
[[ "$checksum" =~ ^[[:xdigit:]]{64}$ ]] || {
    echo "Could not create a valid SHA-256 checksum." >&2
    exit 1
}

chmod 600 "$temporary_file" "$temporary_checksum"
mv -- "$temporary_file" "$backup_file"
mv -- "$temporary_checksum" "$checksum_file"
published=true

printf 'Backup created: %s\n' "$backup_file"
printf 'Checksum: %s\n' "$checksum_file"
