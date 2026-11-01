CREATE SEQUENCE IF NOT EXISTS moonbyte_session_id_seq AS BIGINT START WITH 1;

CREATE TABLE IF NOT EXISTS game_sessions (
    session_id TEXT PRIMARY KEY,
    game_id TEXT NOT NULL,
    guild_id TEXT,
    phase TEXT NOT NULL CHECK (phase IN ('waiting for players', 'active', 'completed', 'ended')),
    state_version SMALLINT NOT NULL CHECK (state_version > 0),
    action_count BIGINT NOT NULL DEFAULT 0 CHECK (action_count >= 0),
    session_state JSONB NOT NULL CHECK (jsonb_typeof(session_state) = 'object'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS game_sessions_expiry_idx
    ON game_sessions (expires_at);

CREATE INDEX IF NOT EXISTS game_sessions_guild_phase_idx
    ON game_sessions (guild_id, phase, updated_at DESC);
