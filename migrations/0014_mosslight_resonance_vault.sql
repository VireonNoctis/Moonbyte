-- Durable, private checkpoints for the authored Mosslight Resonance blueprint.
CREATE TABLE IF NOT EXISTS player_vault_runs (
    vault_run_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    discord_user_id TEXT NOT NULL
        REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    blueprint_id TEXT NOT NULL CHECK (blueprint_id = 'mosslight-resonance'),
    content_version SMALLINT NOT NULL CHECK (content_version > 0),
    state TEXT NOT NULL CHECK (state IN ('active', 'paused', 'left', 'completed')),
    current_room_index SMALLINT NOT NULL DEFAULT 0
        CHECK (current_room_index BETWEEN 0 AND 3),
    row_version BIGINT NOT NULL DEFAULT 0 CHECK (row_version >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ,
    CHECK (
        (state = 'completed' AND current_room_index = 3 AND completed_at IS NOT NULL)
        OR (state <> 'completed' AND current_room_index < 3 AND completed_at IS NULL)
    )
);

-- A player can have only one resumable run for a given authored version.
CREATE UNIQUE INDEX IF NOT EXISTS player_vault_one_open_run_idx
    ON player_vault_runs (discord_user_id, blueprint_id, content_version)
    WHERE state IN ('active', 'paused', 'left');
CREATE INDEX IF NOT EXISTS player_vault_runs_player_history_idx
    ON player_vault_runs (discord_user_id, created_at DESC, vault_run_id DESC);

CREATE TABLE IF NOT EXISTS player_vault_checkpoints (
    vault_run_id BIGINT NOT NULL
        REFERENCES player_vault_runs(vault_run_id) ON DELETE CASCADE,
    room_id TEXT NOT NULL CHECK (
        room_id IN ('waystone-order', 'tideglass-code', 'keepers-seal')
    ),
    room_index SMALLINT NOT NULL CHECK (room_index BETWEEN 0 AND 2),
    solved_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (vault_run_id, room_id),
    UNIQUE (vault_run_id, room_index),
    CHECK (
        (room_id = 'waystone-order' AND room_index = 0)
        OR (room_id = 'tideglass-code' AND room_index = 1)
        OR (room_id = 'keepers-seal' AND room_index = 2)
    )
);

CREATE INDEX IF NOT EXISTS player_vault_checkpoints_run_idx
    ON player_vault_checkpoints (vault_run_id, room_index ASC);
