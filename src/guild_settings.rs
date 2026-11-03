//! Guild-scoped command routing and prefix configuration.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sqlx::PgPool;

/// Settings isolated to one Discord server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildCommandSettings {
    pub prefix: String,
    pub command_channel_id: Option<String>,
    pub updated_by: String,
}

/// PostgreSQL-backed guild settings with an in-process read cache.
#[derive(Clone)]
pub struct GuildSettingsStore {
    pool: PgPool,
    cache: Arc<RwLock<HashMap<String, GuildCommandSettings>>>,
}

impl GuildSettingsStore {
    /// Load all saved settings after schema migrations have completed.
    pub async fn new(pool: PgPool) -> Result<Self, sqlx::Error> {
        let rows: Vec<(String, String, Option<String>, String)> = sqlx::query_as(
            "SELECT guild_id, prefix, command_channel_id, updated_by
             FROM guild_command_settings",
        )
        .fetch_all(&pool)
        .await?;

        let cache = rows
            .into_iter()
            .map(|(guild_id, prefix, command_channel_id, updated_by)| {
                (
                    guild_id,
                    GuildCommandSettings {
                        prefix,
                        command_channel_id,
                        updated_by,
                    },
                )
            })
            .collect();

        Ok(Self {
            pool,
            cache: Arc::new(RwLock::new(cache)),
        })
    }

    /// Read without querying PostgreSQL on every incoming Discord message.
    pub fn get(&self, guild_id: &str) -> Option<GuildCommandSettings> {
        self.cache
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(guild_id)
            .cloned()
    }

    /// Persist the configuration before publishing it to the read cache.
    pub async fn save(
        &self,
        guild_id: &str,
        prefix: &str,
        command_channel_id: Option<&str>,
        updated_by: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO guild_command_settings
                (guild_id, prefix, command_channel_id, updated_by)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (guild_id) DO UPDATE SET
                prefix = EXCLUDED.prefix,
                command_channel_id = EXCLUDED.command_channel_id,
                updated_by = EXCLUDED.updated_by,
                updated_at = now()",
        )
        .bind(guild_id)
        .bind(prefix)
        .bind(command_channel_id)
        .bind(updated_by)
        .execute(&self.pool)
        .await?;

        self.cache
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(
                guild_id.to_owned(),
                GuildCommandSettings {
                    prefix: prefix.to_owned(),
                    command_channel_id: command_channel_id.map(str::to_owned),
                    updated_by: updated_by.to_owned(),
                },
            );
        Ok(())
    }

    /// Remove the override and return the server to the process default.
    pub async fn reset(&self, guild_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM guild_command_settings WHERE guild_id = $1")
            .bind(guild_id)
            .execute(&self.pool)
            .await?;
        self.cache
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(guild_id);
        Ok(())
    }
}

/// Prefixes are short tokens and cannot contain whitespace or Discord mentions.
pub fn validate_prefix(prefix: &str) -> Result<(), &'static str> {
    let length = prefix.chars().count();
    if !(1..=8).contains(&length) {
        return Err("A prefix must contain between 1 and 8 characters.");
    }
    if prefix.chars().any(char::is_whitespace) {
        return Err("A prefix cannot contain whitespace.");
    }
    if prefix.starts_with("<@") || prefix.starts_with("<#") {
        return Err("A prefix cannot be a Discord mention.");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_prefix;

    #[test]
    fn accepts_short_prefixes_without_whitespace() {
        for prefix in [".", "!", "mb!", "§", ">>"] {
            assert_eq!(validate_prefix(prefix), Ok(()));
        }
    }

    #[test]
    fn rejects_empty_long_whitespace_and_mention_prefixes() {
        for prefix in ["", "123456789", "two words", "with\ttab", "<@123>"] {
            assert!(validate_prefix(prefix).is_err(), "accepted {prefix:?}");
        }
    }

    #[test]
    fn validates_unicode_character_count_not_utf8_byte_count() {
        assert_eq!(validate_prefix("§§§§§§§§"), Ok(()));
        assert!(validate_prefix("§§§§§§§§§").is_err());
    }
}
