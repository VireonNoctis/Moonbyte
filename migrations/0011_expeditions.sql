CREATE TABLE IF NOT EXISTS expedition_runs (
    expedition_run_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    route_id TEXT NOT NULL CHECK (route_id = 'lantern-road'),
    content_version SMALLINT NOT NULL CHECK (content_version > 0),
    guild_id TEXT,
    host_discord_user_id TEXT REFERENCES player_profiles(discord_user_id) ON DELETE SET NULL,
    shared BOOLEAN NOT NULL DEFAULT FALSE,
    state TEXT NOT NULL CHECK (state IN ('recruiting', 'active', 'paused', 'completed', 'cancelled')),
    current_checkpoint_index SMALLINT NOT NULL DEFAULT 0 CHECK (current_checkpoint_index BETWEEN 0 AND 3),
    row_version BIGINT NOT NULL DEFAULT 0 CHECK (row_version >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    CHECK (NOT shared OR guild_id IS NOT NULL)
);

CREATE INDEX IF NOT EXISTS expedition_runs_guild_state_idx
    ON expedition_runs (guild_id, state, created_at DESC);

CREATE TABLE IF NOT EXISTS expedition_participants (
    expedition_run_id BIGINT NOT NULL REFERENCES expedition_runs(expedition_run_id) ON DELETE CASCADE,
    discord_user_id TEXT NOT NULL REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    left_at TIMESTAMPTZ,
    last_checkpoint_index SMALLINT NOT NULL DEFAULT 0 CHECK (last_checkpoint_index BETWEEN 0 AND 3),
    PRIMARY KEY (expedition_run_id, discord_user_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS expedition_one_active_run_per_player_idx
    ON expedition_participants (discord_user_id) WHERE is_active;
CREATE INDEX IF NOT EXISTS expedition_participants_run_active_idx
    ON expedition_participants (expedition_run_id, is_active, joined_at ASC);

CREATE TABLE IF NOT EXISTS expedition_checkpoint_history (
    expedition_run_id BIGINT NOT NULL REFERENCES expedition_runs(expedition_run_id) ON DELETE CASCADE,
    content_version SMALLINT NOT NULL CHECK (content_version > 0),
    checkpoint_index SMALLINT NOT NULL CHECK (checkpoint_index BETWEEN 1 AND 3),
    checkpoint_id TEXT NOT NULL CHECK (checkpoint_id IN ('mosslight-grove', 'ember-archive', 'starfall-observatory')),
    choice_id TEXT NOT NULL CHECK (choice_id IN ('listen', 'inspect', 'star', 'lantern', 'chart')),
    completed_by TEXT REFERENCES player_profiles(discord_user_id) ON DELETE SET NULL,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (expedition_run_id, content_version, checkpoint_id),
    UNIQUE (expedition_run_id, content_version, checkpoint_index)
);

CREATE INDEX IF NOT EXISTS expedition_checkpoint_history_run_idx
    ON expedition_checkpoint_history (expedition_run_id, checkpoint_index ASC);
