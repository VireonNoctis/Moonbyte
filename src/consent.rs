//! Versioned Terms of Service and privacy-policy acceptance.

use serde::Serialize;
use sqlx::PgPool;

/// Bump this whenever the Terms of Service materially change.
pub const TERMS_VERSION: &str = "1.0.0";
/// Bump this whenever the Privacy Policy materially changes.
pub const PRIVACY_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConsentRecord {
    pub terms_version: String,
    pub privacy_version: String,
    pub accepted_at: String,
    pub accepted_in_guild_id: Option<String>,
}

#[derive(Clone)]
pub struct ConsentStore {
    pool: PgPool,
}

impl ConsentStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Only a matching acceptance of both current document versions unlocks commands.
    pub async fn is_current(&self, player_id: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM user_consents
                WHERE discord_user_id = $1
                  AND terms_version = $2
                  AND privacy_version = $3
            )",
        )
        .bind(player_id)
        .bind(TERMS_VERSION)
        .bind(PRIVACY_VERSION)
        .fetch_one(&self.pool)
        .await
    }

    /// Store the user's explicit acceptance of the two current documents.
    pub async fn accept(
        &self,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<ConsentRecord, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        // Serialize acceptance against account deletion for this Discord user.
        sqlx::query(
            "SELECT pg_advisory_xact_lock(hashtextextended('moonbyte:user:' || $1, 0))",
        )
        .bind(player_id)
        .execute(&mut *transaction)
        .await?;

        let (terms_version, privacy_version, accepted_at, accepted_in_guild_id) =
            sqlx::query_as::<_, (String, String, String, Option<String>)>(
                "INSERT INTO user_consents
                    (discord_user_id, terms_version, privacy_version, accepted_at, accepted_in_guild_id)
                 VALUES ($1, $2, $3, now(), $4)
                 ON CONFLICT (discord_user_id) DO UPDATE SET
                    terms_version = EXCLUDED.terms_version,
                    privacy_version = EXCLUDED.privacy_version,
                    accepted_at = now(),
                    accepted_in_guild_id = EXCLUDED.accepted_in_guild_id
                 RETURNING terms_version, privacy_version, accepted_at::text, accepted_in_guild_id",
            )
            .bind(player_id)
            .bind(TERMS_VERSION)
            .bind(PRIVACY_VERSION)
            .bind(guild_id)
            .fetch_one(&mut *transaction)
            .await?;

        transaction.commit().await?;
        Ok(ConsentRecord {
            terms_version,
            privacy_version,
            accepted_at,
            accepted_in_guild_id,
        })
    }

    pub async fn get(&self, player_id: &str) -> Result<Option<ConsentRecord>, sqlx::Error> {
        sqlx::query_as::<_, (String, String, String, Option<String>)>(
            "SELECT terms_version, privacy_version, accepted_at::text, accepted_in_guild_id
             FROM user_consents
             WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await
        .map(|record| {
            record.map(
                |(terms_version, privacy_version, accepted_at, accepted_in_guild_id)| {
                    ConsentRecord {
                        terms_version,
                        privacy_version,
                        accepted_at,
                        accepted_in_guild_id,
                    }
                },
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{PRIVACY_VERSION, TERMS_VERSION};

    #[test]
    fn current_policy_versions_are_explicit_and_non_empty() {
        assert!(!TERMS_VERSION.trim().is_empty());
        assert!(!PRIVACY_VERSION.trim().is_empty());
    }
}
