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

pub const MATERIAL_CATALOGUE: [ItemDefinition; 2] = [
    ItemDefinition {
        item_id: "material.mosslight-silk",
        location_id: "mosslight-grove",
        location_name: "Mosslight Grove",
        title: "Mosslight Silk",
        description: "A soft luminous thread gathered from the grove's old woven roots.",
    },
    ItemDefinition {
        item_id: "material.tideglass",
        location_id: "glasswater-pier",
        location_name: "Glasswater Pier",
        title: "Tideglass",
        description: "A smooth, sea-polished glass fragment from the western shore.",
    },
];

pub const CRAFTED_CATALOGUE: [ItemDefinition; 2] = [
    ItemDefinition {
        item_id: "keepsake.wayfarer-lantern",
        location_id: "wayfarer-workshop",
        location_name: "Wayfarer Workshop",
        title: "Wayfarer's Lantern",
        description: "A small, account-bound keepsake that holds a warm light for the road.",
    },
    ItemDefinition {
        item_id: "keepsake.mosslight-seal",
        location_id: "mosslight-grove",
        location_name: "Mosslight Grove",
        title: "Mosslight Seal",
        description: "A fixed, account-bound keepsake awarded for completing Mosslight Resonance.",
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeIngredient {
    pub item_id: &'static str,
    pub quantity: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CraftingRecipe {
    pub recipe_id: &'static str,
    pub output_item_id: &'static str,
    pub ingredients: [RecipeIngredient; 2],
}

pub const CRAFTING_RECIPES: [CraftingRecipe; 1] = [CraftingRecipe {
    recipe_id: "wayfarer-lantern",
    output_item_id: "keepsake.wayfarer-lantern",
    ingredients: [
        RecipeIngredient {
            item_id: "material.mosslight-silk",
            quantity: 1,
        },
        RecipeIngredient {
            item_id: "material.tideglass",
            quantity: 1,
        },
    ],
}];

/// A presentation-ready slash autocomplete option backed by the real recipe catalogue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeSuggestion {
    pub label: String,
    pub recipe_id: String,
}

/// Return all fixed recipes with their real output and ingredient names.
/// Discord autocomplete filters this list; the stored choice value stays the canonical ID.
pub fn crafting_recipe_options() -> Vec<RecipeSuggestion> {
    CRAFTING_RECIPES
        .iter()
        .filter_map(|recipe| {
            let output = definition_by_id(recipe.output_item_id)?;
            let ingredients = recipe
                .ingredients
                .iter()
                .map(|ingredient| {
                    let definition = definition_by_id(ingredient.item_id)?;
                    Some(format!("{}× {}", ingredient.quantity, definition.title))
                })
                .collect::<Option<Vec<_>>>()?
                .join(" + ");

            Some(RecipeSuggestion {
                label: format!("{} · {}", output.title, ingredients),
                recipe_id: recipe.recipe_id.to_owned(),
            })
        })
        .collect()
}

/// Resolve either the canonical recipe ID or its exact display title.
/// Fuzzy guesses are intentionally handled by the command's separate suggestion helper.
pub fn resolve_recipe_id(input: &str) -> Option<&'static str> {
    let input = input.trim();
    CRAFTING_RECIPES.iter().find_map(|recipe| {
        let output = definition_by_id(recipe.output_item_id)?;
        (recipe.recipe_id.eq_ignore_ascii_case(input) || output.title.eq_ignore_ascii_case(input))
            .then_some(recipe.recipe_id)
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CraftOutcome {
    Crafted,
    AlreadyCrafted,
    MissingIngredients,
    UnknownRecipe,
}

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
            .map(
                |(item_id, quantity, source_type, source_reference, acquired_at)| {
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
                },
            )
            .collect();

        Ok(InventorySnapshot { entries })
    }

    /// Craft a fixed, account-bound recipe in one serialised PostgreSQL transaction.
    pub async fn craft_recipe(
        &self,
        player_id: &str,
        recipe_id: &str,
    ) -> Result<CraftOutcome, sqlx::Error> {
        let Some(recipe) = recipe_by_id(recipe_id) else {
            return Ok(CraftOutcome::UnknownRecipe);
        };

        let mut transaction = self.pool.begin().await?;
        sqlx::query(
            "SELECT pg_advisory_xact_lock(hashtextextended('moonbyte:inventory:' || $1, 0))",
        )
        .bind(player_id)
        .execute(&mut *transaction)
        .await?;

        let already_crafted: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM player_inventory_items
                WHERE discord_user_id = $1
                  AND item_id = $2
                  AND source_type = 'craft_recipe'
                  AND source_reference = $3
            )",
        )
        .bind(player_id)
        .bind(recipe.output_item_id)
        .bind(recipe.recipe_id)
        .fetch_one(&mut *transaction)
        .await?;

        if already_crafted {
            transaction.commit().await?;
            return Ok(CraftOutcome::AlreadyCrafted);
        }

        for ingredient in recipe.ingredients {
            let available: i64 = sqlx::query_scalar(
                "SELECT COALESCE(SUM(quantity), 0)::BIGINT
                 FROM player_inventory_items
                 WHERE discord_user_id = $1 AND item_id = $2",
            )
            .bind(player_id)
            .bind(ingredient.item_id)
            .fetch_one(&mut *transaction)
            .await?;

            if available < i64::from(ingredient.quantity) {
                transaction.rollback().await?;
                return Ok(CraftOutcome::MissingIngredients);
            }
        }

        for ingredient in recipe.ingredients {
            consume_inventory_item(
                &mut transaction,
                player_id,
                ingredient.item_id,
                ingredient.quantity,
            )
            .await?;
        }

        sqlx::query(
            "INSERT INTO player_inventory_items
             (discord_user_id, item_id, quantity, source_type, source_reference)
             VALUES ($1, $2, 1, 'craft_recipe', $3)",
        )
        .bind(player_id)
        .bind(recipe.output_item_id)
        .bind(recipe.recipe_id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;
        Ok(CraftOutcome::Crafted)
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

/// Record one fixed material from an Atlas discovery in the caller's transaction.
pub(crate) async fn record_atlas_material(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    location_id: &str,
) -> Result<bool, sqlx::Error> {
    let Some(definition) = MATERIAL_CATALOGUE
        .iter()
        .copied()
        .find(|item| item.location_id == location_id)
    else {
        return Ok(false);
    };

    let inserted = sqlx::query(
        "INSERT INTO player_inventory_items
         (discord_user_id, item_id, quantity, source_type, source_reference)
         VALUES ($1, $2, 1, 'atlas_discovery', $3)
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

/// Grant the fixed Vault keepsake once per player. The source identifies the
/// completing run and blueprint version; the per-player advisory lock is held
/// by the Vault service before this helper is called.
pub(crate) async fn record_vault_keepsake(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    run_id: i64,
    blueprint_version: i16,
) -> Result<bool, sqlx::Error> {
    let source_reference = format!("vault-run:{run_id}:mosslight-resonance:v{blueprint_version}");
    let inserted = sqlx::query(
        "INSERT INTO player_inventory_items
         (discord_user_id, item_id, quantity, source_type, source_reference)
         SELECT $1, 'keepsake.mosslight-seal', 1, 'vault_completion', $2
         WHERE NOT EXISTS (
             SELECT 1 FROM player_inventory_items
             WHERE discord_user_id = $1 AND item_id = 'keepsake.mosslight-seal'
         )
         ON CONFLICT (discord_user_id, item_id, source_type, source_reference) DO NOTHING",
    )
    .bind(player_id)
    .bind(source_reference)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    Ok(inserted > 0)
}

async fn consume_inventory_item(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    item_id: &str,
    required_quantity: i32,
) -> Result<(), sqlx::Error> {
    let sources = sqlx::query_as::<_, (String, String, i32)>(
        "SELECT source_type, source_reference, quantity
         FROM player_inventory_items
         WHERE discord_user_id = $1 AND item_id = $2
         ORDER BY acquired_at ASC, source_type ASC, source_reference ASC
         FOR UPDATE",
    )
    .bind(player_id)
    .bind(item_id)
    .fetch_all(&mut **transaction)
    .await?;

    let mut remaining = required_quantity;
    for (source_type, source_reference, quantity) in sources {
        if remaining == 0 {
            break;
        }

        let consumed = remaining.min(quantity);
        if consumed == quantity {
            sqlx::query(
                "DELETE FROM player_inventory_items
                 WHERE discord_user_id = $1 AND item_id = $2
                   AND source_type = $3 AND source_reference = $4",
            )
            .bind(player_id)
            .bind(item_id)
            .bind(&source_type)
            .bind(&source_reference)
            .execute(&mut **transaction)
            .await?;
        } else {
            sqlx::query(
                "UPDATE player_inventory_items SET quantity = quantity - $5
                 WHERE discord_user_id = $1 AND item_id = $2
                   AND source_type = $3 AND source_reference = $4",
            )
            .bind(player_id)
            .bind(item_id)
            .bind(&source_type)
            .bind(&source_reference)
            .bind(consumed)
            .execute(&mut **transaction)
            .await?;
        }
        remaining -= consumed;
    }

    debug_assert_eq!(
        remaining, 0,
        "validated recipe inputs must still be available"
    );
    Ok(())
}

pub fn recipe_by_id(recipe_id: &str) -> Option<CraftingRecipe> {
    CRAFTING_RECIPES
        .iter()
        .copied()
        .find(|recipe| recipe.recipe_id == recipe_id)
}

pub fn definition_by_id(item_id: &str) -> Option<ItemDefinition> {
    POSTCARD_CATALOGUE
        .iter()
        .chain(MATERIAL_CATALOGUE.iter())
        .chain(CRAFTED_CATALOGUE.iter())
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
                entry.title,
                entry.quantity,
                entry.description,
                entry.source_name,
                entry.acquired_at
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "**Your Satchel** · {} item record{}\n\n{}\n\nAtlas materials and crafted keepsakes are account-bound. Trading is not enabled.",
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
                if found {
                    "Collected"
                } else {
                    "Not discovered yet"
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "**The Postcard Wall** · {collected}/{} collected\n\n{}\n\nEach postcard is earned once by discovering its location. The Workshop uses fixed recipes and verified materials; there are no random drops.",
        POSTCARD_CATALOGUE.len(),
        entries
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{
        CRAFTED_CATALOGUE, CRAFTING_RECIPES, MATERIAL_CATALOGUE, POSTCARD_CATALOGUE,
        crafting_recipe_options, definition_by_id, definition_for_location, recipe_by_id,
        resolve_recipe_id,
    };

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

        for item in MATERIAL_CATALOGUE.into_iter().chain(CRAFTED_CATALOGUE) {
            assert!(!item.title.is_empty());
            assert!(!item.description.is_empty());
            assert_eq!(definition_by_id(item.item_id), Some(item));
        }

        assert_eq!(recipe_by_id("wayfarer-lantern"), Some(CRAFTING_RECIPES[0]));
        assert_eq!(recipe_by_id("unknown"), None);
        assert_eq!(
            resolve_recipe_id("wayfarer-lantern"),
            Some("wayfarer-lantern")
        );
        assert_eq!(
            resolve_recipe_id("Wayfarer's Lantern"),
            Some("wayfarer-lantern")
        );
        assert_eq!(resolve_recipe_id("wayfarer-lanern"), None);

        let suggestions = crafting_recipe_options();
        assert_eq!(suggestions.len(), CRAFTING_RECIPES.len());
        assert_eq!(suggestions[0].recipe_id, "wayfarer-lantern");
        assert!(suggestions[0].label.contains("Wayfarer's Lantern"));
        assert!(suggestions[0].label.contains("Mosslight Silk"));
        assert!(suggestions[0].label.contains("Tideglass"));
    }
}

#[cfg(test)]
mod postgres_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::database::Database;

    use super::{CraftOutcome, InventoryStore, record_atlas_keepsake, record_atlas_material};

    #[tokio::test]
    async fn fixed_recipe_consumes_verified_inputs_once_and_preserves_output_provenance() {
        use std::time::{SystemTime, UNIX_EPOCH};

        use crate::database::Database;

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
        let player_id = format!("craft-{nonce}");
        let profiles = crate::profile::ProfileStore::new(pool.clone());
        crate::profile::create_test_player(pool.clone(), &player_id, "Craft Test")
            .await
            .expect("create consented player profile");

        let mut transaction = pool.begin().await.expect("begin material grant");
        assert!(
            record_atlas_material(&mut transaction, &player_id, "mosslight-grove")
                .await
                .expect("grant Mosslight material")
        );
        assert!(
            record_atlas_material(&mut transaction, &player_id, "glasswater-pier")
                .await
                .expect("grant Tideglass material")
        );
        assert!(
            !record_atlas_material(&mut transaction, &player_id, "glasswater-pier")
                .await
                .expect("duplicate material grant is ignored")
        );
        transaction.commit().await.expect("commit material grant");

        let inventory = InventoryStore::new(pool.clone());
        assert_eq!(
            inventory
                .craft_recipe(&player_id, "wayfarer-lantern")
                .await
                .expect("craft recipe"),
            CraftOutcome::Crafted
        );
        assert_eq!(
            inventory
                .craft_recipe(&player_id, "wayfarer-lantern")
                .await
                .expect("repeat recipe"),
            CraftOutcome::AlreadyCrafted
        );

        let snapshot = inventory
            .snapshot(&player_id)
            .await
            .expect("read crafted inventory");
        assert_eq!(snapshot.entries.len(), 1);
        assert_eq!(snapshot.entries[0].item_id, "keepsake.wayfarer-lantern");
        assert_eq!(snapshot.entries[0].source_type, "craft_recipe");
        assert_eq!(snapshot.entries[0].source_reference, "wayfarer-lantern");

        let export = profiles
            .export_json(&player_id)
            .await
            .expect("export crafted provenance");
        assert!(export.contains("keepsake.wayfarer-lantern"));

        let deletion = profiles
            .delete_account(&player_id)
            .await
            .expect("delete crafted inventory with account");
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
        let player_id = format!("inv-{nonce}");
        let profiles = crate::profile::ProfileStore::new(pool.clone());
        crate::profile::create_test_player(pool.clone(), &player_id, "Inventory Test")
            .await
            .expect("create consented player profile");

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
        let snapshot = inventory
            .snapshot(&player_id)
            .await
            .expect("read inventory");
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
