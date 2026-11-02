//! Persistent, server-authoritative player achievements.
//!
//! Achievement checks run in the game-completion transaction after aggregate
//! statistics are updated. The unique player/achievement key makes unlocks
//! idempotent across retries and concurrent completions.

use sqlx::{PgPool, Postgres, Transaction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AchievementDefinition {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub category: &'static str,
}

pub const ACHIEVEMENT_CATALOGUE: [AchievementDefinition; 5] = [
    AchievementDefinition {
        id: "first_clear",
        title: "First Steps",
        description: "Complete your first Moonbyte game.",
        category: "milestone",
    },
    AchievementDefinition {
        id: "first_victory",
        title: "Bright Start",
        description: "Earn your first game victory.",
        category: "milestone",
    },
    AchievementDefinition {
        id: "game_collector",
        title: "Game Hopper",
        description: "Complete all four games currently in the library.",
        category: "collection",
    },
    AchievementDefinition {
        id: "ten_games",
        title: "Table Regular",
        description: "Complete ten games.",
        category: "milestone",
    },
    AchievementDefinition {
        id: "five_wins",
        title: "Rising Champion",
        description: "Win five games.",
        category: "mastery",
    },
];

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
pub struct PlayerAchievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub source_session_id: String,
    pub unlocked_at: String,
}

#[derive(Clone)]
pub struct AchievementStore {
    pool: PgPool,
}

impl AchievementStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_for_player(
        &self,
        player_id: &str,
    ) -> Result<Vec<PlayerAchievement>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
            "SELECT d.achievement_id, d.title, d.description, d.category, \
             p.source_session_id, p.unlocked_at::text \
             FROM player_achievements p \
             JOIN achievement_definitions d USING (achievement_id) \
             WHERE p.discord_user_id = $1 \
             ORDER BY p.unlocked_at ASC, d.achievement_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(id, title, description, category, source_session_id, unlocked_at)| {
                    PlayerAchievement {
                        id,
                        title,
                        description,
                        category,
                        source_session_id,
                        unlocked_at,
                    }
                },
            )
            .collect())
    }
}

/// Evaluate the fixed catalogue after a participant's game counters are updated.
/// The function is intentionally transaction-scoped so unlocks cannot outlive
/// a failed or rolled-back game completion.
pub(crate) async fn award_for_player_completion(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    source_session_id: &str,
) -> Result<u64, sqlx::Error> {
    let (games_played, games_won, distinct_games): (i64, i64, i64) = sqlx::query_as(
        "SELECT COALESCE(SUM(games_played), 0), \
         COALESCE(SUM(games_won), 0), COUNT(*) \
         FROM player_game_stats WHERE discord_user_id = $1 AND games_played > 0",
    )
    .bind(player_id)
    .fetch_one(&mut **transaction)
    .await?;

    let mut inserted = 0_u64;
    for definition in ACHIEVEMENT_CATALOGUE {
        let qualifies = match definition.id {
            "first_clear" => games_played >= 1,
            "first_victory" => games_won >= 1,
            "game_collector" => distinct_games >= 4,
            "ten_games" => games_played >= 10,
            "five_wins" => games_won >= 5,
            _ => false,
        };
        if !qualifies {
            continue;
        }

        let awarded = sqlx::query(
            "INSERT INTO player_achievements \
             (discord_user_id, achievement_id, source_session_id) \
             VALUES ($1, $2, $3) ON CONFLICT (discord_user_id, achievement_id) DO NOTHING",
        )
        .bind(player_id)
        .bind(definition.id)
        .bind(source_session_id)
        .execute(&mut **transaction)
        .await?
        .rows_affected();

        inserted += awarded;
    }

    Ok(inserted)
}

#[cfg(test)]
mod tests {
    use super::ACHIEVEMENT_CATALOGUE;

    #[test]
    fn catalogue_ids_are_unique_and_descriptions_are_nonempty() {
        for (index, achievement) in ACHIEVEMENT_CATALOGUE.iter().enumerate() {
            assert!(!achievement.title.trim().is_empty());
            assert!(!achievement.description.trim().is_empty());
            assert!(
                ACHIEVEMENT_CATALOGUE[index + 1..]
                    .iter()
                    .all(|other| other.id != achievement.id)
            );
        }
    }

    #[test]
    fn catalogue_has_stable_expected_ids() {
        assert_eq!(
            ACHIEVEMENT_CATALOGUE.map(|achievement| achievement.id),
            [
                "first_clear",
                "first_victory",
                "game_collector",
                "ten_games",
                "five_wins"
            ]
        );
    }

    #[tokio::test]
    async fn achievements_unlock_at_thresholds_once() {
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
        let player_id = format!("achievement-{nonce}");
        let mut transaction = pool.begin().await.expect("begin fixture transaction");

        for (game_id, games_played, games_won) in [
            ("tic-tac-toe", 3_i64, 2_i64),
            ("connect-four", 3_i64, 1_i64),
            ("memory-match", 2_i64, 1_i64),
            ("word-scramble", 2_i64, 1_i64),
        ] {
            sqlx::query(
                "INSERT INTO player_game_stats \
                 (discord_user_id, game_id, games_played, games_won, draws) \
                 VALUES ($1, $2, $3, $4, 0)",
            )
            .bind(&player_id)
            .bind(game_id)
            .bind(games_played)
            .bind(games_won)
            .execute(&mut *transaction)
            .await
            .expect("insert deterministic player statistics");
        }

        let awarded =
            super::award_for_player_completion(&mut transaction, &player_id, "MB-ACHIEVEMENT-TEST")
                .await
                .expect("award eligible achievements");
        assert_eq!(awarded, 5);

        let repeated = super::award_for_player_completion(
            &mut transaction,
            &player_id,
            "MB-ACHIEVEMENT-RETRY",
        )
        .await
        .expect("repeat achievement evaluation");
        assert_eq!(repeated, 0);

        let unlocked: Vec<String> = sqlx::query_scalar(
            "SELECT achievement_id FROM player_achievements \
             WHERE discord_user_id = $1 ORDER BY achievement_id",
        )
        .bind(&player_id)
        .fetch_all(&mut *transaction)
        .await
        .expect("read unlocked achievement IDs");
        assert_eq!(
            unlocked,
            [
                "first_clear",
                "first_victory",
                "five_wins",
                "game_collector",
                "ten_games"
            ]
        );

        sqlx::query("DELETE FROM player_achievements WHERE discord_user_id = $1")
            .bind(&player_id)
            .execute(&mut *transaction)
            .await
            .expect("clean achievement fixture");
        sqlx::query("DELETE FROM player_game_stats WHERE discord_user_id = $1")
            .bind(&player_id)
            .execute(&mut *transaction)
            .await
            .expect("clean player stats fixture");
        transaction.commit().await.expect("commit fixture cleanup");
    }
}
