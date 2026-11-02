CREATE TABLE IF NOT EXISTS player_atlas_discoveries (
    discord_user_id TEXT NOT NULL CHECK (length(discord_user_id) BETWEEN 1 AND 32),
    location_id TEXT NOT NULL CHECK (
        location_id IN (
            'lantern-square',
            'mosslight-grove',
            'glasswater-pier',
            'ember-archive',
            'starfall-observatory'
        )
    ),
    discovered_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_visited_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    visit_count BIGINT NOT NULL DEFAULT 1 CHECK (visit_count >= 1),
    PRIMARY KEY (discord_user_id, location_id)
);

CREATE INDEX IF NOT EXISTS player_atlas_discoveries_history_idx
    ON player_atlas_discoveries (discord_user_id, discovered_at ASC, location_id);

CREATE TABLE IF NOT EXISTS player_world_state (
    discord_user_id TEXT PRIMARY KEY CHECK (length(discord_user_id) BETWEEN 1 AND 32),
    current_location_id TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT player_world_state_current_discovery_fk
        FOREIGN KEY (discord_user_id, current_location_id)
        REFERENCES player_atlas_discoveries (discord_user_id, location_id)
        ON DELETE CASCADE
        DEFERRABLE INITIALLY DEFERRED
);
