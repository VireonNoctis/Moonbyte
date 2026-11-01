use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

struct Migration {
    version: i64,
    sql: &'static str,
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        sql: include_str!("../migrations/0001_postgres_sessions.sql"),
    },
    Migration {
        version: 2,
        sql: include_str!("../migrations/0002_player_profiles.sql"),
    },
];

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
            "CREATE TABLE IF NOT EXISTS moonbyte_schema_migrations ( \
                version BIGINT PRIMARY KEY, \
                applied_at TIMESTAMPTZ NOT NULL DEFAULT now() \
            )",
        )
        .execute(&mut *transaction)
        .await?;

        for migration in MIGRATIONS {
            let already_applied: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM moonbyte_schema_migrations WHERE version = $1)",
            )
            .bind(migration.version)
            .fetch_one(&mut *transaction)
            .await?;

            if already_applied {
                continue;
            }

            for statement in migration
                .sql
                .split(';')
                .map(str::trim)
                .filter(|sql| !sql.is_empty())
            {
                sqlx::query(statement).execute(&mut *transaction).await?;
            }

            sqlx::query("INSERT INTO moonbyte_schema_migrations (version) VALUES ($1)")
                .bind(migration.version)
                .execute(&mut *transaction)
                .await?;
        }

        transaction.commit().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migrations_are_ordered_and_idempotent() {
        let database_url = std::env::var("DATABASE_URL")
            .expect("PostgreSQL integration tests require DATABASE_URL");
        let database = Database::connect(&database_url)
            .await
            .expect("first migration run");
        database.migrate().await.expect("repeated migration run");

        let applied: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM moonbyte_schema_migrations WHERE version IN (1, 2)",
        )
        .fetch_one(&database.pool)
        .await
        .expect("read migration ledger");

        assert_eq!(applied, 2);
    }
}
