//! Provenance-backed, account-owned inventory and Atlas postcard collection.
//!
//! Postcards are granted only for authored first discoveries. They are not random,
//! tradable, purchasable, or spendable. Atlas remains the authority for discovery;
//! this module owns the resulting inventory record.

use sqlx::{PgPool, Postgres, Transaction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ItemDefinition {
    pub item_id: &'static str,
    pub location_id: &'static str,
    pub location_name: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub const POSTCARD_CATALOGUE: [ItemDefinition; 5] = [
    ItemDefinition {
        item_id: "atlas.postcard.lantern-square",
        location_id: "lantern-square",
        location_name: "Lantern Square",
        title: "Lantern Square Postcard",
        description: "The warm lights of the place where every journey begins.",
    },
    ItemDefinition {
        item_id: "atlas.postcard.mosslight-grove",
        location_id: "mosslight-grove",
        location_name: "Mosslight Grove",
        title: "Mosslight Grove Postcard",
        description: "A quiet path beneath the soft glow of the old trees.",
    },
    ItemDefinition {
        item_id: "atlas.postcard.glasswater-pier",
        location_id: "glasswater-pier",
        location_name: "Glasswater Pier",
        title: "Glasswater Pier Postcard",
        description: "A clear horizon, calm water and sails in the distance.",
    },
    ItemDefinition {
        item_id: "atlas.postcard.ember-archive",
        location_id: "ember-archive",
        location_name: "The Ember Archive",
        title: "Ember Archive Postcard",
        description: "A small archive keeping the stories of old roads.",
    },
    ItemDefinition {
        item_id: "atlas.postcard.starfall-observatory",
        location_id: "starfall-observatory",
        location_name: "Starfall Observatory",
        title: "Starfall Observatory Postcard",
        description: "A hilltop view across the connected lands.",
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InventoryEntry {
    pub item_id: String,
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub source_type: String,
    pub source_reference: String,
    pub source_name: String,
    pub acquired_at: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InventorySnapshot {
    pub entries: Vec<InventoryEntry>,
}

#[derive(Clone)]
pub struct InventoryStore {
    pool: PgPool,
}

impl InventoryStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn snapshot(&self, player_id: &str) -> Result<InventorySnapshot, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, i32, String, String, String)>(
            "SELECT item_id, quantity, source_type, source_reference, acquired_at::text \
             FROM player_inventory_items WHERE discord_user_id = $1 \
             ORDER BY acquired_at ASC, item_id ASC, source_reference ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;

        let entries = rows
            .into_iter()
            .map(|(item_id, quantity, source_type, source_reference, acquired_at)| {
                let definition = definition_by_id(&item_id);
                InventoryEntry {
                    title: definition
                        .map(|item| item.title)
                        .unwrap_or("Uncatalogued item")
                        .to_owned(),
                    description: definition
                        .map(|item| item.description)
                        .unwrap_or("This item is retained with its original source record.")
                        .to_owned(),
                    source_name: definition
                        .map(|item| item.location_name)
                        .unwrap_or(source_reference.as_str())
                        .to_owned(),
                    item_id,
                    quantity,
                    source_type,
                    source_reference,
                    acquired_at,
                }
            })
            .collect();

        Ok(InventorySnapshot { entries })
    }
}

/// Record one Atlas postcard inside the caller's discovery transaction.
pub(crate) async fn record_atlas_keepsake(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    location_id: &str,
) -> Result<bool, sqlx::Error> {
    let Some(definition) = definition_for_location(location_id) else {
        return Ok(false);
    };

    let inserted = sqlx::query(
        "INSERT INTO player_inventory_items \
         (discord_user_id, item_id, quantity, source_type, source_reference) \
         VALUES ($1, $2, 1, 'atlas_discovery', $3) \
         ON CONFLICT (discord_user_id, item_id, source_type, source_reference) DO NOTHING",
    )
    .bind(player_id)
    .bind(definition.item_id)
    .bind(definition.location_id)
    .execute(&mut **transaction)
    .await?
    .rows_affected();

    Ok(inserted > 0)
}

pub fn definition_by_id(item_id: &str) -> Option<ItemDefinition> {
    POSTCARD_CATALOGUE
        .iter()
        .copied()
        .find(|item| item.item_id == item_id)
}

pub fn definition_for_location(location_id: &str) -> Option<ItemDefinition> {
    POSTCARD_CATALOGUE
        .iter()
        .copied()
        .find(|item| item.location_id == location_id)
}

pub fn render_inventory(snapshot: &InventorySnapshot) -> String {
    if snapshot.entries.is_empty() {
        return String::from(
            "**Your Satchel**\n\nNo keepsakes yet. Discover a new place through the Atlas to collect its postcard.",
        );
    }

    let entries = snapshot
        .entries
        .iter()
        .map(|entry| {
            format!(
                "🖼️ **{}** · ×{}\n{}\nFound at: {}\nAcquired: {}",
                entry.title, entry.quantity, entry.description, entry.source_name, entry.acquired_at
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "**Your Satchel** · {} item record{}\n\n{}\n\nKeepsakes are account-bound. Trading and crafting are not enabled.",
        snapshot.entries.len(),
        if snapshot.entries.len() == 1 { "" } else { "s" },
        entries
    )
}

pub fn render_collection(snapshot: &InventorySnapshot) -> String {
    let collected = POSTCARD_CATALOGUE
        .iter()
        .filter(|definition| {
            snapshot
                .entries
                .iter()
                .any(|entry| entry.item_id.as_str() == definition.item_id)
        })
        .count();

    let entries = POSTCARD_CATALOGUE
        .iter()
        .map(|definition| {
            let found = snapshot
                .entries
                .iter()
                .any(|entry| entry.item_id.as_str() == definition.item_id);
            format!(
                "{} **{}**\n{}",
                if found { "✅" } else { "▫️" },
                definition.title,
                if found { "Collected" } else { "Not discovered yet" }
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "**The Postcard Wall** · {collected}/{} collected\n\n{}\n\nEach postcard is earned once by discovering its location. There are no random drops.",
        POSTCARD_CATALOGUE.len(),
        entries
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{POSTCARD_CATALOGUE, definition_by_id, definition_for_location};

    #[test]
    fn postcard_catalogue_is_stable_and_covers_each_atlas_location_once() {
        let item_ids: HashSet<_> = POSTCARD_CATALOGUE
            .map(|item| item.item_id)
            .into_iter()
            .collect();
        let locations: HashSet<_> = POSTCARD_CATALOGUE
            .map(|item| item.location_id)
            .into_iter()
            .collect();
        assert_eq!(item_ids.len(), POSTCARD_CATALOGUE.len());
        assert_eq!(locations.len(), POSTCARD_CATALOGUE.len());
        for item in POSTCARD_CATALOGUE {
            assert!(!item.title.is_empty());
            assert!(!item.description.is_empty());
            assert_eq!(definition_by_id(item.item_id), Some(item));
            assert_eq!(definition_for_location(item.location_id), Some(item));
        }
    }
}

#[cfg(test)]
mod postgres_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::database::Database;

    use super::{InventoryStore, record_atlas_keepsake};

    #[tokio::test]
    async fn atlas_keepsakes_are_idempotent_and_account_deletion_removes_them() {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must point to the CI PostgreSQL service");
        let database = Database::connect(&database_url)
            .await
            .expect("connect and migrate PostgreSQL");
        let pool = database.pool();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
            % 1_000_000_000_000_000_000;
        let player_id = format!("inventory-test-{nonce}");
        let profiles = crate::profile::ProfileStore::new(pool.clone());
        profiles
            .get_or_create(&player_id, "Inventory Test")
            .await
            .expect("create player profile");

        let mut transaction = pool.begin().await.expect("begin discovery transaction");
        assert!(
            record_atlas_keepsake(&mut transaction, &player_id, "lantern-square")
                .await
                .expect("award first postcard")
        );
        assert!(
            !record_atlas_keepsake(&mut transaction, &player_id, "lantern-square")
                .await
                .expect("repeat postcard award")
        );
        transaction.commit().await.expect("commit discovery");

        let inventory = InventoryStore::new(pool);
        let snapshot = inventory.snapshot(&player_id).await.expect("read inventory");
        assert_eq!(snapshot.entries.len(), 1);
        assert_eq!(snapshot.entries[0].item_id, "atlas.postcard.lantern-square");
        assert_eq!(snapshot.entries[0].source_reference, "lantern-square");

        let export = profiles
            .export_json(&player_id)
            .await
            .expect("export inventory provenance");
        assert!(export.contains("account-bound Atlas keepsakes and acquisition provenance"));
        assert!(export.contains("atlas.postcard.lantern-square"));

        let deletion = profiles
            .delete_account(&player_id)
            .await
            .expect("delete inventory with account");
        assert_eq!(deletion.inventory_items_removed, 1);
        assert!(
            inventory
                .snapshot(&player_id)
                .await
                .expect("read deleted inventory")
                .entries
                .is_empty()
        );
    }
}
