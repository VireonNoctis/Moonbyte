//! Authoritative world map and per-player exploration records.
//!
//! This first Atlas slice uses a fixed, reviewed route graph, direct-neighbour
//! travel, persistent discoveries, and no travel rewards.

use std::fmt;

use sqlx::{PgPool, Postgres, Transaction};

const STARTING_LOCATION: &str = "lantern-square";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AtlasLocation {
    pub id: &'static str,
    pub name: &'static str,
    pub region: &'static str,
    pub description: &'static str,
    pub routes_to: &'static [&'static str],
}

pub const LOCATION_CATALOGUE: [AtlasLocation; 5] = [
    AtlasLocation {
        id: "lantern-square",
        name: "Lantern Square",
        region: "Hearthland",
        description: "The warm-lit meeting place where the known roads begin.",
        routes_to: &["mosslight-grove", "glasswater-pier"],
    },
    AtlasLocation {
        id: "mosslight-grove",
        name: "Mosslight Grove",
        region: "Verdant Reach",
        description: "A quiet woodland path threaded with old stones and soft green light.",
        routes_to: &["lantern-square", "ember-archive", "starfall-observatory"],
    },
    AtlasLocation {
        id: "glasswater-pier",
        name: "Glasswater Pier",
        region: "Western Coast",
        description: "A calm shoreline of clear water, weathered maps and distant sails.",
        routes_to: &["lantern-square", "starfall-observatory"],
    },
    AtlasLocation {
        id: "ember-archive",
        name: "The Ember Archive",
        region: "Cinder Vale",
        description: "A small archive preserving the stories of the world's old roads.",
        routes_to: &["mosslight-grove", "starfall-observatory"],
    },
    AtlasLocation {
        id: "starfall-observatory",
        name: "Starfall Observatory",
        region: "High Astral",
        description: "A hilltop observatory with a clear view of the connected lands.",
        routes_to: &["mosslight-grove", "glasswater-pier", "ember-archive"],
    },
];

#[derive(Debug)]
pub enum AtlasError {
    Database(sqlx::Error),
    UnknownLocation,
    NotReachable,
    UnknownStoredLocation,
    UnexpectedLocation,
    ExpeditionInProgress,
}

impl fmt::Display for AtlasError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(_) => {
                formatter.write_str("The Atlas is temporarily unavailable. Please try again.")
            }
            Self::UnknownLocation => {
                formatter.write_str("That location is not in the current Atlas catalogue.")
            }
            Self::NotReachable => {
                formatter.write_str("That location is not connected to your current route.")
            }
            Self::UnknownStoredLocation => formatter.write_str(
                "Your saved Atlas location is not recognised by this version of Moonbyte.",
            ),
            Self::UnexpectedLocation => formatter.write_str(
                "Your saved Atlas position no longer matches this Expedition checkpoint.",
            ),
            Self::ExpeditionInProgress => formatter.write_str(
                "Your Atlas movement is managed by an active Expedition. Pause or leave that run first.",
            ),
        }
    }
}

impl std::error::Error for AtlasError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for AtlasError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtlasDiscovery {
    pub location: AtlasLocation,
    pub discovered_at: String,
    pub last_visited_at: String,
    pub visits: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtlasSnapshot {
    pub current_location: AtlasLocation,
    pub discoveries: Vec<AtlasDiscovery>,
    pub available_routes: Vec<AtlasLocation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TravelResult {
    pub moved: bool,
    pub from: AtlasLocation,
    pub to: AtlasLocation,
    pub snapshot: AtlasSnapshot,
}

#[derive(Clone)]
pub struct AtlasStore {
    pool: PgPool,
}

impl AtlasStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Opening the Atlas initializes the player's starting point once.
    pub async fn snapshot(&self, player_id: &str) -> Result<AtlasSnapshot, AtlasError> {
        let mut transaction = self.pool.begin().await?;
        ensure_starting_point(&mut transaction, player_id).await?;
        transaction.commit().await?;
        self.load_snapshot(player_id).await
    }

    /// Travel is limited to an edge in the authored route graph.
    /// Revisiting a location is allowed but never creates a reward.
    pub async fn travel(
        &self,
        player_id: &str,
        destination: &str,
    ) -> Result<TravelResult, AtlasError> {
        let destination = find_location(destination).ok_or(AtlasError::UnknownLocation)?;
        let mut transaction = self.pool.begin().await?;
        ensure_starting_point(&mut transaction, player_id).await?;

        let current_id: String = sqlx::query_scalar(
            "SELECT current_location_id FROM player_world_state WHERE discord_user_id = $1 FOR UPDATE",
        )
        .bind(player_id)
        .fetch_one(&mut *transaction)
        .await?;
        let current = location_by_id(&current_id).ok_or(AtlasError::UnknownStoredLocation)?;

        if current.id == destination.id {
            transaction.commit().await?;
            let snapshot = self.load_snapshot(player_id).await?;
            return Ok(TravelResult {
                moved: false,
                from: current,
                to: destination,
                snapshot,
            });
        }

        let expedition_active: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM expedition_participants WHERE discord_user_id = $1 AND is_active)",
        )
        .bind(player_id)
        .fetch_one(&mut *transaction)
        .await?;
        if expedition_active {
            return Err(AtlasError::ExpeditionInProgress);
        }

