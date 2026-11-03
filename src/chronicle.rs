//! Private archive pages unlocked by first-time Atlas discoveries.

use sqlx::{PgPool, Postgres, Transaction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChronicleFragmentDefinition {
    pub id: &'static str,
    pub location_id: &'static str,
    pub location_name: &'static str,
    pub region: &'static str,
    pub title: &'static str,
    pub lore: &'static str,
}

pub const CHRONICLE_FRAGMENTS: [ChronicleFragmentDefinition; 5] = [
    ChronicleFragmentDefinition {
        id: "lantern_first_light",
        location_id: "lantern-square",
        location_name: "Lantern Square",
        region: "Hearthland",
        title: "The First Light",
        lore: "Before the roads had names, a keeper lit one lantern and left it burning for every traveller who had not yet found the way home.",
    },
    ChronicleFragmentDefinition {
        id: "mosslight_old_stones",
        location_id: "mosslight-grove",
        location_name: "Mosslight Grove",
        region: "Verdant Reach",
        title: "The Stones That Remember",
        lore: "The oldest stones are said to remember every footstep. The grove keeps their stories beneath a patient layer of green.",
    },
    ChronicleFragmentDefinition {
        id: "glasswater_tide_notes",
        location_id: "glasswater-pier",
        location_name: "Glasswater Pier",
        region: "Western Coast",
        title: "The Tidekeeper's Measure",
        lore: "A tidekeeper once marked the passing water, not to hold it still, but to remind the coast that every journey has its own rhythm.",
    },
    ChronicleFragmentDefinition {
        id: "ember_storykeepers",
        location_id: "ember-archive",
        location_name: "The Ember Archive",
        region: "Cinder Vale",
        title: "A Page Kept Warm",
        lore: "The Archive's first keeper believed a story should be carried, not sealed away. Even its oldest pages were left close to the hearth.",
    },
    ChronicleFragmentDefinition {
        id: "starfall_night_chart",
        location_id: "starfall-observatory",
        location_name: "Starfall Observatory",
        region: "High Astral",
        title: "A Map for Returning",
        lore: "The observatory's chartmaker mapped the stars so travellers could find their way back to one another, not so anyone could predict tomorrow.",
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChronicleEntry {
    pub fragment: ChronicleFragmentDefinition,
    pub unlocked_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChronicleSnapshot {
    pub entries: Vec<ChronicleEntry>,
}

#[derive(Clone)]
pub struct ChronicleStore {
    pool: PgPool,
}

impl ChronicleStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn snapshot(&self, player_id: &str) -> Result<ChronicleSnapshot, sqlx::Error> {
        let unlocked = sqlx::query_as::<_, (String, String)>(
            "SELECT fragment_id, unlocked_at::text FROM player_archive_fragments \
             WHERE discord_user_id = $1 ORDER BY unlocked_at ASC, fragment_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;

        let entries = CHRONICLE_FRAGMENTS
            .iter()
            .copied()
            .map(|fragment| ChronicleEntry {
                fragment,
                unlocked_at: unlocked
                    .iter()
                    .find(|(fragment_id, _)| fragment_id.as_str() == fragment.id)
                    .map(|(_, unlocked_at)| unlocked_at.clone()),
            })
            .collect();

        Ok(ChronicleSnapshot { entries })
    }
}

/// Called in the Atlas transaction after the matching discovery row exists.
pub(crate) async fn record_discovery(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    location_id: &str,
) -> Result<(), sqlx::Error> {
    let fragment = CHRONICLE_FRAGMENTS
        .iter()
        .find(|fragment| fragment.location_id == location_id)
        .expect("every authored Atlas location must have a Chronicle page");

    sqlx::query(
        "INSERT INTO player_archive_fragments (discord_user_id, fragment_id, location_id) \
         VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
    )
    .bind(player_id)
    .bind(fragment.id)
    .bind(fragment.location_id)
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

pub fn render(snapshot: &ChronicleSnapshot, prefix: &str) -> String {
    let safe_prefix = prefix
        .replace('@', "@\u{200B}")
        .replace(char::from(96), "ˋ")
        .replace('\\', "＼");
    let unlocked = snapshot
        .entries
        .iter()
        .filter(|entry| entry.unlocked_at.is_some())
        .count();
    let pages = snapshot
        .entries
        .iter()
        .map(|entry| match entry.unlocked_at.as_deref() {
            Some(_) => format!(
                "📖 **{}** · {} ({})\n{}",
                entry.fragment.title,
                entry.fragment.location_name,
                entry.fragment.region,
                entry.fragment.lore
            ),
            None => format!(
                "◇ **Unwritten page** · {} ({})\nDiscover this place in the Atlas to uncover its story.",
                entry.fragment.location_name, entry.fragment.region
            ),
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "**The Chronicle — Pages of the World**\n\n\
         Discovered pages: **{unlocked}/{}**\n\
         Every first-time Atlas discovery records one page in your private collection. \
         Revisits do not create duplicates or repeatable rewards.\n\n\
         {pages}\n\n\
         Open {safe_prefix}atlas to chart a connected route.",
        snapshot.entries.len()
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::CHRONICLE_FRAGMENTS;
    use crate::atlas::LOCATION_CATALOGUE;

    #[test]
    fn every_authored_atlas_location_has_one_unique_chronicle_page() {
        let ids = CHRONICLE_FRAGMENTS
            .iter()
            .map(|fragment| fragment.id)
            .collect::<HashSet<_>>();
        let locations = CHRONICLE_FRAGMENTS
            .iter()
            .map(|fragment| fragment.location_id)
            .collect::<HashSet<_>>();

        assert_eq!(ids.len(), CHRONICLE_FRAGMENTS.len());
        assert_eq!(locations.len(), CHRONICLE_FRAGMENTS.len());
        assert_eq!(CHRONICLE_FRAGMENTS.len(), LOCATION_CATALOGUE.len());

        for location in LOCATION_CATALOGUE {
            let matches = CHRONICLE_FRAGMENTS
                .iter()
                .filter(|fragment| fragment.location_id == location.id)
                .count();
            assert_eq!(
                matches, 1,
                "missing or duplicated page for {}",
                location.id
            );
        }
    }

    #[test]
    fn authored_pages_have_complete_player_facing_content() {
        for fragment in CHRONICLE_FRAGMENTS {
            assert!(!fragment.title.trim().is_empty());
            assert!(!fragment.lore.trim().is_empty());
            assert!(!fragment.region.trim().is_empty());
        }
    }
}
