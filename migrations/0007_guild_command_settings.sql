CREATE TABLE IF NOT EXISTS guild_command_settings (
    guild_id TEXT PRIMARY KEY,
    prefix TEXT NOT NULL
        CHECK (char_length(prefix) BETWEEN 1 AND 8 AND prefix !~ '[[:space:]]'),
    command_channel_id TEXT,
    updated_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS guild_command_settings_channel_idx
    ON guild_command_settings (command_channel_id)
    WHERE command_channel_id IS NOT NULL;
