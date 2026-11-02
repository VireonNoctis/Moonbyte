CREATE TABLE IF NOT EXISTS player_profiles (
    discord_user_id TEXT PRIMARY KEY CHECK (length(discord_user_id) BETWEEN 1 AND 32),
    display_name VARCHAR(32) NOT NULL CHECK (length(trim(display_name)) BETWEEN 1 AND 32),
    bio VARCHAR(160),
    visibility TEXT NOT NULL DEFAULT 'private'
        CHECK (visibility IN ('public', 'private')),
    show_statistics BOOLEAN NOT NULL DEFAULT TRUE,
    quiet_mode BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS player_profiles_visibility_idx
    ON player_profiles (visibility, updated_at DESC);
