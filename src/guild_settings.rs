//! Guild-scoped command routing, live cache invalidation and removal cleanup.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use sqlx::{PgPool, postgres::PgListener};

const SETTINGS_CHANNEL: &str = "moonbyte_guild_settings_changed";

/// Settings isolated to one Discord server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildCommandSettings {
    pub prefix: String,
    pub command_channel_id: Option<String>,
    pub updated_by: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuildCleanup {
    pub sessions_removed: u64,
    pub command_settings_removed: u64,
}

/// PostgreSQL-backed guild settings with an in-process read cache.
///
/// PostgreSQL LISTEN/NOTIFY keeps caches in separate bot processes coherent
/// without introducing Redis or querying PostgreSQL for each incoming message.
#[derive(Clone)]
pub struct GuildSettingsStore {
    pool: PgPool,
    cache: Arc<RwLock<HashMap<String, GuildCommandSettings>>>,
}

impl GuildSettingsStore {
    /// Load settings and subscribe before the store is used by command dispatch.
    pub async fn new(pool: PgPool) -> Result<Self, sqlx::Error> {
        let mut listener = PgListener::connect_with(&pool).await?;
        listener.listen(SETTINGS_CHANNEL).await?;

        let cache = Arc::new(RwLock::new(load_all_settings(&pool).await?));
        let store = Self {
            pool: pool.clone(),
            cache: Arc::clone(&cache),
        };

        tokio::spawn(run_settings_listener(pool, cache, listener));
        Ok(store)
    }

    /// Read cached settings without a database round trip on every incoming message.
    pub fn get(&self, guild_id: &str) -> Option<GuildCommandSettings> {
        self.cache
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(guild_id)
            .cloned()
    }

    /// Persist and publish the new configuration to every running bot process.
    pub async fn save(
        &self,
        guild_id: &str,
        prefix: &str,
        command_channel_id: Option<&str>,
        updated_by: &str,
    ) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
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
        .execute(&mut *transaction)
        .await?;
        notify_settings_changed(&mut transaction, guild_id).await?;
        transaction.commit().await?;

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

    /// Remove the override and return this server to the process default prefix.
    pub async fn reset(&self, guild_id: &str) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM guild_command_settings WHERE guild_id = $1")
            .bind(guild_id)
            .execute(&mut *transaction)
            .await?;
        notify_settings_changed(&mut transaction, guild_id).await?;
        transaction.commit().await?;

        self.cache
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(guild_id);
        Ok(())
    }

    /// Remove guild-owned routing and active session data after the bot leaves.
    ///
    /// Discord emits GuildDelete for temporary unavailability as well as removal.
    /// The event adapter must call this only when the unavailable flag is false.
    pub async fn remove_guild_data(&self, guild_id: &str) -> Result<GuildCleanup, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let sessions_removed = sqlx::query("DELETE FROM game_sessions WHERE guild_id = $1")
            .bind(guild_id)
            .execute(&mut *transaction)
            .await?
            .rows_affected();
        let command_settings_removed =
            sqlx::query("DELETE FROM guild_command_settings WHERE guild_id = $1")
                .bind(guild_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        notify_settings_changed(&mut transaction, guild_id).await?;
        transaction.commit().await?;

        self.cache
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(guild_id);

        Ok(GuildCleanup {
            sessions_removed,
            command_settings_removed,
        })
    }
}

async fn notify_settings_changed(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    guild_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT pg_notify('moonbyte_guild_settings_changed', $1)")
        .bind(guild_id)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

async fn load_all_settings(
    pool: &PgPool,
) -> Result<HashMap<String, GuildCommandSettings>, sqlx::Error> {
    let rows: Vec<(String, String, Option<String>, String)> = sqlx::query_as(
        "SELECT guild_id, prefix, command_channel_id, updated_by
         FROM guild_command_settings",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
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
        .collect())
}

async fn refresh_guild_setting(
    pool: &PgPool,
    cache: &RwLock<HashMap<String, GuildCommandSettings>>,
    guild_id: &str,
) -> Result<(), sqlx::Error> {
    let row: Option<(String, Option<String>, String)> = sqlx::query_as(
        "SELECT prefix, command_channel_id, updated_by
         FROM guild_command_settings
         WHERE guild_id = $1",
    )
    .bind(guild_id)
    .fetch_optional(pool)
    .await?;

    let mut cache = cache
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((prefix, command_channel_id, updated_by)) = row {
        cache.insert(
            guild_id.to_owned(),
            GuildCommandSettings {
                prefix,
                command_channel_id,
                updated_by,
            },
        );
    } else {
        cache.remove(guild_id);
    }
    Ok(())
}

async fn run_settings_listener(
    pool: PgPool,
    cache: Arc<RwLock<HashMap<String, GuildCommandSettings>>>,
    mut listener: PgListener,
) {
    loop {
        match listener.recv().await {
            Ok(notification) => {
                if let Err(error) =
                    refresh_guild_setting(&pool, &cache, notification.payload()).await
                {
                    eprintln!("Moonbyte: failed to refresh a guild settings cache entry: {error}");
                }
            }
            Err(error) => {
                eprintln!("Moonbyte: guild settings listener disconnected: {error}");
                loop {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    let reconnect = async {
                        let mut next = PgListener::connect_with(&pool).await?;
                        next.listen(SETTINGS_CHANNEL).await?;
                        let latest = load_all_settings(&pool).await?;
                        Ok::<_, sqlx::Error>((next, latest))
                    };
                    match reconnect.await {
                        Ok((next, latest)) => {
                            *cache
                                .write()
                                .unwrap_or_else(std::sync::PoisonError::into_inner) = latest;
                            listener = next;
                            break;
                        }
                        Err(error) => {
                            eprintln!(
                                "Moonbyte: unable to reconnect the guild settings listener: {error}"
                            );
                        }
                    }
                }
            }
        }
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

#[cfg(test)]
mod postgres_tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use crate::{
        database::Database,
        game::{GameKind, GameRuntime},
    };

    use super::GuildSettingsStore;

    #[tokio::test]
    async fn settings_notifications_sync_instances_and_guild_removal_cleans_data() {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must point to the CI PostgreSQL service");
        let database = Database::connect(&database_url)
            .await
            .expect("connect and migrate PostgreSQL");
        let pool = database.pool();
        let first = GuildSettingsStore::new(pool.clone())
            .await
            .expect("start first settings listener");
        let second = GuildSettingsStore::new(pool.clone())
            .await
            .expect("start second settings listener");

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let guild_id = format!("guild-settings-test-{nonce}");
        let player_id = format!("guild-player-{nonce}");

        first
            .save(&guild_id, "mb!", Some("123456789012345678"), &player_id)
            .await
            .expect("save settings");

        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if second
                    .get(&guild_id)
                    .is_some_and(|settings| settings.prefix == "mb!")
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
        })
        .await
        .expect("second process receives settings notification");

        let runtime = GameRuntime::new(pool);
        let session = runtime
            .start(GameKind::MemoryMatch, &player_id, Some(&guild_id))
            .await
            .expect("create guild session");
        let cleanup = first
            .remove_guild_data(&guild_id)
            .await
            .expect("remove the departed guild's records");
        assert_eq!(cleanup.sessions_removed, 1);
        assert_eq!(cleanup.command_settings_removed, 1);

        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if second.get(&guild_id).is_none() {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
        })
        .await
        .expect("second process receives guild-removal notification");

        assert!(
            runtime
                .get(&session.id, &player_id, Some(&guild_id))
                .await
                .is_err(),
            "guild-owned session data must be removed when the bot leaves the guild"
        );
    }
}
