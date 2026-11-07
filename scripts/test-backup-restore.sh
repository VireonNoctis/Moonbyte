#!/usr/bin/env bash
set -Eeuo pipefail

: "${DATABASE_URL:?DATABASE_URL must point to the disposable CI source database}"
: "${MOONBYTE_RESTORE_TEST_DATABASE_URL:?Set an isolated empty restore-test database URL}"

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
backup_script="$script_dir/backup-postgres.sh"
restore_script="$script_dir/restore-postgres.sh"
restore_database="moonbyte_restore_test"
temporary_dir="$(mktemp -d)"
target_created=false
probe_created=false

cleanup() {
    if [[ "$target_created" == true ]]; then
        dropdb --if-exists --maintenance-db="$DATABASE_URL" "$restore_database" >/dev/null 2>&1 || true
    fi
    if [[ "$probe_created" == true ]]; then
        psql --no-psqlrc --set=ON_ERROR_STOP=1 --dbname="$DATABASE_URL" \
            --command="DROP TABLE IF EXISTS public.moonbyte_backup_restore_probe" >/dev/null 2>&1 || true
    fi
    rm -rf -- "$temporary_dir"
}
trap cleanup EXIT

source_database="$(psql --no-psqlrc --tuples-only --no-align --set=ON_ERROR_STOP=1 \
    --dbname="$DATABASE_URL" --command="SELECT current_database()")"
target_database="$(psql --no-psqlrc --tuples-only --no-align --set=ON_ERROR_STOP=1 \
    --dbname="$MOONBYTE_RESTORE_TEST_DATABASE_URL" --command="SELECT current_database()")"
[[ "$target_database" == "$restore_database" ]] || {
    echo "Restore drill target must be the disposable $restore_database database." >&2
    exit 1
}
[[ "$source_database" != "$target_database" ]] || {
    echo "Source and restore-test databases must be different." >&2
    exit 1
}

psql --no-psqlrc --set=ON_ERROR_STOP=1 --dbname="$DATABASE_URL" \
    --command="CREATE TABLE public.moonbyte_backup_restore_probe (id INTEGER PRIMARY KEY, marker TEXT NOT NULL); INSERT INTO public.moonbyte_backup_restore_probe VALUES (1, 'moonbyte-backup-restore-ok')"
probe_created=true

export MOONBYTE_BACKUP_DIR="$temporary_dir/backups"
mkdir -p -- "$MOONBYTE_BACKUP_DIR"
backup_output="$("$backup_script")"
backup_file="$(printf '%s\n' "$backup_output" | sed -n 's/^Backup created: //p')"
[[ -n "$backup_file" && -s "$backup_file" && -s "$backup_file.sha256" ]] || {
    echo "Backup script did not publish a complete archive and checksum." >&2
    exit 1
}
[[ "$(stat -c '%a' "$backup_file")" == "600" ]] || {
    echo "Backup archive permissions must be 600." >&2
    exit 1
}
[[ "$(stat -c '%a' "$backup_file.sha256")" == "600" ]] || {
    echo "Backup checksum permissions must be 600." >&2
    exit 1
}

dropdb --if-exists --maintenance-db="$DATABASE_URL" "$restore_database" >/dev/null
createdb --maintenance-db="$DATABASE_URL" "$restore_database"
target_created=true

valid_checksum="$temporary_dir/valid.sha256"
cp -- "$backup_file.sha256" "$valid_checksum"
printf 'not-a-checksum\n' > "$backup_file.sha256"
if DATABASE_URL="$MOONBYTE_RESTORE_TEST_DATABASE_URL" "$restore_script" "$backup_file" --confirm-empty-database >/dev/null 2>&1; then
    echo "Restore accepted a malformed checksum manifest." >&2
    exit 1
fi
mv -- "$valid_checksum" "$backup_file.sha256"

DATABASE_URL="$MOONBYTE_RESTORE_TEST_DATABASE_URL" \
    "$restore_script" "$backup_file" --confirm-empty-database

restored_marker="$(psql --no-psqlrc --tuples-only --no-align --set=ON_ERROR_STOP=1 \
    --dbname="$MOONBYTE_RESTORE_TEST_DATABASE_URL" \
    --command="SELECT marker FROM public.moonbyte_backup_restore_probe WHERE id = 1")"
[[ "$restored_marker" == "moonbyte-backup-restore-ok" ]] || {
    echo "Restored data did not match the source probe." >&2
    exit 1
}

if DATABASE_URL="$MOONBYTE_RESTORE_TEST_DATABASE_URL" "$restore_script" "$backup_file" --confirm-empty-database >/dev/null 2>&1; then
    echo "Restore did not refuse a non-empty target database." >&2
    exit 1
fi

printf 'PostgreSQL backup/restore round-trip passed (%s -> %s).\n' "$source_database" "$target_database"
