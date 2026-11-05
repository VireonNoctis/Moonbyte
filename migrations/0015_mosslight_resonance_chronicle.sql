-- A Vault completion is a second private Chronicle page at Mosslight Grove.
-- The old per-location uniqueness was too restrictive for authored multi-stage content.
ALTER TABLE player_archive_fragments
    DROP CONSTRAINT IF EXISTS player_archive_fragments_discord_user_id_location_id_key;

ALTER TABLE player_archive_fragments
    DROP CONSTRAINT IF EXISTS player_archive_fragments_content_check;

ALTER TABLE player_archive_fragments
    ADD CONSTRAINT player_archive_fragments_content_check CHECK (
        (fragment_id = 'lantern_first_light' AND location_id = 'lantern-square')
        OR (fragment_id = 'mosslight_old_stones' AND location_id = 'mosslight-grove')
        OR (fragment_id = 'mosslight_resonance_vault' AND location_id = 'mosslight-grove')
        OR (fragment_id = 'glasswater_tide_notes' AND location_id = 'glasswater-pier')
        OR (fragment_id = 'ember_storykeepers' AND location_id = 'ember-archive')
        OR (fragment_id = 'starfall_night_chart' AND location_id = 'starfall-observatory')
    );

CREATE INDEX IF NOT EXISTS player_archive_fragments_location_history_idx
    ON player_archive_fragments (discord_user_id, location_id, unlocked_at ASC);
