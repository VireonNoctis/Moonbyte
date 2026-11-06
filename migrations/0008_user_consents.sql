CREATE TABLE IF NOT EXISTS user_consents (
    discord_user_id TEXT PRIMARY KEY,
    terms_version TEXT NOT NULL,
    privacy_version TEXT NOT NULL,
    accepted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    accepted_in_guild_id TEXT
);

CREATE INDEX IF NOT EXISTS user_consents_accepted_at_idx
    ON user_consents (accepted_at);
