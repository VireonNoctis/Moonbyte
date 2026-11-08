//! Persistent Lantern Road runs. Domain state is authoritative in PostgreSQL.

use crate::{
    atlas::{self, AtlasError, LOCATION_CATALOGUE},
    guild_project::GuildProjectStore,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::fmt;

pub const ROUTE_ID: &str = "lantern-road";
pub const ROUTE_VERSION: i16 = 1;
pub const GLASSWATER_ROUTE_ID: &str = "glasswater-crossing";
pub const GLASSWATER_ROUTE_VERSION: i16 = 1;
pub const MAX_PARTICIPANTS: i64 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    pub id: &'static str,
    pub location_id: &'static str,
    pub title: &'static str,
    pub prompt: &'static str,
    pub choices: &'static [&'static str],
    pub correct_choice: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteDefinition {
    pub id: &'static str,
    pub version: i16,
    pub title: &'static str,
    pub summary: &'static str,
    pub checkpoints: &'static [Checkpoint],
}

pub static LANTERN_ROAD: [Checkpoint; 4] = [
    Checkpoint {
        id: "lantern-square",
        location_id: "lantern-square",
        title: "The Lantern Square Departure",
        prompt: "The road begins where the first lantern was lit. Check the route and prepare to set out.",
        choices: &["ready"],
        correct_choice: None,
    },
    Checkpoint {
        id: "mosslight-grove",
        location_id: "mosslight-grove",
        title: "The Mosslight Observation",
        prompt: "The old stones hold the grove's stories. Choose how to make your first observation.",
        choices: &["listen", "inspect"],
        correct_choice: None,
    },
    Checkpoint {
        id: "ember-archive",
        location_id: "ember-archive",
        title: "The Ember Archive Riddle",
        prompt: "The keeper shows two marks on the old road chart: a lantern and a star. Which mark points toward the Starfall Observatory?",
        choices: &["star", "lantern"],
        correct_choice: Some("star"),
    },
    Checkpoint {
        id: "starfall-observatory",
        location_id: "starfall-observatory",
        title: "The Starfall Chart",
        prompt: "The observatory is reached. Set the final chart in its place to complete the Lantern Road.",
        choices: &["chart"],
        correct_choice: None,
    },
];

pub static GLASSWATER_CROSSING: [Checkpoint; 4] = [
    Checkpoint {
        id: "lantern-square",
        location_id: "lantern-square",
        title: "The Lantern Square Departure",
        prompt: "The coastal path starts at Lantern Square. Check the tide notes before leaving.",
        choices: &["ready"],
        correct_choice: None,
    },
    Checkpoint {
        id: "glasswater-pier",
        location_id: "glasswater-pier",
        title: "The Glasswater Survey",
        prompt: "The pier has two calm channels. Record the shoreline before following the connected star path.",
        choices: &["survey"],
        correct_choice: None,
    },
    Checkpoint {
        id: "glasswater-star-chart",
        location_id: "starfall-observatory",
        title: "The Western Star Chart",
        prompt: "From the observatory, the clear western marker points back toward Mosslight Grove. Which direction is recorded on the chart?",
        choices: &["east", "west"],
        correct_choice: Some("west"),
    },
    Checkpoint {
        id: "mosslight-return",
        location_id: "mosslight-grove",
        title: "The Mosslight Return",
        prompt: "The crossing ends at Mosslight Grove. Record the return marker to complete the route.",
        choices: &["record"],
        correct_choice: None,
    },
];

pub static EXPEDITION_ROUTES: [RouteDefinition; 2] = [
    RouteDefinition {
        id: ROUTE_ID,
        version: ROUTE_VERSION,
        title: "The Lantern Road",
        summary: "Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory",
        checkpoints: &LANTERN_ROAD,
    },
    RouteDefinition {
        id: GLASSWATER_ROUTE_ID,
        version: GLASSWATER_ROUTE_VERSION,
        title: "The Glasswater Crossing",
        summary: "Lantern Square → Glasswater Pier → Starfall Observatory → Mosslight Grove",
        checkpoints: &GLASSWATER_CROSSING,
    },
];

pub fn route_definition(route_id: &str) -> Option<&'static RouteDefinition> {
    EXPEDITION_ROUTES.iter().find(|route| route.id == route_id)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Participant {
    pub discord_user_id: String,
    pub active: bool,
    pub is_host: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    pub run_id: i64,
    pub route_id: String,
    pub route_title: &'static str,
    pub route_checkpoint_count: usize,
    pub content_version: i16,
    pub guild_id: Option<String>,
    pub shared: bool,
    pub host: Option<String>,
    pub state: String,
    pub checkpoint_index: i16,
    pub row_version: i64,
    pub current: Checkpoint,
    pub next: Option<Checkpoint>,
    pub participants: Vec<Participant>,
}

#[derive(Debug)]
pub enum ExpeditionError {
    Database(sqlx::Error),
    Atlas(AtlasError),
    SharedRequiresGuild,
    WrongGuild,
    NotAtStart,
    RunNotFound,
    NotParticipant,
    NotActiveParticipant,
    AlreadyInRun,
    NotRecruiting,
    NotActive,
    NotPaused,
    NotHost,
    PartyFull,
    StaleCheckpoint,
    PartyChanged,
    InvalidChoice,
    IncorrectAnswer,
    UnknownRoute,
}