        if !current.routes_to.contains(&destination.id) {
            return Err(AtlasError::NotReachable);
        }

        let already_discovered: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM player_atlas_discoveries \
             WHERE discord_user_id = $1 AND location_id = $2)",
        )
        .bind(player_id)
        .bind(destination.id)
        .fetch_one(&mut *transaction)
        .await?;

        if !already_discovered {
            crate::wayfarer::record_atlas_discovery(&mut transaction, player_id).await?;
        }

        sqlx::query(
            "INSERT INTO player_atlas_discoveries (discord_user_id, location_id, discovered_at, last_visited_at, visit_count) \
             VALUES ($1, $2, now(), now(), 1) \
             ON CONFLICT (discord_user_id, location_id) DO UPDATE SET \
                 last_visited_at = now(), visit_count = player_atlas_discoveries.visit_count + 1",
        )
        .bind(player_id)
        .bind(destination.id)
        .execute(&mut *transaction)
        .await?;

        crate::chronicle::record_discovery(&mut transaction, player_id, destination.id).await?;
        crate::inventory::record_atlas_keepsake(&mut transaction, player_id, destination.id).await?;

        sqlx::query(
            "UPDATE player_world_state SET current_location_id = $2, updated_at = now() \
             WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .bind(destination.id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;
        let snapshot = self.load_snapshot(player_id).await?;

        Ok(TravelResult {
            moved: true,
            from: current,
            to: destination,
            snapshot,
        })
    }

    async fn load_snapshot(&self, player_id: &str) -> Result<AtlasSnapshot, AtlasError> {
        let current_id: String = sqlx::query_scalar(
            "SELECT current_location_id FROM player_world_state WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let current_location =
            location_by_id(&current_id).ok_or(AtlasError::UnknownStoredLocation)?;

        let rows = sqlx::query_as::<_, (String, String, String, i64)>(
            "SELECT location_id, discovered_at::text, last_visited_at::text, visit_count \
             FROM player_atlas_discoveries WHERE discord_user_id = $1 \
             ORDER BY discovered_at ASC, location_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;

        let discoveries = rows
            .into_iter()
            .filter_map(|(location_id, discovered_at, last_visited_at, visits)| {
                location_by_id(&location_id).map(|location| AtlasDiscovery {
                    location,
                    discovered_at,
                    last_visited_at,
                    visits,
                })
            })
            .collect();

        let available_routes = current_location
            .routes_to
            .iter()
            .filter_map(|location_id| location_by_id(location_id))
            .collect();

        Ok(AtlasSnapshot {
            current_location,
            discoveries,
            available_routes,
        })
    }
}

async fn ensure_starting_point(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), AtlasError> {
    sqlx::query(
        "INSERT INTO player_atlas_discoveries (discord_user_id, location_id) \
         VALUES ($1, $2) ON CONFLICT (discord_user_id, location_id) DO NOTHING",
    )
    .bind(player_id)
    .bind(STARTING_LOCATION)
    .execute(&mut **transaction)
    .await?;

    sqlx::query(
        "INSERT INTO player_world_state (discord_user_id, current_location_id) \
         VALUES ($1, $2) ON CONFLICT (discord_user_id) DO NOTHING",
    )
    .bind(player_id)
    .bind(STARTING_LOCATION)
    .execute(&mut **transaction)
    .await?;

    crate::chronicle::record_discovery(transaction, player_id, STARTING_LOCATION).await?;
    crate::inventory::record_atlas_keepsake(transaction, player_id, STARTING_LOCATION).await?;
    Ok(())
}

/// Apply a route checkpoint inside the caller's Expedition transaction.
/// Atlas, Chronicle and Expedition history therefore commit or roll back together.
pub(crate) async fn apply_expedition_travel(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    expected_from: &str,
    destination_id: &str,
) -> Result<(), AtlasError> {
    let from = location_by_id(expected_from).ok_or(AtlasError::UnknownLocation)?;
    let destination = location_by_id(destination_id).ok_or(AtlasError::UnknownLocation)?;
    if !from.routes_to.contains(&destination.id) {
        return Err(AtlasError::NotReachable);
    }

    let current_id: String = sqlx::query_scalar(
        "SELECT current_location_id FROM player_world_state WHERE discord_user_id = $1 FOR UPDATE",
    )
    .bind(player_id)
    .fetch_one(&mut **transaction)
    .await?;
    if current_id != expected_from {
        return Err(AtlasError::UnexpectedLocation);
    }

    let already_discovered: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM player_atlas_discoveries WHERE discord_user_id = $1 AND location_id = $2)",
    )
    .bind(player_id)
    .bind(destination.id)
    .fetch_one(&mut **transaction)
    .await?;
    if !already_discovered {
        crate::wayfarer::record_atlas_discovery(transaction, player_id).await?;
    }

    sqlx::query(
        "INSERT INTO player_atlas_discoveries (discord_user_id, location_id, discovered_at, last_visited_at, visit_count)
         VALUES ($1, $2, now(), now(), 1)
         ON CONFLICT (discord_user_id, location_id) DO UPDATE SET
             last_visited_at = now(), visit_count = player_atlas_discoveries.visit_count + 1",
    )
    .bind(player_id)
    .bind(destination.id)
    .execute(&mut **transaction)
    .await?;

    crate::chronicle::record_discovery(transaction, player_id, destination.id).await?;
    crate::inventory::record_atlas_keepsake(transaction, player_id, destination.id).await?;

    sqlx::query(
        "UPDATE player_world_state SET current_location_id = $2, updated_at = now()
         WHERE discord_user_id = $1",
    )
    .bind(player_id)
    .bind(destination.id)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

