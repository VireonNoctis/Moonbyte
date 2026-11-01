CREATE TABLE IF NOT EXISTS player_profiles (
    discord_user_id TEXT PRIMARY KEY
        CHECK (length(discord_user_id) BETWEEN 1 AND 20)
        CHECK (discord_user_id ~ '^[0-9]+$'),
    board_theme TEXT NOT NULL DEFAULT 'classic'
        CHECK (board_theme IN ('classic', 'forest', 'sunset', 'lavender')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
