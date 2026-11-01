use std::{fmt, time::{SystemTime, UNIX_EPOCH}};

use sqlx::PgPool;

/// A small, non-gameplay preference saved to a player's account.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardTheme {
    Classic,
    Forest,
    Sunset,
    Lavender,
}

impl BoardTheme {
    pub const ALL: [Self; 4] = [Self::Classic, Self::Forest, Self::Sunset, Self::Lavender];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Forest => "forest",
            Self::Sunset => "sunset",
            Self::Lavender => "lavender",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::Forest => "Forest",
            Self::Sunset => "Sunset",
            Self::Lavender => "Lavender",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "classic" => Some(Self::Classic),
            "forest" => Some(Self::Forest),
            "sunset" => Some(Self::Sunset),
            "lavender" => Some(Self::Lavender),
            _ => None,
        }
    }

    fn parse_stored(value: &str) -> Result<Self, ProfileError> {
        Self::parse(value).ok_or(ProfileError::InvalidStoredTheme)
    }
}

impl fmt::Display for BoardTheme {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

#[derive(Debug)]
pub enum ProfileError {
    InvalidDiscordUserId,
    InvalidStoredTheme,
    Database(sqlx::Error),
}

impl fmt::Display for ProfileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDiscordUserId => formatter.write_str("The player reference is invalid."),
            Self::InvalidStoredTheme => formatter.write_str("The saved profile preference is invalid."),
            Self::Database(_) => formatter.write_str("The player profile store is temporarily unavailable."),
        }
    }
}

impl std::error::Error for ProfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::InvalidDiscordUserId | Self::InvalidStoredTheme => None,
        }
    }
}

impl From<sqlx::Error> for ProfileError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

/// PostgreSQL access for minimal, user-owned profile preferences.
#[derive(Clone)]
pub struct ProfileRepository {
    pool: PgPool,
}

impl ProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a default profile if necessary, then return its current theme.
    pub async fn get_or_create_theme(
        &self,
        discord_user_id: &str,
    ) -> Result<BoardTheme, ProfileError> {
        validate_discord_user_id(discord_user_id)?;
        sqlx::query(
            "INSERT INTO player_profiles (discord_user_id, board_theme) \
             VALUES ($1, 'classic') ON CONFLICT (discord_user_id) DO NOTHING",
        )
        .bind(discord_user_id)
        .execute(&self.pool)
        .await?;

        let stored: String = sqlx::query_scalar(
            "SELECT board_theme FROM player_profiles WHERE discord_user_id = $1",
        )
        .bind(discord_user_id)
        .fetch_one(&self.pool)
        .await?;

        BoardTheme::parse_stored(&stored)
    }

    /// Save one of the supported cosmetic themes.
    pub async fn set_theme(
        &self,
        discord_user_id: &str,
        theme: BoardTheme,
    ) -> Result<BoardTheme, ProfileError> {
        validate_discord_user_id(discord_user_id)?;
        let stored: String = sqlx::query_scalar(
            "INSERT INTO player_profiles (discord_user_id, board_theme) \
             VALUES ($1, $2) \
             ON CONFLICT (discord_user_id) DO UPDATE \
             SET board_theme = EXCLUDED.board_theme, updated_at = now() \
             RETURNING board_theme",
        )
        .bind(discord_user_id)
        .bind(theme.as_str())
        .fetch_one(&self.pool)
        .await?;

        BoardTheme::parse_stored(&stored)
    }

    /// Delete only this profile row. This is not the full account-deletion flow.
    pub async fn delete_profile(
        &self,
        discord_user_id: &str,
    ) -> Result<bool, ProfileError> {
        validate_discord_user_id(discord_user_id)?;
        let result = sqlx::query("DELETE FROM player_profiles WHERE discord_user_id = $1")
            .bind(discord_user_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

fn validate_discord_user_id(value: &str) -> Result<(), ProfileError> {
    if value.is_empty()
        || value.len() > 20
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(ProfileError::InvalidDiscordUserId);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_user_id() -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos()
            .to_string()
    }

    #[test]
    fn accepts_only_the_fixed_cosmetic_theme_catalogue() {
        for theme in BoardTheme::ALL {
            assert_eq!(BoardTheme::parse(theme.as_str()), Some(theme));
        }
        assert_eq!(BoardTheme::parse("FOREST"), Some(BoardTheme::Forest));
        assert_eq!(BoardTheme::parse("admin"), None);
        assert_eq!(BoardTheme::parse(""), None);
    }

    #[test]
    fn accepts_only_decimal_discord_user_references() {
        assert!(validate_discord_user_id("123456789012345678").is_ok());
        assert!(validate_discord_user_id("").is_err());
        assert!(validate_discord_user_id("user-123").is_err());
        assert!(validate_discord_user_id("123456789012345678901").is_err());
    }

    #[tokio::test]
    async fn profile_preferences_persist_and_are_isolated_per_player() {
        let database_url = std::env::var("DATABASE_URL")
            .expect("PostgreSQL integration tests require DATABASE_URL");
        let database = crate::database::Database::connect(&database_url)
            .await
            .expect("database and migrations");
        let profiles = ProfileRepository::new(database.pool());
        let first_user = unique_user_id();
        let second_user = unique_user_id();

        let first_default = profiles
            .get_or_create_theme(&first_user)
            .await
            .expect("create the default profile");
        let second_default = profiles
            .get_or_create_theme(&second_user)
            .await
            .expect("create the second default profile");
        assert_eq!(first_default, BoardTheme::Classic);
        assert_eq!(second_default, BoardTheme::Classic);

        assert_eq!(
            profiles
                .set_theme(&first_user, BoardTheme::Sunset)
                .await
                .expect("save the preference"),
            BoardTheme::Sunset
        );

        let recovered = ProfileRepository::new(database.pool())
            .get_or_create_theme(&first_user)
            .await
            .expect("read the preference from a new repository");
        assert_eq!(recovered, BoardTheme::Sunset);
        assert_eq!(
            profiles
                .get_or_create_theme(&second_user)
                .await
                .expect("other player's preference is isolated"),
            BoardTheme::Classic
        );

        assert!(profiles
            .delete_profile(&first_user)
            .await
            .expect("remove the test profile"));
        assert!(profiles
            .delete_profile(&second_user)
            .await
            .expect("remove the second test profile"));
    }
}