impl fmt::Display for ExpeditionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(_) => f.write_str("The Expedition service is temporarily unavailable."),
            Self::Atlas(error) => error.fmt(f),
            Self::SharedRequiresGuild => f.write_str("A shared Expedition must be created inside a Discord server."),
            Self::WrongGuild => f.write_str("That group run belongs to a different Discord server."),
            Self::NotAtStart => f.write_str("You must be at the route starting location before starting or joining this Expedition."),
            Self::RunNotFound => f.write_str("That Expedition run was not found."),
            Self::NotParticipant => f.write_str("Only a participant can view that private run."),
            Self::NotActiveParticipant => f.write_str("You are not an active participant in that run."),
            Self::AlreadyInRun => f.write_str("You already have an active Expedition. Leave or finish it before joining another."),
            Self::NotRecruiting => f.write_str("That run is no longer accepting participants."),
            Self::NotActive => f.write_str("That run is not active."),
            Self::NotPaused => f.write_str("That run is not paused."),
            Self::NotHost => f.write_str("Only the current host can start or cancel this group run."),
            Self::PartyFull => f.write_str("An Expedition supports at most four participants."),
            Self::StaleCheckpoint => f.write_str("That checkpoint is no longer next. Refresh the run status before advancing."),
            Self::PartyChanged => f.write_str("The party changed while this checkpoint was being prepared. Refresh the Expedition and try again."),
            Self::InvalidChoice => f.write_str("That choice is not available at this checkpoint."),
            Self::IncorrectAnswer => f.write_str("That clue does not match this route. Nothing advanced; review the checkpoint and try again."),
            Self::UnknownRoute => f.write_str("This run refers to an unknown route or checkpoint."),
        }
    }
}
impl std::error::Error for ExpeditionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::Atlas(error) => Some(error),
            _ => None,
        }
    }
}
impl From<AtlasError> for ExpeditionError {
    fn from(error: AtlasError) -> Self {
        Self::Atlas(error)
    }
}

impl From<sqlx::Error> for ExpeditionError {
    fn from(error: sqlx::Error) -> Self {
        if matches!(&error, sqlx::Error::Database(db) if db.code().as_deref() == Some("23505")) {
            Self::AlreadyInRun
        } else {
            Self::Database(error)
        }
    }
}

#[derive(Clone)]
pub struct ExpeditionStore {
    pool: PgPool,
    guild_projects: GuildProjectStore,
}

impl ExpeditionStore {
    pub fn new(pool: PgPool) -> Self {
        Self {
            guild_projects: GuildProjectStore::new(pool.clone()),
            pool,
        }
    }

    pub async fn start(
        &self,
        player: &str,
        guild: Option<&str>,
        shared: bool,
    ) -> Result<Snapshot, ExpeditionError> {
        self.start_route(player, guild, ROUTE_ID, shared).await
    }

