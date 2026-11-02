//! Persistent player identity and privacy controls.
//!
//! Only fields required by the player-facing profile are stored. Discord remains
//! the identity provider; Moonbyte does not request email, real name, or location.

use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug)]
pub enum ProfileError {
    Database(sqlx::Error),
    Serialization(serde_json::Error),
    InvalidDisplayName,
    InvalidBio,
    InvalidVisibility,
}

impl fmt::Display for ProfileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(_) => formatter.write_str("The player store is temporarily unavailable."),
            Self::Serialization(_) => formatter.write_str("The player data could not be exported."),
            Self::InvalidDisplayName => formatter.write_str("Display names must contain 1 to 32 visible characters."),
            Self::InvalidBio => formatter.write_str("Profile bios must be 160 characters or fewer."),
            Self::InvalidVisibility => formatter.write_str("Visibility must be either public or private."),
        }
    }
}

impl std::error::Error for ProfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for ProfileError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<serde_json::Error> for ProfileError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub discord_user_id: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub visibility: String,
    pub show_statistics: bool,
    pub quiet_mode: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileInventory {
    pub profile_record: bool,
    pub game_session_records: i64,
    pub session_retention: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedSession {
    session_id: String,
    game_id: String,
    phase: String,
    action_count: i64,
    created_at: String,
    updated_at: String,
    expires_at: String,
}

#[derive(Debug, Serialize)]
struct PlayerExport {
    format_version: u16,
    exported_at: String,
    data_categories: [&'static str; 2],
    profile: Option<PlayerProfile>,
    game_sessions: Vec<ExportedSession>,
    limitations: [&'static str; 2],
}

/// Owns the persistent player profile boundary. Game rules never depend on it.
#[derive(Clone)]
pub struct ProfileStore {
    pool: PgPool,
}

impl ProfileStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_or_create(
        &self,
        player_id: &str,
        default_name: &str,
    ) -> Result<PlayerProfile, ProfileError> {
        let display_name = normalise_display_name(default_name)?;
        sqlx::query(
            "INSERT INTO player_profiles (discord_user_id, display_name) \
             VALUES ($1, $2) ON CONFLICT (discord_user_id) DO NOTHING",
        )
        .bind(player_id)
        .bind(display_name)
        .execute(&self.pool)
        .await?;
        self.get(player_id).await?.ok_or_else(|| {
            ProfileError::Database(sqlx::Error::RowNotFound)
        })
    }

    pub async fn get(&self, player_id: &str) -> Result<Option<PlayerProfile>, ProfileError> {
        let row = sqlx::query_as::<_, (String, String, Option<String>, String, bool, bool, String, String)>(
            "SELECT discord_user_id, display_name, bio, visibility, show_statistics, quiet_mode, \
             created_at::text, updated_at::text FROM player_profiles WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(discord_user_id, display_name, bio, visibility, show_statistics, quiet_mode, created_at, updated_at)| {
            PlayerProfile {
                discord_user_id,
                display_name,
                bio,
                visibility,
                show_statistics,
                quiet_mode,
                created_at,
                updated_at,
            }
        }))
    }

    pub async fn update_identity(
        &self,
        player_id: &str,
        display_name: &str,
        bio: &str,
    ) -> Result<PlayerProfile, ProfileError> {
        let display_name = normalise_display_name(display_name)?;
        let bio = normalise_bio(bio)?;
        sqlx::query(
            "UPDATE player_profiles SET display_name = $2, bio = $3, updated_at = now() \
             WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .bind(display_name)
        .bind(bio)
        .execute(&self.pool)
        .await?;
        self.get(player_id).await?.ok_or_else(|| {
            ProfileError::Database(sqlx::Error::RowNotFound)
        })
    }

    pub async fn update_preferences(
        &self,
        player_id: &str,
        visibility: &str,
        show_statistics: bool,
        quiet_mode: bool,
    ) -> Result<PlayerProfile, ProfileError> {
        let visibility = match visibility.trim().to_ascii_lowercase().as_str() {
            "public" => "public",
            "private" => "private",
            _ => return Err(ProfileError::InvalidVisibility),
        };
        sqlx::query(
            "UPDATE player_profiles SET visibility = $2, show_statistics = $3, \
             quiet_mode = $4, updated_at = now() WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .bind(visibility)
        .bind(show_statistics)
        .bind(quiet_mode)
        .execute(&self.pool)
        .await?;
        self.get(player_id).await?.ok_or_else(|| {
            ProfileError::Database(sqlx::Error::RowNotFound)
        })
    }

    pub async fn inventory(&self, player_id: &str) -> Result<ProfileInventory, ProfileError> {
        let profile_record: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM player_profiles WHERE discord_user_id = $1)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        // Include expired-but-not-yet-cleaned rows: they still physically exist.
        let game_session_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM game_sessions \
             WHERE (session_state -> 'players') @> jsonb_build_array($1::text)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(ProfileInventory {
            profile_record,
            game_session_records,
            session_retention: "Sessions expire 24 hours after their last successful action; cleanup runs every 15 minutes.",
        })
    }

    pub async fn export_json(&self, player_id: &str) -> Result<String, ProfileError> {
        let profile = self.get(player_id).await?;
        let rows = sqlx::query_as::<_, (String, String, String, i64, String, String, String)>(
            "SELECT session_id, game_id, phase, action_count, created_at::text, \
             updated_at::text, expires_at::text FROM game_sessions \
             WHERE (session_state -> 'players') @> jsonb_build_array($1::text) \
             ORDER BY created_at DESC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;

        let exported_at: String = sqlx::query_scalar("SELECT now()::text")
            .fetch_one(&self.pool)
            .await?;

        let game_sessions = rows
            .into_iter()
            .map(|(session_id, game_id, phase, action_count, created_at, updated_at, expires_at)| {
                ExportedSession {
                    session_id,
                    game_id,
                    phase,
                    action_count,
                    created_at,
                    updated_at,
                    expires_at,
                }
            })
            .collect();

        let export = PlayerExport {
            format_version: 1,
            exported_at,
            data_categories: ["player profile and preferences", "game session metadata"],
            profile,
            game_sessions,
            limitations: [
                "Discord independently retains interactions under its own policies.",
                "This export includes records in the current Moonbyte PostgreSQL schema only.",
            ],
        };
        Ok(serde_json::to_string_pretty(&export)?)
    }

    /// Deletes the profile and every stored session snapshot containing this player.
    /// Ending shared sessions is intentional: the snapshot contains every participant ID.
    pub async fn delete_account(&self, player_id: &str) -> Result<(u64, bool), ProfileError> {
        let mut transaction = self.pool.begin().await?;
        let sessions = sqlx::query(
            "DELETE FROM game_sessions \
             WHERE (session_state -> 'players') @> jsonb_build_array($1::text)",
        )
        .bind(player_id)
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        let profile = sqlx::query("DELETE FROM player_profiles WHERE discord_user_id = $1")
            .bind(player_id)
            .execute(&mut *transaction)
            .await?
            .rows_affected()
            > 0;
        transaction.commit().await?;
        Ok((sessions, profile))
    }
}

fn normalise_display_name(value: &str) -> Result<String, ProfileError> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > 32 || value.chars().any(char::is_control) {
        return Err(ProfileError::InvalidDisplayName);
    }
    Ok(value.to_owned())
}

fn normalise_bio(value: &str) -> Result<Option<String>, ProfileError> {
    let value = value.trim();
    if value.chars().count() > 160 || value.chars().any(char::is_control) {
        return Err(ProfileError::InvalidBio);
    }
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::{ProfileError, normalise_bio, normalise_display_name};

    #[test]
    fn display_name_requires_one_to_thirty_two_visible_characters() {
        assert_eq!(normalise_display_name("Moon Explorer").unwrap(), "Moon Explorer");
        assert!(matches!(normalise_display_name("  "), Err(ProfileError::InvalidDisplayName)));
        assert!(matches!(normalise_display_name(&"x".repeat(33)), Err(ProfileError::InvalidDisplayName)));
        assert!(matches!(normalise_display_name("Moon\nByte"), Err(ProfileError::InvalidDisplayName)));
    }

    #[test]
    fn bio_is_optional_and_bounded() {
        assert_eq!(normalise_bio(" ").unwrap(), None);
        assert_eq!(normalise_bio("Finding tiny worlds").unwrap().as_deref(), Some("Finding tiny worlds"));
        assert!(matches!(normalise_bio(&"x".repeat(161)), Err(ProfileError::InvalidBio)));
        assert!(matches!(normalise_bio("line one\nline two"), Err(ProfileError::InvalidBio)));
    }
}


#[cfg(test)]
mod postgres_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        database::Database,
        game::{GameKind, GameRuntime},
    };

    use super::ProfileStore;

    #[tokio::test]
    async fn profile_export_and_deletion_cover_current_player_records() {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must point to the CI PostgreSQL service");
        let database = Database::connect(&database_url)
            .await
            .expect("connect and migrate PostgreSQL");
        let store = ProfileStore::new(database.pool());
        let runtime = GameRuntime::new(database.pool());
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let player_id = format!("profile-test-{}-{nonce}", std::process::id());

        let profile = store
            .get_or_create(&player_id, "Moon Explorer")
            .await
            .expect("create profile");
        assert_eq!(profile.visibility, "private");

        let profile = store
            .update_identity(&player_id, "Moonbyte Pilot", "Finding small worlds")
            .await
            .expect("update identity");
        assert_eq!(profile.display_name, "Moonbyte Pilot");
        assert_eq!(profile.bio.as_deref(), Some("Finding small worlds"));

        let profile = store
            .update_preferences(&player_id, "public", false, true)
            .await
            .expect("save preferences");
        assert_eq!(profile.visibility, "public");
        assert!(!profile.show_statistics);
        assert!(profile.quiet_mode);

        let session = runtime
            .start(GameKind::MemoryMatch, &player_id, None)
            .await
            .expect("create linked session");
        let inventory = store.inventory(&player_id).await.expect("read inventory");
        assert!(inventory.profile_record);
        assert_eq!(inventory.game_session_records, 1);

        let export = store.export_json(&player_id).await.expect("export data");
        assert!(export.contains("Moonbyte Pilot"));
        assert!(export.contains(&session.id));
        assert!(export.contains("game session metadata"));

        let (sessions_removed, profile_removed) = store
            .delete_account(&player_id)
            .await
            .expect("delete player records");
        assert_eq!(sessions_removed, 1);
        assert!(profile_removed);
        assert!(store.get(&player_id).await.expect("read deleted profile").is_none());
        assert_eq!(
            store.inventory(&player_id).await.expect("read deleted inventory").game_session_records,
            0
        );
        assert!(
            runtime.get(&session.id, &player_id, None).await.is_err(),
            "deleted session state must no longer be recoverable"
        );
    }
}
