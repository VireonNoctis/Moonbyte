CREATE TABLE IF NOT EXISTS player_archive_fragments (
    discord_user_id TEXT NOT NULL,
    fragment_id TEXT NOT NULL,
    location_id TEXT NOT NULL,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (discord_user_id, fragment_id),
    UNIQUE (discord_user_id, location_id),
    CONSTRAINT player_archive_fragments_content_check CHECK (
        (fragment_id = 'lantern_first_light' AND location_id = 'lantern-square')
        OR (fragment_id = 'mosslight_old_stones' AND location_id = 'mosslight-grove')
        OR (fragment_id = 'glasswater_tide_notes' AND location_id = 'glasswater-pier')
        OR (fragment_id = 'ember_storykeepers' AND location_id = 'ember-archive')
        OR (fragment_id = 'starfall_night_chart' AND location_id = 'starfall-observatory')
    ),
    CONSTRAINT player_archive_fragments_discovery_fk
        FOREIGN KEY (discord_user_id, location_id)
        REFERENCES player_atlas_discoveries (discord_user_id, location_id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS player_archive_fragments_history_idx
    ON player_archive_fragments (discord_user_id, unlocked_at ASC, fragment_id);

INSERT INTO player_archive_fragments (discord_user_id, fragment_id, location_id)
SELECT discord_user_id,
       CASE location_id
           WHEN 'lantern-square' THEN 'lantern_first_light'
           WHEN 'mosslight-grove' THEN 'mosslight_old_stones'
           WHEN 'glasswater-pier' THEN 'glasswater_tide_notes'
           WHEN 'ember-archive' THEN 'ember_storykeepers'
           WHEN 'starfall-observatory' THEN 'starfall_night_chart'
       END,
       location_id
FROM player_atlas_discoveries
ON CONFLICT DO NOTHING;