fn normalise_reference(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace([' ', '_'], "-")
}

pub fn find_location(value: &str) -> Option<AtlasLocation> {
    let reference = normalise_reference(value);
    LOCATION_CATALOGUE.iter().copied().find(|location| {
        location.id == reference || normalise_reference(location.name) == reference
    })
}

pub fn location_by_id(id: &str) -> Option<AtlasLocation> {
    LOCATION_CATALOGUE
        .iter()
        .copied()
        .find(|location| location.id == id)
}

pub fn render(snapshot: &AtlasSnapshot, prefix: &str) -> String {
    let safe_prefix = prefix
        .replace('@', "@\u{200B}")
        .replace(char::from(96), "ˋ")
        .replace(char::from(92), "＼");
    let known = snapshot
        .discoveries
        .iter()
        .map(|discovery| {
            format!(
                "• **{}** — {} visit{}",
                discovery.location.name,
                discovery.visits,
                if discovery.visits == 1 { "" } else { "s" }
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let routes = snapshot
        .available_routes
        .iter()
        .map(|location| {
            let charted = snapshot
                .discoveries
                .iter()
                .any(|discovery| discovery.location.id == location.id);
            format!(
                "• **{}** · {}\n  {safe_prefix}travel {}",
                location.name,
                if charted {
                    "charted route"
                } else {
                    "new discovery"
                },
                location.id
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "**The Atlas — World Index**\n\n\
         **You are here:** {} · {}\n{}\n\n\
         **Known places** · {}/{}\n{}\n\n\
         **Routes from here**\n{}\n\n\
         Travel follows connected roads only. Revisiting a place is always allowed, \
         but it does not grant currency or repeatable rewards. Use {safe_prefix}atlas \
         at any time to reopen your map.",
        snapshot.current_location.name,
        snapshot.current_location.region,
        snapshot.current_location.description,
        snapshot.discoveries.len(),
        LOCATION_CATALOGUE.len(),
        known,
        routes
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{LOCATION_CATALOGUE, find_location, location_by_id};

    #[test]
    fn location_ids_are_unique_and_routes_are_reciprocal() {
        let ids = LOCATION_CATALOGUE.map(|location| location.id);
        let unique: HashSet<_> = ids.into_iter().collect();
        assert_eq!(unique.len(), LOCATION_CATALOGUE.len());

        for location in LOCATION_CATALOGUE {
            assert!(!location.name.is_empty());
            assert!(!location.description.is_empty());
            for route in location.routes_to {
                let neighbour = location_by_id(route).expect("route target exists");
                assert!(neighbour.routes_to.contains(&location.id));
            }
        }
    }

    #[test]
    fn every_location_is_reachable_from_the_start() {
        let mut visited = HashSet::new();
        let mut frontier = vec!["lantern-square"];
        while let Some(id) = frontier.pop() {
            if !visited.insert(id) {
                continue;
            }
            let location = location_by_id(id).expect("catalogue location");
            frontier.extend(location.routes_to.iter().copied());
        }
        assert_eq!(visited.len(), LOCATION_CATALOGUE.len());
    }

    #[test]
    fn location_lookup_accepts_stable_ids_and_display_names() {
        assert_eq!(
            find_location("mosslight-grove").unwrap().id,
            "mosslight-grove"
        );
        assert_eq!(
            find_location("Mosslight Grove").unwrap().id,
            "mosslight-grove"
        );
        assert_eq!(find_location("missing-place"), None);
    }

    #[tokio::test]
    async fn travel_records_only_connected_discoveries_and_revisits() {
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
        let player_id = format!("atlas-{nonce}");
        let store = super::AtlasStore::new(pool.clone());
        let profiles = crate::profile::ProfileStore::new(pool.clone());
        profiles
            .get_or_create(&player_id, "Atlas Test")
            .await
            .expect("create player profile");

        let initial = store.snapshot(&player_id).await.expect("open Atlas");
        assert_eq!(initial.current_location.id, "lantern-square");
        assert_eq!(initial.discoveries.len(), 1);
        let starting_pages: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_archive_fragments WHERE discord_user_id = $1",
        )
        .bind(&player_id)
        .fetch_one(&pool)
        .await
        .expect("read initial Chronicle page");
        assert_eq!(starting_pages, 1);

        assert!(matches!(
            store.travel(&player_id, "starfall-observatory").await,
            Err(super::AtlasError::NotReachable)
        ));

        let first_trip = store
            .travel(&player_id, "Mosslight Grove")
            .await
            .expect("travel");
        assert!(first_trip.moved);
        assert_eq!(first_trip.to.id, "mosslight-grove");
        assert_eq!(first_trip.snapshot.discoveries.len(), 2);
        let discovered_pages: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_archive_fragments WHERE discord_user_id = $1",
        )
        .bind(&player_id)
        .fetch_one(&pool)
        .await
        .expect("read newly discovered Chronicle page");
        assert_eq!(discovered_pages, 2);

        let return_trip = store
            .travel(&player_id, "lantern-square")
            .await
            .expect("return");
        assert!(return_trip.moved);
        assert_eq!(return_trip.to.id, "lantern-square");

        let unchanged = store
            .travel(&player_id, "Lantern Square")
            .await
            .expect("already here");
        assert!(!unchanged.moved);

        let pages_after_revisiting: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_archive_fragments WHERE discord_user_id = $1",
        )
        .bind(&player_id)
        .fetch_one(&pool)
        .await
        .expect("read Chronicle pages after revisiting known locations");
        assert_eq!(pages_after_revisiting, 2);

        let discoveries: Vec<(String, i64)> = sqlx::query_as(
            "SELECT location_id, visit_count FROM player_atlas_discoveries \
             WHERE discord_user_id = $1 ORDER BY location_id",
        )
        .bind(&player_id)
        .fetch_all(&pool)
        .await
        .expect("read discoveries");
        assert_eq!(
            discoveries,
            vec![
                (String::from("lantern-square"), 2),
                (String::from("mosslight-grove"), 1)
            ]
        );

        let deletion = profiles
            .delete_account(&player_id)
            .await
            .expect("clean up player and Atlas fixture through privacy service");
        assert!(deletion.profile_removed);
        assert!(deletion.world_state_removed);
        assert_eq!(deletion.atlas_discoveries_removed, 2);
        assert_eq!(deletion.archive_fragments_removed, 2);
    }
}