    pub async fn start_route(
        &self,
        player: &str,
        guild: Option<&str>,
        route_id: &str,
        shared: bool,
    ) -> Result<Snapshot, ExpeditionError> {
        let route = route_definition(route_id).ok_or(ExpeditionError::UnknownRoute)?;
        if shared && guild.is_none() {
            return Err(ExpeditionError::SharedRequiresGuild);
        }
        let mut tx = self.pool.begin().await?;
        lock_player(&mut tx, player).await?;
        ensure_at_start(&mut tx, player).await?;
        let state = if shared { "recruiting" } else { "active" };
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO expedition_runs (route_id, content_version, guild_id, host_discord_user_id, shared, state, current_checkpoint_index, started_at)
             VALUES ($1, $2, $3, $4, $5, $6, 0, CASE WHEN $5 THEN NULL ELSE now() END)
             RETURNING expedition_run_id",
        ).bind(route.id).bind(route.version).bind(guild).bind(player).bind(shared).bind(state)
         .fetch_one(&mut *tx).await?;
        sqlx::query("INSERT INTO expedition_participants (expedition_run_id, discord_user_id) VALUES ($1, $2)")
            .bind(id).bind(player).execute(&mut *tx).await?;
        sqlx::query(
            "UPDATE expedition_runs SET row_version = row_version + 1, updated_at = now()
             WHERE expedition_run_id = $1",
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn join(
        &self,
        player: &str,
        guild: &str,
        id: i64,
    ) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        lock_player(&mut tx, player).await?;
        let (route_id, content_version, saved_guild, state, shared) =
            sqlx::query_as::<_, (String, i16, Option<String>, String, bool)>(
                "SELECT route_id, content_version, guild_id, state, shared
                 FROM expedition_runs WHERE expedition_run_id = $1 FOR UPDATE",
            )
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ExpeditionError::RunNotFound)?;
        if !route_definition(&route_id).is_some_and(|route| route.version == content_version) {
            return Err(ExpeditionError::UnknownRoute);
        }
        if !shared || saved_guild.as_deref() != Some(guild) {
            return Err(ExpeditionError::WrongGuild);
        }
        if state != "recruiting" {
            return Err(ExpeditionError::NotRecruiting);
        }
        ensure_at_start(&mut tx, player).await?;
        let count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM expedition_participants WHERE expedition_run_id = $1 AND is_active",
        ).bind(id).fetch_one(&mut *tx).await?;
        if count >= MAX_PARTICIPANTS {
            return Err(ExpeditionError::PartyFull);
        }
        sqlx::query("INSERT INTO expedition_participants (expedition_run_id, discord_user_id) VALUES ($1, $2)")
            .bind(id).bind(player).execute(&mut *tx).await?;
        sqlx::query(
            "UPDATE expedition_runs SET row_version = row_version + 1, updated_at = now()
             WHERE expedition_run_id = $1",
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        self.snapshot(player, Some(guild), id).await
    }

    pub async fn begin(
        &self,
        player: &str,
        guild: Option<&str>,
        id: i64,
    ) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let row = lock_run(&mut tx, id).await?;
        ensure_supported_route(&row)?;
        scope(&row.guild_id, guild)?;
        if row.host.as_deref() != Some(player) {
            return Err(ExpeditionError::NotHost);
        }
        if row.state != "recruiting" {
            return Err(ExpeditionError::NotRecruiting);
        }
        let members = active_members(&mut tx, id).await?;
        for member in &members {
            ensure_at_start(&mut tx, member).await?;
        }
        sqlx::query("UPDATE expedition_runs SET state = 'active', started_at = now(), row_version = row_version + 1, updated_at = now() WHERE expedition_run_id = $1")
            .bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    /// Read whether this player completed Lantern Road as an active participant.
    /// This powers the Almanac archive only; it creates no additional reward.
    pub async fn has_completed_lantern_road(&self, player: &str) -> Result<bool, ExpeditionError> {
        Ok(sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1
                FROM expedition_runs AS runs
                JOIN expedition_participants AS participants
                  ON participants.expedition_run_id = runs.expedition_run_id
                WHERE runs.route_id = $2
                  AND runs.state = 'completed'
                  AND participants.discord_user_id = $1
                  AND participants.is_active = FALSE
                  AND participants.last_checkpoint_index = 3
            )",
        )
        .bind(player)
        .bind(ROUTE_ID)
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn snapshot(
        &self,
        player: &str,
        guild: Option<&str>,
        id: i64,
    ) -> Result<Snapshot, ExpeditionError> {
        let row = load_run(&self.pool, id).await?;
        let route = ensure_supported_route(&row)?;
        scope(&row.guild_id, guild)?;
        let member: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM expedition_participants WHERE expedition_run_id = $1 AND discord_user_id = $2)",
        ).bind(id).bind(player).fetch_one(&self.pool).await?;
        if !member {
            return Err(ExpeditionError::NotParticipant);
        }
        let index =
            usize::try_from(row.checkpoint_index).map_err(|_| ExpeditionError::UnknownRoute)?;
        let current = *route
            .checkpoints
            .get(index)
            .ok_or(ExpeditionError::UnknownRoute)?;
        let next = route.checkpoints.get(index + 1).copied();
        let participants = sqlx::query_as::<_, (String, bool)>(
            "SELECT discord_user_id, is_active FROM expedition_participants WHERE expedition_run_id = $1 ORDER BY joined_at ASC, discord_user_id ASC",
        ).bind(id).fetch_all(&self.pool).await?
         .into_iter().map(|(discord_user_id, active)| Participant {
             is_host: row.host.as_deref() == Some(discord_user_id.as_str()),
             discord_user_id, active,
         }).collect();
        Ok(Snapshot {
            run_id: id,
            route_id: row.route_id,
            route_title: route.title,
            route_checkpoint_count: route.checkpoints.len(),
            content_version: row.content_version,
            guild_id: row.guild_id,
            shared: row.shared,
            host: row.host,
            state: row.state,
            checkpoint_index: row.checkpoint_index,
            row_version: row.row_version,
            current,
            next,
            participants,
        })
    }

    pub async fn advance(
        &self,
        player: &str,
        guild: Option<&str>,
        id: i64,
        expected: &str,
        choice: &str,
    ) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        // Checkpoint advancement writes Atlas, Chronicle, inventory and Wayfarer
        // records for every active participant. Lock their accounts in stable ID
        // order before the run row, matching account deletion's user -> run order.
        let run_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM expedition_runs WHERE expedition_run_id = $1)",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;
        if !run_exists {
            return Err(ExpeditionError::RunNotFound);
        }
        let protected_members: Vec<String> = sqlx::query_scalar(
            "SELECT discord_user_id FROM expedition_participants WHERE expedition_run_id = $1 AND is_active ORDER BY discord_user_id ASC",
        )
        .bind(id)
        .fetch_all(&mut *tx)
        .await?;
        if !protected_members.iter().any(|member| member == player) {
            return Err(ExpeditionError::NotActiveParticipant);
        }
        for member in &protected_members {
            lock_player(&mut tx, member).await?;
        }
        let row = lock_run(&mut tx, id).await?;
        let route = ensure_supported_route(&row)?;
        scope(&row.guild_id, guild)?;
        if row.state != "active" {
            return Err(ExpeditionError::NotActive);
        }
        let members = active_members(&mut tx, id).await?;
        let mut current_members = members.clone();
        current_members.sort_unstable();
        if current_members != protected_members {
            return Err(ExpeditionError::PartyChanged);
        }
        if !members.iter().any(|m| m == player) {
            return Err(ExpeditionError::NotActiveParticipant);
        }
        let next_index =
            usize::try_from(row.checkpoint_index).map_err(|_| ExpeditionError::UnknownRoute)? + 1;
        let next = *route
            .checkpoints
            .get(next_index)
            .ok_or(ExpeditionError::NotActive)?;
        if next.id != expected {
            return Err(ExpeditionError::StaleCheckpoint);
        }
        validate_checkpoint_choice(&next, choice)?;
        let previous = route
            .checkpoints
            .get(next_index - 1)
            .ok_or(ExpeditionError::UnknownRoute)?;
        for member in &members {
            atlas::apply_expedition_travel(&mut tx, member, previous.location_id, next.location_id)
                .await?;
        }
        let checkpoint_index =
            i16::try_from(next_index).map_err(|_| ExpeditionError::UnknownRoute)?;
        sqlx::query(
            "INSERT INTO expedition_checkpoint_history (expedition_run_id, content_version, checkpoint_index, checkpoint_id, choice_id, completed_by)
             VALUES ($1, $2, $3, $4, $5, $6)",
        ).bind(id).bind(route.version).bind(checkpoint_index).bind(next.id)
         .bind(choice.trim().to_ascii_lowercase()).bind(player).execute(&mut *tx).await?;
        // The first Guild Project is specifically a Lantern Road Survey.
        // Other shared routes must never contribute to its server-owned board.
        if row.shared
            && row.route_id == ROUTE_ID
            && let Some(guild_id) = row.guild_id.as_deref()
        {
            self.guild_projects
                .record_shared_checkpoint(&mut tx, guild_id, player, id, next.id)
                .await?;
        }
        sqlx::query("UPDATE expedition_participants SET last_checkpoint_index = $2 WHERE expedition_run_id = $1 AND is_active")
            .bind(id).bind(checkpoint_index).execute(&mut *tx).await?;
        let completed = next_index == route.checkpoints.len() - 1;
        sqlx::query(
            "UPDATE expedition_runs SET current_checkpoint_index = $2,
             state = CASE WHEN $3 THEN 'completed' ELSE 'active' END,
             completed_at = CASE WHEN $3 THEN now() ELSE completed_at END,
             row_version = row_version + 1, updated_at = now()
             WHERE expedition_run_id = $1",
        )
        .bind(id)
        .bind(checkpoint_index)
        .bind(completed)
        .execute(&mut *tx)
        .await?;
        if completed {
            sqlx::query("UPDATE expedition_participants SET is_active = FALSE, left_at = now() WHERE expedition_run_id = $1 AND is_active")
                .bind(id).execute(&mut *tx).await?;
            for member in &members {
                crate::wayfarer::record_expedition_completion(&mut tx, member).await?;
            }
        }
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn pause(
        &self,
        player: &str,
        guild: Option<&str>,
        id: i64,
    ) -> Result<Snapshot, ExpeditionError> {
        self.change_state(player, guild, id, "active", "paused")
            .await
    }
    pub async fn resume(
        &self,
        player: &str,
        guild: Option<&str>,
        id: i64,
    ) -> Result<Snapshot, ExpeditionError> {
        self.change_state(player, guild, id, "paused", "active")
            .await
    }
    async fn change_state(
        &self,
        player: &str,
        guild: Option<&str>,
        id: i64,
        expected: &str,
        next: &str,
    ) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let row = lock_run(&mut tx, id).await?;
        ensure_supported_route(&row)?;
        scope(&row.guild_id, guild)?;
        if !is_active_member(&mut tx, id, player).await? {
            return Err(ExpeditionError::NotActiveParticipant);
        }
        if row.state != expected {
            return Err(if expected == "active" {
                ExpeditionError::NotActive
            } else {
                ExpeditionError::NotPaused
            });
        }
        sqlx::query("UPDATE expedition_runs SET state = $2, row_version = row_version + 1, updated_at = now() WHERE expedition_run_id = $1")
            .bind(id).bind(next).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn leave(
        &self,
        player: &str,
        guild: Option<&str>,
        id: i64,
    ) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        // Keep membership changes in the same user -> run lock order as deletion.
        lock_player(&mut tx, player).await?;
        let row = lock_run(&mut tx, id).await?;
        ensure_supported_route(&row)?;
        scope(&row.guild_id, guild)?;
        if matches!(row.state.as_str(), "completed" | "cancelled") {
            return Err(ExpeditionError::NotActive);
        }
        if !is_active_member(&mut tx, id, player).await? {
            return Err(ExpeditionError::NotActiveParticipant);
        }
        sqlx::query("UPDATE expedition_participants SET is_active = FALSE, left_at = now() WHERE expedition_run_id = $1 AND discord_user_id = $2")
            .bind(id).bind(player).execute(&mut *tx).await?;
        let next_host: Option<String> = sqlx::query_scalar(
            "SELECT discord_user_id FROM expedition_participants WHERE expedition_run_id = $1 AND is_active ORDER BY joined_at ASC, discord_user_id ASC LIMIT 1",
        ).bind(id).fetch_optional(&mut *tx).await?;
        let empty = next_host.is_none();
        let host = if row.host.as_deref() == Some(player) {
            next_host.as_deref()
        } else {
            row.host.as_deref()
        };
        sqlx::query(
            "UPDATE expedition_runs SET host_discord_user_id = $2,
             state = CASE WHEN $3 THEN 'cancelled' ELSE state END,
             cancelled_at = CASE WHEN $3 THEN COALESCE(cancelled_at, now()) ELSE cancelled_at END,
             row_version = row_version + 1, updated_at = now() WHERE expedition_run_id = $1",
        )
        .bind(id)
        .bind(host)
        .bind(empty)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn cancel(
        &self,
        player: &str,
        guild: Option<&str>,
        id: i64,
    ) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let row = lock_run(&mut tx, id).await?;
        ensure_supported_route(&row)?;
        scope(&row.guild_id, guild)?;
        if row.host.as_deref() != Some(player) {
            return Err(ExpeditionError::NotHost);
        }
        if matches!(row.state.as_str(), "completed" | "cancelled") {
            return Err(ExpeditionError::NotActive);
        }
        sqlx::query("UPDATE expedition_participants SET is_active = FALSE, left_at = COALESCE(left_at, now()) WHERE expedition_run_id = $1 AND is_active")
            .bind(id).execute(&mut *tx).await?;
        sqlx::query("UPDATE expedition_runs SET state = 'cancelled', cancelled_at = now(), row_version = row_version + 1, updated_at = now() WHERE expedition_run_id = $1")
            .bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }
}

