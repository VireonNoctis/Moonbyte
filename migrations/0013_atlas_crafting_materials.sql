-- Fixed, source-verified crafting materials are granted by first Atlas discovery.
-- Existing discoveries receive the same material during this idempotent backfill.
INSERT INTO player_inventory_items
    (discord_user_id, item_id, quantity, source_type, source_reference, acquired_at)
SELECT
    discord_user_id,
    CASE location_id
        WHEN 'mosslight-grove' THEN 'material.mosslight-silk'
        WHEN 'glasswater-pier' THEN 'material.tideglass'
    END,
    1,
    'atlas_discovery',
    location_id,
    discovered_at
FROM player_atlas_discoveries
WHERE location_id IN ('mosslight-grove', 'glasswater-pier')
ON CONFLICT (discord_user_id, item_id, source_type, source_reference) DO NOTHING;
