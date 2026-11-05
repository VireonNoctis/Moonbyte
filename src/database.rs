use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

const INITIAL_MIGRATION: &str = include_str!("../migrations/0001_postgres_sessions.sql");
const PLAYER_MIGRATION: &str = include_str!("../migrations/0002_player_profiles.sql");
const GAME_STATS_MIGRATION: &str = include_str!("../migrations/0003_game_results_and_stats.sql");
const WALLET_MIGRATION: &str = include_str!("../migrations/0004_earned_currency_wallet.sql");
const ACHIEVEMENTS_MIGRATION: &str = include_str!("../migrations/0005_player_achievements.sql");
const ATLAS_MIGRATION: &str = include_str!("../migrations/0006_player_atlas.sql");
const GUILD_COMMAND_MIGRATION: &str = include_str!("../migrations/0007_guild_command_settings.sql");
const USER_CONSENT_MIGRATION: &str = include_str!("../migrations/0008_user_consents.sql");
const WAYFARER_MIGRATION: &str = include_str!("../migrations/0009_wayfarer_rivalry.sql");
const CHRONICLE_MIGRATION: &str = include_str!("../migrations/0010_player_chronicle.sql");
const EXPEDITION_MIGRATION: &str = include_str!("../migrations/0011_expeditions.sql");
const INVENTORY_MIGRATION: &str = include_str!("../migrations/0012_player_inventory.sql");

/// PostgreSQL connection pool and ordered schema migrations.
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(10))
            .connect(database_url)
            .await?;

        let database = Self { pool };
        database.migrate().await?;
        Ok(database)
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    /// Remove sessions whose inactivity window has elapsed.
    pub async fn remove_expired_sessions(&self) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM game_sessions WHERE expires_at <= now()")
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    async fn migrate(&self) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        // Serialise startup migrations when multiple bot processes start together.
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(0x4d4f_4f4e_4259_5445_i64)
            .execute(&mut *transaction)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS moonbyte_schema_migrations (
                version BIGINT PRIMARY KEY,
                applied_at TIMESTAMPTZ NOT NULL DEFAULT now()
            )",
        )
        .execute(&mut *transaction)
        .await?;

        for (version, migration) in [
            (1_i64, INITIAL_MIGRATION),
            (2_i64, PLAYER_MIGRATION),
            (3_i64, GAME_STATS_MIGRATION),
            (4_i64, WALLET_MIGRATION),
            (5_i64, ACHIEVEMENTS_MIGRATION),
            (6_i64, ATLAS_MIGRATION),
            (7_i64, GUILD_COMMAND_MIGRATION),
            (8_i64, USER_CONSENT_MIGRATION),
            (9_i64, WAYFARER_MIGRATION),
            (10_i64, CHRONICLE_MIGRATION),
            (11_i64, EXPEDITION_MIGRATION),
            (12_i64, INVENTORY_MIGRATION),
        ] {
            let already_applied: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM moonbyte_schema_migrations WHERE version = $1)",
            )
            .bind(version)
            .fetch_one(&mut *transaction)
            .await?;

            if already_applied {
                continue;
            }

            for statement in migration
                .split(';')
                .map(str::trim)
                .filter(|sql| !sql.is_empty())
            {
                sqlx::query(statement).execute(&mut *transaction).await?;
            }

            sqlx::query("INSERT INTO moonbyte_schema_migrations (version) VALUES ($1)")
                .bind(version)
                .execute(&mut *transaction)
                .await?;
        }

        transaction.commit().await
    }
}
