//! Opt-in, Discord-server-scoped cooperative project progress.
//!
//! Membership is personal and exportable/deletable. Contribution rows contain no
//! contributor identity, so shared progress survives account deletion.

use sqlx::{PgPool, Postgres, Transaction};

pub const PROJECT_KEY: &str = "lantern_road_survey";
pub const PROJECT_NAME: &str = "Lantern Road Survey";
pub const CONTRIBUTION_GOAL: i64 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectSnapshot {
    pub id: i64,
    pub project_key: String,
    pub name: String,
    pub status: String,
    pub contribution_count: i64,
    pub contribution_goal: i64,
    pub member_count: i64,
    pub joined: bool,
}

#[derive(Clone)]
pub struct GuildProjectStore {
    pool: PgPool,
}

impl GuildProjectStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create this server's authored project, or return its existing state.
    /// Repeating setup cannot reset progress or create a second copy.
    pub async fn start_or_get(&self, guild_id: &str) -> Result<ProjectSnapshot, sqlx::Error> {
        sqlx::query(
            "INSERT INTO guild_projects (guild_id, project_key, project_name, contribution_goal)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (guild_id, project_key) DO NOTHING",
        )
        .bind(guild_id)
        .bind(PROJECT_KEY)
        .bind(PROJECT_NAME)
        .bind(CONTRIBUTION_GOAL)
        .execute(&self.pool)
        .await?;

        self.snapshot(guild_id, "")
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    /// Explicitly opt a player into an active project in the invoking server.
    pub async fn join(
        &self,
        guild_id: &str,
        player_id: &str,
    ) -> Result<Option<ProjectSnapshot>, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let project_id: Option<i64> = sqlx::query_scalar(
            "SELECT guild_project_id FROM guild_projects
             WHERE guild_id = $1 AND project_key = $2 AND status = 'active'
             FOR UPDATE",
        )
        .bind(guild_id)
        .bind(PROJECT_KEY)
        .fetch_optional(&mut *transaction)
        .await?;
        let Some(project_id) = project_id else {
            return Ok(None);
        };

        sqlx::query(
            "INSERT INTO guild_project_members (guild_project_id, discord_user_id)
             VALUES ($1, $2) ON CONFLICT (guild_project_id, discord_user_id) DO NOTHING",
        )
        .bind(project_id)
        .bind(player_id)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        self.snapshot(guild_id, player_id).await
    }

    /// Leave without removing earned guild progress. The project row lock
    /// serialises leaving against a concurrent checkpoint contribution.
    pub async fn leave(
        &self,
        guild_id: &str,
        player_id: &str,
    ) -> Result<Option<ProjectSnapshot>, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let project_id: Option<i64> = sqlx::query_scalar(
            "SELECT guild_project_id FROM guild_projects
             WHERE guild_id = $1 AND project_key = $2
             FOR UPDATE",
        )
        .bind(guild_id)
        .bind(PROJECT_KEY)
        .fetch_optional(&mut *transaction)
        .await?;
        let Some(project_id) = project_id else {
            return Ok(None);
        };

        sqlx::query(
            "DELETE FROM guild_project_members
             WHERE guild_project_id = $1 AND discord_user_id = $2",
        )
        .bind(project_id)
        .bind(player_id)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        self.snapshot(guild_id, player_id).await
    }

    /// Read only this guild's aggregate board and the invoking player's own join state.
    pub async fn snapshot(
        &self,
        guild_id: &str,
        player_id: &str,
    ) -> Result<Option<ProjectSnapshot>, sqlx::Error> {
        let row = sqlx::query_as::<_, (i64, String, String, String, i64, i64, i64, bool)>(
            "SELECT p.guild_project_id, p.project_key, p.project_name, p.status,
                    p.contribution_count, p.contribution_goal,
                    (SELECT count(*) FROM guild_project_members m
                     WHERE m.guild_project_id = p.guild_project_id),
                    EXISTS (SELECT 1 FROM guild_project_members m
                            WHERE m.guild_project_id = p.guild_project_id
                              AND m.discord_user_id = $2)
             FROM guild_projects p
             WHERE p.guild_id = $1 AND p.project_key = $3",
        )
        .bind(guild_id)
        .bind(player_id)
        .bind(PROJECT_KEY)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(
            |(
                id,
                project_key,
                name,
                status,
                contribution_count,
                contribution_goal,
                member_count,
                joined,
            )| {
                ProjectSnapshot {
                    id,
                    project_key,
                    name,
                    status,
                    contribution_count,
                    contribution_goal,
                    member_count,
                    joined,
                }
            },
        ))
    }

    /// Credit an opted-in actor's checkpoint only for an active shared Expedition
    /// from this exact server. Called inside the Expedition transaction after the
    /// authoritative checkpoint-history row is inserted.
    pub async fn record_shared_checkpoint(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        guild_id: &str,
        player_id: &str,
        expedition_run_id: i64,
        checkpoint_id: &str,
    ) -> Result<(), sqlx::Error> {
        let project_id: Option<i64> = sqlx::query_scalar(
            "SELECT guild_project_id FROM guild_projects
             WHERE guild_id = $1 AND project_key = $2 AND status = 'active'
             FOR UPDATE",
        )
        .bind(guild_id)
        .bind(PROJECT_KEY)
        .fetch_optional(&mut **transaction)
        .await?;
        let Some(project_id) = project_id else {
            return Ok(());
        };

        let eligible: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1
                FROM expedition_runs r
                JOIN expedition_participants ep
                  ON ep.expedition_run_id = r.expedition_run_id
                JOIN expedition_checkpoint_history h
                  ON h.expedition_run_id = r.expedition_run_id
                 AND h.content_version = r.content_version
                JOIN guild_project_members m
                  ON m.guild_project_id = $5
                 AND m.discord_user_id = $3
                WHERE r.expedition_run_id = $1
                  AND r.guild_id = $2
                  AND r.shared = TRUE
                  AND r.route_id = 'lantern-road'
                  AND r.state = 'active'
                  AND ep.discord_user_id = $3
                  AND ep.is_active = TRUE
                  AND h.checkpoint_id = $4
                  AND h.completed_by = $3
            )",
        )
        .bind(expedition_run_id)
        .bind(guild_id)
        .bind(player_id)
        .bind(checkpoint_id)
        .bind(project_id)
        .fetch_one(&mut **transaction)
        .await?;
        if !eligible {
            return Ok(());
        }

        // This unique source key is idempotent. No contributor ID is persisted.
        let inserted = sqlx::query(
            "INSERT INTO guild_project_contributions
                (guild_project_id, expedition_run_id, checkpoint_id)
             VALUES ($1, $2, $3)
             ON CONFLICT (guild_project_id, expedition_run_id, checkpoint_id) DO NOTHING",
        )
        .bind(project_id)
        .bind(expedition_run_id)
        .bind(checkpoint_id)
        .execute(&mut **transaction)
        .await?
        .rows_affected();
        if inserted == 0 {
            return Ok(());
        }

        sqlx::query(
            "UPDATE guild_projects
             SET contribution_count = contribution_count + 1,
                 status = CASE
                     WHEN contribution_count + 1 >= contribution_goal THEN 'completed'
                     ELSE 'active'
                 END,
                 completed_at = CASE
                     WHEN contribution_count + 1 >= contribution_goal THEN now()
                     ELSE NULL
                 END
             WHERE guild_project_id = $1 AND status = 'active'",
        )
        .bind(project_id)
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }

    /// Remove this server's board and its member/contribution rows after a real uninstall.
    pub async fn remove_guild(&self, guild_id: &str) -> Result<u64, sqlx::Error> {
        let removed = sqlx::query("DELETE FROM guild_projects WHERE guild_id = $1")
            .bind(guild_id)
            .execute(&self.pool)
            .await?
            .rows_affected();
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::{CONTRIBUTION_GOAL, PROJECT_KEY, PROJECT_NAME};

    #[test]
    fn first_project_is_fixed_and_has_a_bounded_authored_goal() {
        assert_eq!(PROJECT_KEY, "lantern_road_survey");
        assert_eq!(PROJECT_NAME, "Lantern Road Survey");
        assert_eq!(CONTRIBUTION_GOAL, 2);
    }
}

