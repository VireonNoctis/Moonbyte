CREATE TABLE IF NOT EXISTS player_vault_runs (
    vault_run_id BIGSERIAL PRIMARY KEY,
    discord_user_id TEXT NOT NULL REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    blueprint_id TEXT NOT NULL CHECK (length(blueprint_id) BETWEEN 1 AND 80),
    content_version SMALLINT NOT NULL CHECK (content_version > 0),
    state TEXT NOT NULL CHECK (state IN ('active', 'paused', 'completed', 'left')),
    current_room_index SMALLINT NOT NULL DEFAULT 0 CHECK (current_room_index BETWEEN 0 AND 3),
    row_version BIGINT NOT NULL DEFAULT 0 CHECK (row_version >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ,
    CHECK ((state = 'completed') = (completed_at IS NOT NULL)),
    CHECK (state <> 'completed' OR current_room_index = 3)
);

CREATE UNIQUE INDEX IF NOT EXISTS player_vault_runs_one_open_blueprint_idx
    ON player_vault_runs (discord_user_id, blueprint_id, content_version)
    WHERE state IN ('active', 'paused', 'left');

CREATE INDEX IF NOT EXISTS player_vault_runs_player_history_idx
    ON player_vault_runs (discord_user_id, created_at DESC, vault_run_id DESC);

CREATE TABLE IF NOT EXISTS player_vault_checkpoints (
    vault_run_id BIGINT NOT NULL REFERENCES player_vault_runs(vault_run_id) ON DELETE CASCADE,
    room_id TEXT NOT NULL CHECK (length(room_id) BETWEEN 1 AND 80),
    room_index SMALLINT NOT NULL CHECK (room_index BETWEEN 0 AND 2),
    solved_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (vault_run_id, room_id),
    UNIQUE (vault_run_id, room_index)
);

CREATE INDEX IF NOT EXISTS player_vault_checkpoints_run_order_idx
    ON player_vault_checkpoints (vault_run_id, room_index);