struct RunRow {
    route_id: String,
    content_version: i16,
    guild_id: Option<String>,
    host: Option<String>,
    shared: bool,
    state: String,
    checkpoint_index: i16,
    row_version: i64,
}
fn ensure_supported_route(row: &RunRow) -> Result<&'static RouteDefinition, ExpeditionError> {
    route_definition(&row.route_id)
        .filter(|route| route.version == row.content_version)
        .ok_or(ExpeditionError::UnknownRoute)
}

async fn load_run(pool: &PgPool, id: i64) -> Result<RunRow, ExpeditionError> {
    query_run("SELECT route_id, content_version, guild_id, host_discord_user_id, shared, state, current_checkpoint_index, row_version FROM expedition_runs WHERE expedition_run_id = $1", pool, id).await
}
async fn lock_run(tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<RunRow, ExpeditionError> {
    let values = sqlx::query_as::<_, (String, i16, Option<String>, Option<String>, bool, String, i16, i64)>(
        "SELECT route_id, content_version, guild_id, host_discord_user_id, shared, state, current_checkpoint_index, row_version FROM expedition_runs WHERE expedition_run_id = $1 FOR UPDATE",
    ).bind(id).fetch_optional(&mut **tx).await?.ok_or(ExpeditionError::RunNotFound)?;
    Ok(to_run_row(values))
}
async fn query_run(sql: &str, pool: &PgPool, id: i64) -> Result<RunRow, ExpeditionError> {
    let values = sqlx::query_as::<
        _,
        (
            String,
            i16,
            Option<String>,
            Option<String>,
            bool,
            String,
            i16,
            i64,
        ),
    >(sql)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ExpeditionError::RunNotFound)?;
    Ok(to_run_row(values))
}
fn to_run_row(
    v: (
        String,
        i16,
        Option<String>,
        Option<String>,
        bool,
        String,
        i16,
        i64,
    ),
) -> RunRow {
    RunRow {
        route_id: v.0,
        content_version: v.1,
        guild_id: v.2,
        host: v.3,
        shared: v.4,
        state: v.5,
        checkpoint_index: v.6,
        row_version: v.7,
    }
}
async fn active_members(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
) -> Result<Vec<String>, ExpeditionError> {
    Ok(sqlx::query_scalar("SELECT discord_user_id FROM expedition_participants WHERE expedition_run_id = $1 AND is_active ORDER BY joined_at ASC, discord_user_id ASC FOR UPDATE")
        .bind(id).fetch_all(&mut **tx).await?)
}
async fn is_active_member(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    player: &str,
) -> Result<bool, ExpeditionError> {
    Ok(sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM expedition_participants WHERE expedition_run_id = $1 AND discord_user_id = $2 AND is_active)")
        .bind(id).bind(player).fetch_one(&mut **tx).await?)
}
async fn lock_player(tx: &mut Transaction<'_, Postgres>, player: &str) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('moonbyte:user:' || $1, 0))")
        .bind(player)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

async fn ensure_at_start(
    tx: &mut Transaction<'_, Postgres>,
    player: &str,
) -> Result<(), ExpeditionError> {
    let current: Option<String> = sqlx::query_scalar(
        "SELECT current_location_id FROM player_world_state WHERE discord_user_id = $1 FOR UPDATE",
    )
    .bind(player)
    .fetch_optional(&mut **tx)
    .await?;
    if current.as_deref() != Some("lantern-square") {
        return Err(ExpeditionError::NotAtStart);
    }
    Ok(())
}
fn scope(saved: &Option<String>, requested: Option<&str>) -> Result<(), ExpeditionError> {
    if saved.as_deref() == requested {
        Ok(())
    } else {
        Err(ExpeditionError::WrongGuild)
    }
}
fn validate_checkpoint_choice(
    checkpoint: &Checkpoint,
    choice: &str,
) -> Result<(), ExpeditionError> {
    let choice = choice.trim().to_ascii_lowercase();
    if !checkpoint.choices.contains(&choice.as_str()) {
        return Err(ExpeditionError::InvalidChoice);
    }
    if checkpoint
        .correct_choice
        .is_some_and(|correct| correct != choice)
    {
        return Err(ExpeditionError::IncorrectAnswer);
    }
    Ok(())
}

#[cfg(test)]
fn validate_choice(index: usize, choice: &str) -> Result<(), ExpeditionError> {
    let checkpoint = LANTERN_ROAD
        .get(index)
        .ok_or(ExpeditionError::UnknownRoute)?;
    validate_checkpoint_choice(checkpoint, choice)
}

pub fn render_catalogue(prefix: &str) -> String {
    let p = safe_prefix(prefix);
    let routes = EXPEDITION_ROUTES
        .iter()
        .map(|route| {
            format!(
                "**{}** (`{}`, v{}) — {}\n{} checkpoints · Solo or up to four players in one server · No deadline or attendance penalty\nStart solo: {p}expedition start {} solo\nCreate a group: {p}expedition start {} group",
                route.title,
                route.id,
                route.version,
                route.summary,
                route.checkpoints.len(),
                route.id,
                route.id
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    format!("**Expedition Routes**\n\n{routes}")
}

pub fn render(snapshot: &Snapshot, prefix: &str) -> String {
    let p = safe_prefix(prefix);
    let participants = snapshot
        .participants
        .iter()
        .map(|member| {
            let active = if member.active {
                "active"
            } else {
                "left / finished"
            };
            let host = if member.is_host { " · host" } else { "" };
            format!("• <@{}>{host} · {active}", member.discord_user_id)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let next = snapshot.next.map_or_else(
        || "The final checkpoint is recorded.".to_owned(),
        |checkpoint| {
            format!(
                "**Next — {}**\n{}\nChoices: {}",
                checkpoint.title,
                checkpoint.prompt,
                checkpoint.choices.join(" · ")
            )
        },
    );
    let actions = match snapshot.state.as_str() {
        "recruiting" => format!(
            "{p}expedition begin {}\n{p}expedition join {}",
            snapshot.run_id, snapshot.run_id
        ),
        "active" if snapshot.next.is_some() => format!(
            "{p}expedition advance {} {} <choice>\n{p}expedition pause {}\n{p}expedition leave {}",
            snapshot.run_id,
            snapshot.next.map(|checkpoint| checkpoint.id).unwrap_or(""),
            snapshot.run_id,
            snapshot.run_id
        ),
        "paused" => format!(
            "{p}expedition resume {}\n{p}expedition leave {}",
            snapshot.run_id, snapshot.run_id
        ),
        "completed" => format!(
            "{} is complete. Your Atlas and private Chronicle remain yours.",
            snapshot.route_title
        ),
        "cancelled" => "This run was cancelled. Completed discoveries remain.".to_owned(),
        _ => "No action is available.".to_owned(),
    };
    let place = LOCATION_CATALOGUE
        .iter()
        .find(|location| location.id == snapshot.current.location_id)
        .map(|location| location.name)
        .unwrap_or("Unknown");
    format!(
        "**{} · Expedition #{}**\n\nRoute: `{}` · State: **{}** · Checkpoint: **{}/{}** · Route v{}\nCurrent place: **{}**\nParticipants:\n{}\n\n{}\n\n{}",
        snapshot.route_title,
        snapshot.run_id,
        snapshot.route_id,
        snapshot.state,
        snapshot.checkpoint_index + 1,
        snapshot.route_checkpoint_count,
        snapshot.content_version,
        place,
        participants,
        next,
        actions
    )
}
fn safe_prefix(prefix: &str) -> String {
    prefix
        .replace('@', "@\u{200B}")
        .replace(char::from(96), "ˋ")
        .replace('\\', "＼")
}

#[cfg(test)]
mod tests {
    use super::{
        EXPEDITION_ROUTES, ExpeditionError, GLASSWATER_CROSSING, GLASSWATER_ROUTE_ID, LANTERN_ROAD,
        ROUTE_ID, route_definition, validate_checkpoint_choice, validate_choice,
    };
    use crate::atlas::LOCATION_CATALOGUE;

    #[test]
    fn lantern_road_uses_existing_reciprocal_atlas_edges() {
        assert_eq!(ROUTE_ID, "lantern-road");
        for pair in LANTERN_ROAD.windows(2) {
            let from = LOCATION_CATALOGUE
                .iter()
                .find(|p| p.id == pair[0].location_id)
                .unwrap();
            let to = LOCATION_CATALOGUE
                .iter()
                .find(|p| p.id == pair[1].location_id)
                .unwrap();
            assert!(from.routes_to.contains(&pair[1].location_id));
            assert!(to.routes_to.contains(&pair[0].location_id));
        }
    }

    #[test]
    fn every_route_uses_existing_reciprocal_atlas_edges() {
        for route in EXPEDITION_ROUTES {
            assert_eq!(route.checkpoints.len(), 4);
            for pair in route.checkpoints.windows(2) {
                let from = LOCATION_CATALOGUE
                    .iter()
                    .find(|location| location.id == pair[0].location_id)
                    .expect("known route source");
                let to = LOCATION_CATALOGUE
                    .iter()
                    .find(|location| location.id == pair[1].location_id)
                    .expect("known route destination");
                assert!(
                    from.routes_to.contains(&to.id),
                    "{} route edge missing",
                    route.id
                );
                assert!(
                    to.routes_to.contains(&from.id),
                    "{} route edge is not reciprocal",
                    route.id
                );
            }
        }
    }

    #[test]
    fn route_registry_rejects_unknown_ids_and_pins_each_content_version() {
        assert_eq!(
            route_definition(ROUTE_ID).map(|route| route.version),
            Some(1)
        );
        assert_eq!(
            route_definition(GLASSWATER_ROUTE_ID).map(|route| route.version),
            Some(1)
        );
        assert!(route_definition("unknown-road").is_none());
    }

    #[test]
    fn glasswater_chart_requires_the_reviewed_western_direction() {
        let chart = GLASSWATER_CROSSING
            .iter()
            .find(|checkpoint| checkpoint.id == "glasswater-star-chart")
            .expect("Glasswater clue exists");
        assert!(matches!(validate_checkpoint_choice(chart, "west"), Ok(())));
        assert!(matches!(
            validate_checkpoint_choice(chart, "east"),
            Err(ExpeditionError::IncorrectAnswer)
        ));
        assert!(matches!(
            validate_checkpoint_choice(chart, "north"),
            Err(ExpeditionError::InvalidChoice)
        ));
    }

    #[test]
    fn archive_clue_requires_star() {
        assert!(validate_choice(2, "star").is_ok());
        assert!(matches!(
            validate_choice(2, "lantern"),
            Err(ExpeditionError::IncorrectAnswer)
        ));
        assert!(matches!(
            validate_choice(1, "star"),
            Err(ExpeditionError::InvalidChoice)
        ));
    }

    async fn wait_for_expedition_lock_waiters(
        pool: &sqlx::PgPool,
        lock_key: i64,
        expected_waiters: i64,
    ) {
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            loop {
                let waiters: i64 = sqlx::query_scalar(
                    "SELECT count(*) FROM pg_locks WHERE locktype = 'advisory' AND classid = (($1::bigint >> 32) & 4294967295)::oid AND objid = ($1::bigint & 4294967295)::oid AND objsubid = 1 AND NOT granted",
                )
                .bind(lock_key)
                .fetch_one(pool)
                .await
                .expect("inspect Expedition account-lock waiters");
                if waiters >= expected_waiters {
                    return;
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("Expedition operations queued behind the held account lock");
    }

    #[tokio::test]
    async fn shared_lantern_road_is_transactional_private_and_deletable() {
        use std::time::{SystemTime, UNIX_EPOCH};

        use crate::{
            atlas::AtlasStore, database::Database, profile::ProfileStore, wayfarer::WayfarerStore,
        };

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must point to the CI PostgreSQL service");
        let database = Database::connect(&database_url)
            .await
            .expect("connect and migrate PostgreSQL");
        let pool = database.pool();
        let profiles = ProfileStore::new(pool.clone());
        let atlas = AtlasStore::new(pool.clone());
        let wayfarer = WayfarerStore::new(pool.clone());
        let store = super::ExpeditionStore::new(pool.clone());
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
            % 1_000_000_000_000_000_000;
        let host_id = format!("exp-host-{nonce:x}");
        let member_id = format!("exp-member-{nonce:x}");
        let outsider_id = format!("exp-outsider-{nonce:x}");
        let guild_id = format!("exp-guild-{nonce:x}");

        crate::profile::create_test_player(pool.clone(), &host_id, "Expedition Host")
            .await
            .expect("create consented host profile");
        crate::profile::create_test_player(pool.clone(), &member_id, "Expedition Member")
            .await
            .expect("create consented member profile");
        wayfarer
            .daily_quests(&member_id)
            .await
            .expect("create member's daily quest rotation");
        sqlx::query(
            "INSERT INTO player_daily_quests
             (discord_user_id, quest_date, quest_id, status, progress, target, reward_moonbits)
             VALUES ($1, CURRENT_DATE, 'complete_one_expedition', 'active', 0, 1, 10)
             ON CONFLICT (discord_user_id, quest_date, quest_id)
             DO UPDATE SET status = 'active', progress = 0, target = 1,
                           completed_at = NULL, claimed_at = NULL",
        )
        .bind(&member_id)
        .execute(&pool)
        .await
        .expect("activate Expedition quest for integration test");
        crate::profile::create_test_player(pool.clone(), &outsider_id, "Expedition Observer")
            .await
            .expect("create consented observer profile");
        atlas.snapshot(&host_id).await.expect("host starting place");
        atlas
            .snapshot(&member_id)
            .await
            .expect("member starting place");
        atlas
            .snapshot(&outsider_id)
            .await
            .expect("outsider starting place");

        let recruiting = store
            .start(&host_id, Some(&guild_id), true)
            .await
            .expect("create recruiting group run");
        assert_eq!(recruiting.state, "recruiting");
        assert!(matches!(
            store
                .snapshot(&outsider_id, Some(&guild_id), recruiting.run_id)
                .await,
            Err(super::ExpeditionError::NotParticipant)
        ));
        assert!(matches!(
            store
                .snapshot(&host_id, Some("another-guild"), recruiting.run_id)
                .await,
            Err(super::ExpeditionError::WrongGuild)
        ));
        assert!(matches!(
            atlas.travel(&host_id, "mosslight-grove").await,
            Err(crate::atlas::AtlasError::ExpeditionInProgress)
        ));

        let joined = store
            .join(&member_id, &guild_id, recruiting.run_id)
            .await
            .expect("member explicitly joins");
        assert_eq!(joined.participants.len(), 2);
        assert!(matches!(
            store
                .begin(&member_id, Some(&guild_id), recruiting.run_id)
                .await,
            Err(super::ExpeditionError::NotHost)
        ));
        store
            .begin(&host_id, Some(&guild_id), recruiting.run_id)
            .await
            .expect("host begins group run");

        store
            .pause(&member_id, Some(&guild_id), recruiting.run_id)
            .await
            .expect("participant can pause");
        assert!(matches!(
            store
                .advance(
                    &host_id,
                    Some(&guild_id),
                    recruiting.run_id,
                    "mosslight-grove",
                    "inspect"
                )
                .await,
            Err(super::ExpeditionError::NotActive)
        ));
        store
            .resume(&host_id, Some(&guild_id), recruiting.run_id)
            .await
            .expect("participant can resume");

        let (host_advance, member_advance) = tokio::join!(
            store.advance(
                &host_id,
                Some(&guild_id),
                recruiting.run_id,
                "mosslight-grove",
                "inspect",
            ),
            store.advance(
                &member_id,
                Some(&guild_id),
                recruiting.run_id,
                "mosslight-grove",
                "listen",
            ),
        );
        let grove = match (host_advance, member_advance) {
            (Ok(snapshot), Err(ExpeditionError::StaleCheckpoint))
            | (Err(ExpeditionError::StaleCheckpoint), Ok(snapshot)) => snapshot,
            (host_result, member_result) => panic!(
                "exactly one checkpoint attempt must commit; host={host_result:?}, member={member_result:?}"
            ),
        };
        assert_eq!(grove.checkpoint_index, 1);
        assert!(matches!(
            store
                .advance(
                    &member_id,
                    Some(&guild_id),
                    recruiting.run_id,
                    "ember-archive",
                    "lantern"
                )
                .await,
            Err(super::ExpeditionError::IncorrectAnswer)
        ));
        assert!(matches!(
            store
                .advance(
                    &member_id,
                    Some(&guild_id),
                    recruiting.run_id,
                    "starfall-observatory",
                    "chart"
                )
                .await,
            Err(super::ExpeditionError::StaleCheckpoint)
        ));

        let host_atlas_before_deletion = atlas
            .snapshot(&host_id)
            .await
            .expect("host's Atlas before deletion");
        let member_atlas_before_deletion = atlas
            .snapshot(&member_id)
            .await
            .expect("member's Atlas before deletion");
        assert_eq!(
            host_atlas_before_deletion.current_location.id,
            "mosslight-grove"
        );
        assert_eq!(
            member_atlas_before_deletion.current_location.id,
            "mosslight-grove"
        );

        let host_deletion = profiles
            .delete_account(&host_id)
            .await
            .expect("delete the host without deleting the remaining participant's run");
        assert_eq!(host_deletion.expedition_participations_removed, 1);
        assert_eq!(host_deletion.expedition_runs_host_transferred, 1);
        assert_eq!(host_deletion.expedition_runs_cancelled, 0);
        assert_eq!(host_deletion.expedition_runs_removed, 0);
        let member_run = store
            .snapshot(&member_id, Some(&guild_id), recruiting.run_id)
            .await
            .expect("remaining participant retains the active run");
        assert_eq!(member_run.state, "active");
        assert_eq!(member_run.host.as_deref(), Some(member_id.as_str()));

        store
            .advance(
                &member_id,
                Some(&guild_id),
                recruiting.run_id,
                "ember-archive",
                "star",
            )
            .await
            .expect("solve the archive clue");
        let completed = store
            .advance(
                &member_id,
                Some(&guild_id),
                recruiting.run_id,
                "starfall-observatory",
                "chart",
            )
            .await
            .expect("complete the route");
        assert_eq!(completed.state, "completed");
        assert_eq!(completed.checkpoint_index, 3);
        assert!(
            store
                .has_completed_lantern_road(&member_id)
                .await
                .expect("read completed Expedition archive evidence")
        );
        assert!(
            !store
                .has_completed_lantern_road(&outsider_id)
                .await
                .expect("read unrelated player's archive evidence")
        );
        let expedition_quest: (String, i16, i16) = sqlx::query_as(
            "SELECT status, progress, target FROM player_daily_quests
             WHERE discord_user_id = $1 AND quest_date = CURRENT_DATE
               AND quest_id = 'complete_one_expedition'",
        )
        .bind(&member_id)
        .fetch_one(&pool)
        .await
        .expect("read Expedition quest progress");
        assert_eq!(expedition_quest, (String::from("completed"), 1, 1));

        let member_atlas = atlas
            .snapshot(&member_id)
            .await
            .expect("member's saved Atlas");
        assert_eq!(member_atlas.current_location.id, "starfall-observatory");
        assert!(
            wayfarer
                .snapshot(&member_id)
                .await
                .expect("Wayfarer record")
                .active_days
                >= 1
        );

        let inventory = profiles
            .inventory(&member_id)
            .await
            .expect("Expedition data inventory");
        assert_eq!(inventory.expedition_run_records, 1);
        assert_eq!(inventory.expedition_checkpoint_records, 3);
        let export = profiles
            .export_json(&member_id)
            .await
            .expect("Expedition export");
        assert!(export.contains("lantern-road"));
        assert!(export.contains("ember-archive"));
        assert!(export.contains(&recruiting.run_id.to_string()));
        assert!(export.contains("\"completed_by_you\": true"));

        let member_deletion = profiles
            .delete_account(&member_id)
            .await
            .expect("remove final participant and orphaned run history");
        assert_eq!(member_deletion.expedition_participations_removed, 1);
        assert_eq!(member_deletion.expedition_runs_removed, 1);
        let glasswater = store
            .start_route(&outsider_id, None, GLASSWATER_ROUTE_ID, false)
            .await
            .expect("start Glasswater Crossing");
        assert_eq!(glasswater.route_id, GLASSWATER_ROUTE_ID);
        assert_eq!(glasswater.route_title, "The Glasswater Crossing");
        store
            .advance(
                &outsider_id,
                None,
                glasswater.run_id,
                "glasswater-pier",
                "survey",
            )
            .await
            .expect("survey Glasswater Pier");
        let chart = store
            .advance(
                &outsider_id,
                None,
                glasswater.run_id,
                "glasswater-star-chart",
                "west",
            )
            .await
            .expect("follow western chart");
        assert_eq!(chart.current.location_id, "starfall-observatory");
        let completed_crossing = store
            .advance(
                &outsider_id,
                None,
                glasswater.run_id,
                "mosslight-return",
                "record",
            )
            .await
            .expect("complete Glasswater Crossing");
        assert_eq!(completed_crossing.state, "completed");
        assert!(
            !store
                .has_completed_lantern_road(&outsider_id)
                .await
                .expect("Glasswater does not count as a Lantern Road archive record")
        );
        let crossing_history: Vec<String> = sqlx::query_scalar(
            "SELECT checkpoint_id FROM expedition_checkpoint_history
             WHERE expedition_run_id = $1 ORDER BY checkpoint_index",
        )
        .bind(glasswater.run_id)
        .fetch_all(&pool)
        .await
        .expect("read Glasswater checkpoint history");
        assert_eq!(
            crossing_history,
            vec![
                String::from("glasswater-pier"),
                String::from("glasswater-star-chart"),
                String::from("mosslight-return")
            ]
        );
        let crossing_export = profiles
            .export_json(&outsider_id)
            .await
            .expect("export Glasswater route history");
        assert!(crossing_export.contains("glasswater-crossing"));
        assert!(crossing_export.contains("glasswater-star-chart"));

        profiles
            .delete_account(&outsider_id)
            .await
            .expect("remove test observer and second-route history");
    }

    #[tokio::test]
    async fn checkpoint_advancement_waits_for_member_deletion_before_writing_progress() {
        use std::time::{SystemTime, UNIX_EPOCH};

        use crate::{atlas::AtlasStore, database::Database, profile::ProfileStore};

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must point to the CI PostgreSQL service");
        let database = Database::connect(&database_url)
            .await
            .expect("connect and migrate PostgreSQL");
        let pool = database.pool();
        let profiles = ProfileStore::new(pool.clone());
        let atlas = AtlasStore::new(pool.clone());
        let store = super::ExpeditionStore::new(pool.clone());
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let player_id = format!("exp-delete-race-{nonce}");

        crate::profile::create_test_player(pool.clone(), &player_id, "Expedition Deletion")
            .await
            .expect("create a consented player");
        atlas
            .snapshot(&player_id)
            .await
            .expect("initialise Atlas starting point");
        let run = store
            .start(&player_id, None, false)
            .await
            .expect("start a solo Expedition");

        let lock_key: i64 =
            sqlx::query_scalar("SELECT hashtextextended('moonbyte:user:' || $1, 0)")
                .bind(&player_id)
                .fetch_one(&pool)
                .await
                .expect("derive the per-player advisory lock key");
        let mut blocker = pool.begin().await.expect("begin account-lock blocker");
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(lock_key)
            .execute(&mut *blocker)
            .await
            .expect("hold the account advisory lock");

        let deletion_profiles = profiles.clone();
        let deletion_player = player_id.clone();
        let deletion = tokio::spawn(async move {
            deletion_profiles.delete_account(&deletion_player).await
        });
        wait_for_expedition_lock_waiters(&pool, lock_key, 1).await;

        let stale_store = store.clone();
        let stale_player = player_id.clone();
        let stale_run_id = run.run_id;
        let advancement = tokio::spawn(async move {
            stale_store
                .advance(
                    &stale_player,
                    None,
                    stale_run_id,
                    "mosslight-grove",
                    "inspect",
                )
                .await
        });
        wait_for_expedition_lock_waiters(&pool, lock_key, 2).await;

        blocker
            .commit()
            .await
            .expect("release the queued account operations");
        let deletion = deletion
            .await
            .expect("join account deletion")
            .expect("complete account deletion");
        assert!(deletion.profile_removed);
        assert_eq!(deletion.expedition_runs_removed, 1);

        let advancement = advancement
            .await
            .expect("join stale Expedition advancement");
        assert!(
            matches!(advancement, Err(super::ExpeditionError::RunNotFound)),
            "the queued checkpoint must re-read the run after deletion commits"
        );
        assert!(
            profiles
                .get(&player_id)
                .await
                .expect("inspect deleted profile")
                .is_none()
        );
        let remaining: (i64, i64, i64, i64) = sqlx::query_as(
            "SELECT
                (SELECT count(*) FROM player_atlas_discoveries WHERE discord_user_id = $1),
                (SELECT count(*) FROM player_archive_fragments WHERE discord_user_id = $1),
                (SELECT count(*) FROM player_inventory_items WHERE discord_user_id = $1),
                (SELECT count(*) FROM player_wayfarer_days WHERE discord_user_id = $1)",
        )
        .bind(&player_id)
        .fetch_one(&pool)
        .await
        .expect("inspect account-owned progression after deletion");
        assert_eq!(remaining, (0, 0, 0, 0));
    }
}