#[cfg(test)]
mod postgres_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        atlas::AtlasStore, database::Database, expedition::ExpeditionStore, profile::ProfileStore,
    };

    use super::GuildProjectStore;

    #[tokio::test]
    async fn guild_projects_are_opt_in_scoped_and_privacy_complete() {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must point to the CI PostgreSQL service");
        let database = Database::connect(&database_url)
            .await
            .expect("connect and migrate PostgreSQL");
        let pool = database.pool();
        let profiles = ProfileStore::new(pool.clone());
        let atlas = AtlasStore::new(pool.clone());
        let projects = GuildProjectStore::new(pool.clone());
        let expeditions = ExpeditionStore::new(pool.clone());
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
            % 1_000_000_000_000_000_000;

        // player_profiles validates Discord IDs as decimal snowflakes.
        let base_id = nonce % 100_000_000_000_000_000 + 100_000_000_000_000_000;
        let host_id = base_id.to_string();
        let member_id = (base_id + 1).to_string();
        let observer_id = (base_id + 2).to_string();
        let guild_id = format!("guild-project-server-{nonce}");
        let other_guild_id = format!("guild-project-other-server-{nonce}");

        for (id, name) in [
            (&host_id, "Project Host"),
            (&member_id, "Project Member"),
            (&observer_id, "Project Observer"),
        ] {
            crate::profile::create_test_player(pool.clone(), id, name)
                .await
                .expect("create an explicitly-consented test profile");
        }

        let first = projects
            .start_or_get(&guild_id)
            .await
            .expect("start first board");
        let repeated = projects
            .start_or_get(&guild_id)
            .await
            .expect("repeat setup");
        assert_eq!(first.id, repeated.id, "setup is idempotent");
        assert_eq!(first.contribution_count, 0);
        assert_eq!(first.member_count, 0);

        let other_server = projects
            .start_or_get(&other_guild_id)
            .await
            .expect("start isolated board");
        assert_ne!(first.id, other_server.id);

        let joined = projects
            .join(&guild_id, &member_id)
            .await
            .expect("join")
            .expect("active board");
        assert!(joined.joined);
        assert_eq!(joined.member_count, 1);
        let left = projects
            .leave(&guild_id, &member_id)
            .await
            .expect("leave")
            .expect("board remains");
        assert!(!left.joined);
        assert_eq!(left.member_count, 0);
        let joined = projects
            .join(&guild_id, &member_id)
            .await
            .expect("rejoin")
            .expect("active board");
        assert!(joined.joined);

        let member_export = profiles
            .export_json(&member_id)
            .await
            .expect("export personal membership");
        assert!(member_export.contains("\"format_version\": 12"));
        assert!(member_export.contains(&guild_id));
        assert!(member_export.contains("lantern_road_survey"));
        let inventory = profiles
            .inventory(&member_id)
            .await
            .expect("inventory membership");
        assert_eq!(inventory.guild_project_membership_records, 1);

        atlas
            .snapshot(&host_id)
            .await
            .expect("host starts at Lantern Square");
        atlas
            .snapshot(&member_id)
            .await
            .expect("member starts at Lantern Square");
        atlas
            .snapshot(&observer_id)
            .await
            .expect("observer starts at Lantern Square");

        let run = expeditions
            .start(&host_id, Some(&guild_id), true)
            .await
            .expect("start shared Expedition");
        expeditions
            .join(&member_id, &guild_id, run.run_id)
            .await
            .expect("member joins Expedition");
        expeditions
            .join(&observer_id, &guild_id, run.run_id)
            .await
            .expect("observer joins Expedition");
        expeditions
            .begin(&host_id, Some(&guild_id), run.run_id)
            .await
            .expect("begin shared Expedition");

        // Being in an Expedition alone never enrols the host or contributes.
        expeditions
            .advance(
                &host_id,
                Some(&guild_id),
                run.run_id,
                "mosslight-grove",
                "inspect",
            )
            .await
            .expect("first checkpoint");
        let after_unopted = projects
            .snapshot(&guild_id, &observer_id)
            .await
            .expect("read board")
            .expect("project exists");
        assert_eq!(after_unopted.contribution_count, 0);
        assert!(!after_unopted.joined);

        projects
            .join(&guild_id, &host_id)
            .await
            .expect("host opts in")
            .expect("board active");
        expeditions
            .advance(
                &host_id,
                Some(&guild_id),
                run.run_id,
                "ember-archive",
                "star",
            )
            .await
            .expect("opted-in host checkpoint");
        expeditions
            .advance(
                &member_id,
                Some(&guild_id),
                run.run_id,
                "starfall-observatory",
                "chart",
            )
            .await
            .expect("opted-in member final checkpoint");

        let completed = projects
            .snapshot(&guild_id, &observer_id)
            .await
            .expect("read completed board")
            .expect("project exists");
        assert_eq!(completed.status, "completed");
        assert_eq!(completed.contribution_count, completed.contribution_goal);
        assert_eq!(completed.contribution_count, 2);
        assert_eq!(completed.member_count, 2);
        assert!(!completed.joined);
        assert!(
            projects
                .join(&guild_id, &observer_id)
                .await
                .expect("completed project rejects enrolment")
                .is_none()
        );

        let deletion = profiles
            .delete_account(&host_id)
            .await
            .expect("delete project member while keeping guild progress");
        assert_eq!(deletion.guild_project_memberships_removed, 1);
        assert_eq!(deletion.expedition_checkpoint_actors_anonymized, 2);

        let persisted = projects
            .snapshot(&guild_id, &member_id)
            .await
            .expect("read surviving project")
            .expect("project survives member deletion");
        assert_eq!(persisted.status, "completed");
        assert_eq!(persisted.contribution_count, 2);
        assert_eq!(persisted.member_count, 1);
        let anonymous_events: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM guild_project_contributions WHERE guild_project_id = $1",
        )
        .bind(persisted.id)
        .fetch_one(&pool)
        .await
        .expect("inspect anonymous contributions");
        assert_eq!(anonymous_events, 2);

        let actor_links: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM expedition_checkpoint_history WHERE completed_by = $1",
        )
        .bind(&host_id)
        .fetch_one(&pool)
        .await
        .expect("verify actor identity was removed");
        assert_eq!(actor_links, 0);

        profiles
            .delete_account(&member_id)
            .await
            .expect("delete remaining member");
        profiles
            .delete_account(&observer_id)
            .await
            .expect("delete other Expedition participant");
        assert_eq!(
            projects
                .remove_guild(&guild_id)
                .await
                .expect("remove server board"),
            1
        );
        assert!(
            projects
                .snapshot(&guild_id, "")
                .await
                .expect("verify uninstall")
                .is_none()
        );
        assert_eq!(
            projects
                .remove_guild(&other_guild_id)
                .await
                .expect("remove isolated board"),
            1
        );
    }
}
