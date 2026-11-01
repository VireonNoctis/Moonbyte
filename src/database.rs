use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

const INITIAL_MIGRATION: &str = include_str!("../migrations/0001_postgres_sessions.sql");
const INITIAL_SCHEMA_VERSION: i64 = 1;

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
            "CREATE TABLE IF NOT EXISTS moonbyte_schema_migrations (                version BIGINT PRIMARY KEY,                applied_at TIMESTAMPTZ NOT NULL DEFAULT now()            )",
        )
        .execute(&mut *transaction)
        .await?;

        let already_applied: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM moonbyte_schema_migrations WHERE version = $1)",
        )
        .bind(INITIAL_SCHEMA_VERSION)
        .fetch_one(&mut *transaction)
        .await?;

        if !already_applied {
            for statement in INITIAL_MIGRATION
                .split(';')
                .map(str::trim)
                .filter(|sql| !sql.is_empty())
            {
                sqlx::query(statement).execute(&mut *transaction).await?;
            }

            sqlx::query("INSERT INTO moonbyte_schema_migrations (version) VALUES ($1)")
                .bind(INITIAL_SCHEMA_VERSION)
                .execute(&mut *transaction)
                .await?;
        }

        transaction.commit().await
    }
}
