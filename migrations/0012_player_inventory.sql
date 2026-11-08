CREATE TABLE IF NOT EXISTS player_inventory_items (
    discord_user_id TEXT NOT NULL REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    item_id TEXT NOT NULL CHECK (length(item_id) BETWEEN 1 AND 120),
    quantity INTEGER NOT NULL DEFAULT 1 CHECK (quantity > 0),
    source_type TEXT NOT NULL CHECK (length(source_type) BETWEEN 1 AND 40),
    source_reference TEXT NOT NULL CHECK (length(source_reference) BETWEEN 1 AND 160),
    acquired_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (discord_user_id, item_id, source_type, source_reference)
);

CREATE INDEX IF NOT EXISTS player_inventory_items_player_acquired_idx
    ON player_inventory_items (discord_user_id, acquired_at ASC);

-- Backfill existing Atlas discoveries so current players receive their postcards.
INSERT INTO player_inventory_items
    (discord_user_id, item_id, quantity, source_type, source_reference, acquired_at)
SELECT
    discord_user_id,
    'atlas.postcard.' || location_id,
    1,
    'atlas_discovery',
    location_id,
    discovered_at
FROM player_atlas_discoveries
WHERE location_id IN (
    'lantern-square',
    'mosslight-grove',
    'glasswater-pier',
    'ember-archive',
    'starfall-observatory'
)
ON CONFLICT (discord_user_id, item_id, source_type, source_reference) DO NOTHING;
