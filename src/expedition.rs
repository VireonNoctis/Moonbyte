//! Persistent Lantern Road runs. Domain state is authoritative in PostgreSQL.

use std::fmt;
use sqlx::{PgPool, Postgres, Transaction};
use crate::atlas::{self, AtlasError, LOCATION_CATALOGUE};

pub const ROUTE_ID: &str = "lantern-road";
pub const ROUTE_VERSION: i16 = 1;
pub const MAX_PARTICIPANTS: i64 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    pub id: &'static str,
    pub location_id: &'static str,
    pub title: &'static str,
    pub prompt: &'static str,
    pub choices: &'static [&'static str],
}

pub const LANTERN_ROAD: [Checkpoint; 4] = [
    Checkpoint { id: "lantern-square", location_id: "lantern-square", title: "The Lantern Square Departure", prompt: "The road begins where the first lantern was lit. Check the route and prepare to set out.", choices: &["ready"] },
    Checkpoint { id: "mosslight-grove", location_id: "mosslight-grove", title: "The Mosslight Observation", prompt: "The old stones hold the grove's stories. Choose how to make your first observation.", choices: &["listen", "inspect"] },
    Checkpoint { id: "ember-archive", location_id: "ember-archive", title: "The Ember Archive Riddle", prompt: "The keeper shows two marks on the old road chart: a lantern and a star. Which mark points toward the Starfall Observatory?", choices: &["star", "lantern"] },
    Checkpoint { id: "starfall-observatory", location_id: "starfall-observatory", title: "The Starfall Chart", prompt: "The observatory is reached. Set the final chart in its place to complete the Lantern Road.", choices: &["chart"] },
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Participant {
    pub discord_user_id: String,
    pub active: bool,
    pub is_host: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    pub run_id: i64,
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
            Self::NotAtStart => f.write_str("You must be at Lantern Square before starting or joining the Lantern Road."),
            Self::RunNotFound => f.write_str("That Expedition run was not found."),
            Self::NotParticipant => f.write_str("Only a participant can view that private run."),
            Self::NotActiveParticipant => f.write_str("You are not an active participant in that run."),
            Self::AlreadyInRun => f.write_str("You already have an active Expedition. Leave or finish it before joining another."),
            Self::NotRecruiting => f.write_str("That run is no longer accepting participants."),
            Self::NotActive => f.write_str("That run is not active."),
            Self::NotPaused => f.write_str("That run is not paused."),
            Self::NotHost => f.write_str("Only the current host can start or cancel this group run."),
            Self::PartyFull => f.write_str("The Lantern Road supports at most four participants."),
            Self::StaleCheckpoint => f.write_str("That checkpoint is no longer next. Refresh the run status before advancing."),
            Self::InvalidChoice => f.write_str("That choice is not available at this checkpoint."),
            Self::IncorrectAnswer => f.write_str("That clue does not match the chart. Nothing advanced; review the archive clue and try again."),
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
    fn from(error: AtlasError) -> Self { Self::Atlas(error) }
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
pub struct ExpeditionStore { pool: PgPool }

impl ExpeditionStore {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn start(&self, player: &str, guild: Option<&str>, shared: bool) -> Result<Snapshot, ExpeditionError> {
        if shared && guild.is_none() { return Err(ExpeditionError::SharedRequiresGuild); }
        let mut tx = self.pool.begin().await?;
        ensure_at_start(&mut tx, player).await?;
        let state = if shared { "recruiting" } else { "active" };
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO expedition_runs (route_id, content_version, guild_id, host_discord_user_id, shared, state, current_checkpoint_index, started_at)
             VALUES ($1, $2, $3, $4, $5, $6, 0, CASE WHEN $5 THEN NULL ELSE now() END)
             RETURNING expedition_run_id",
        ).bind(ROUTE_ID).bind(ROUTE_VERSION).bind(guild).bind(player).bind(shared).bind(state)
         .fetch_one(&mut *tx).await?;
        sqlx::query("INSERT INTO expedition_participants (expedition_run_id, discord_user_id) VALUES ($1, $2)")
            .bind(id).bind(player).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn join(&self, player: &str, guild: &str, id: i64) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let (saved_guild, state, shared) = sqlx::query_as::<_, (Option<String>, String, bool)>(
            "SELECT guild_id, state, shared FROM expedition_runs WHERE expedition_run_id = $1 FOR UPDATE",
        ).bind(id).fetch_optional(&mut *tx).await?.ok_or(ExpeditionError::RunNotFound)?;
        if !shared || saved_guild.as_deref() != Some(guild) { return Err(ExpeditionError::WrongGuild); }
        if state != "recruiting" { return Err(ExpeditionError::NotRecruiting); }
        ensure_at_start(&mut tx, player).await?;
        let count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM expedition_participants WHERE expedition_run_id = $1 AND is_active",
        ).bind(id).fetch_one(&mut *tx).await?;
        if count >= MAX_PARTICIPANTS { return Err(ExpeditionError::PartyFull); }
        sqlx::query("INSERT INTO expedition_participants (expedition_run_id, discord_user_id) VALUES ($1, $2)")
            .bind(id).bind(player).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, Some(guild), id).await
    }

    pub async fn begin(&self, player: &str, guild: Option<&str>, id: i64) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let row = lock_run(&mut tx, id).await?;
        scope(&row.guild_id, guild)?;
        if row.host.as_deref() != Some(player) { return Err(ExpeditionError::NotHost); }
        if row.state != "recruiting" { return Err(ExpeditionError::NotRecruiting); }
        let members = active_members(&mut tx, id).await?;
        for member in &members { ensure_at_start(&mut tx, member).await?; }
        sqlx::query("UPDATE expedition_runs SET state = 'active', started_at = now(), row_version = row_version + 1, updated_at = now() WHERE expedition_run_id = $1")
            .bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn snapshot(&self, player: &str, guild: Option<&str>, id: i64) -> Result<Snapshot, ExpeditionError> {
        let row = load_run(&self.pool, id).await?;
        if row.route_id != ROUTE_ID || row.content_version != ROUTE_VERSION {
            return Err(ExpeditionError::UnknownRoute);
        }
        scope(&row.guild_id, guild)?;
        let member: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM expedition_participants WHERE expedition_run_id = $1 AND discord_user_id = $2)",
        ).bind(id).bind(player).fetch_one(&self.pool).await?;
        if !member { return Err(ExpeditionError::NotParticipant); }
        let index = usize::try_from(row.checkpoint_index).map_err(|_| ExpeditionError::UnknownRoute)?;
        let current = *LANTERN_ROAD.get(index).ok_or(ExpeditionError::UnknownRoute)?;
        let next = LANTERN_ROAD.get(index + 1).copied();
        let participants = sqlx::query_as::<_, (String, bool)>(
            "SELECT discord_user_id, is_active FROM expedition_participants WHERE expedition_run_id = $1 ORDER BY joined_at ASC, discord_user_id ASC",
        ).bind(id).fetch_all(&self.pool).await?
         .into_iter().map(|(discord_user_id, active)| Participant {
             is_host: row.host.as_deref() == Some(discord_user_id.as_str()),
             discord_user_id, active,
         }).collect();
        Ok(Snapshot {
            run_id: id, content_version: row.content_version, guild_id: row.guild_id,
            shared: row.shared, host: row.host, state: row.state,
            checkpoint_index: row.checkpoint_index, row_version: row.row_version,
            current, next, participants,
        })
    }

    pub async fn advance(&self, player: &str, guild: Option<&str>, id: i64, expected: &str, choice: &str) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let row = lock_run(&mut tx, id).await?;
        scope(&row.guild_id, guild)?;
        if row.state != "active" { return Err(ExpeditionError::NotActive); }
        let members = active_members(&mut tx, id).await?;
        if !members.iter().any(|m| m == player) { return Err(ExpeditionError::NotActiveParticipant); }
        let next_index = usize::try_from(row.checkpoint_index).map_err(|_| ExpeditionError::UnknownRoute)? + 1;
        let next = *LANTERN_ROAD.get(next_index).ok_or(ExpeditionError::NotActive)?;
        if next.id != expected { return Err(ExpeditionError::StaleCheckpoint); }
        validate_choice(next_index, choice)?;
        let previous = LANTERN_ROAD.get(next_index - 1).ok_or(ExpeditionError::UnknownRoute)?;
        for member in &members {
            atlas::apply_expedition_travel(&mut tx, member, previous.location_id, next.location_id).await?;
        }
        let checkpoint_index = i16::try_from(next_index).map_err(|_| ExpeditionError::UnknownRoute)?;
        sqlx::query(
            "INSERT INTO expedition_checkpoint_history (expedition_run_id, content_version, checkpoint_index, checkpoint_id, choice_id, completed_by)
             VALUES ($1, $2, $3, $4, $5, $6)",
        ).bind(id).bind(ROUTE_VERSION).bind(checkpoint_index).bind(next.id)
         .bind(choice.trim().to_ascii_lowercase()).bind(player).execute(&mut *tx).await?;
        sqlx::query("UPDATE expedition_participants SET last_checkpoint_index = $2 WHERE expedition_run_id = $1 AND is_active")
            .bind(id).bind(checkpoint_index).execute(&mut *tx).await?;
        let completed = next_index == LANTERN_ROAD.len() - 1;
        sqlx::query(
            "UPDATE expedition_runs SET current_checkpoint_index = $2,
             state = CASE WHEN $3 THEN 'completed' ELSE 'active' END,
             completed_at = CASE WHEN $3 THEN now() ELSE completed_at END,
             row_version = row_version + 1, updated_at = now()
             WHERE expedition_run_id = $1",
        ).bind(id).bind(checkpoint_index).bind(completed).execute(&mut *tx).await?;
        if completed {
            sqlx::query("UPDATE expedition_participants SET is_active = FALSE, left_at = now() WHERE expedition_run_id = $1 AND is_active")
                .bind(id).execute(&mut *tx).await?;
            for member in &members { crate::wayfarer::record_expedition_completion(&mut tx, member).await?; }
        }
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn pause(&self, player: &str, guild: Option<&str>, id: i64) -> Result<Snapshot, ExpeditionError> {
        self.change_state(player, guild, id, "active", "paused").await
    }
    pub async fn resume(&self, player: &str, guild: Option<&str>, id: i64) -> Result<Snapshot, ExpeditionError> {
        self.change_state(player, guild, id, "paused", "active").await
    }
    async fn change_state(&self, player: &str, guild: Option<&str>, id: i64, expected: &str, next: &str) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let row = lock_run(&mut tx, id).await?;
        scope(&row.guild_id, guild)?;
        if !is_active_member(&mut tx, id, player).await? { return Err(ExpeditionError::NotActiveParticipant); }
        if row.state != expected {
            return Err(if expected == "active" { ExpeditionError::NotActive } else { ExpeditionError::NotPaused });
        }
        sqlx::query("UPDATE expedition_runs SET state = $2, row_version = row_version + 1, updated_at = now() WHERE expedition_run_id = $1")
            .bind(id).bind(next).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn leave(&self, player: &str, guild: Option<&str>, id: i64) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let row = lock_run(&mut tx, id).await?;
        scope(&row.guild_id, guild)?;
        if matches!(row.state.as_str(), "completed" | "cancelled") { return Err(ExpeditionError::NotActive); }
        if !is_active_member(&mut tx, id, player).await? { return Err(ExpeditionError::NotActiveParticipant); }
        sqlx::query("UPDATE expedition_participants SET is_active = FALSE, left_at = now() WHERE expedition_run_id = $1 AND discord_user_id = $2")
            .bind(id).bind(player).execute(&mut *tx).await?;
        let next_host: Option<String> = sqlx::query_scalar(
            "SELECT discord_user_id FROM expedition_participants WHERE expedition_run_id = $1 AND is_active ORDER BY joined_at ASC, discord_user_id ASC LIMIT 1",
        ).bind(id).fetch_optional(&mut *tx).await?;
        let empty = next_host.is_none();
        let host = if row.host.as_deref() == Some(player) { next_host.as_deref() } else { row.host.as_deref() };
        sqlx::query(
            "UPDATE expedition_runs SET host_discord_user_id = $2,
             state = CASE WHEN $3 THEN 'cancelled' ELSE state END,
             cancelled_at = CASE WHEN $3 THEN COALESCE(cancelled_at, now()) ELSE cancelled_at END,
             row_version = row_version + 1, updated_at = now() WHERE expedition_run_id = $1",
        ).bind(id).bind(host).bind(empty).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }

    pub async fn cancel(&self, player: &str, guild: Option<&str>, id: i64) -> Result<Snapshot, ExpeditionError> {
        let mut tx = self.pool.begin().await?;
        let row = lock_run(&mut tx, id).await?;
        scope(&row.guild_id, guild)?;
        if row.host.as_deref() != Some(player) { return Err(ExpeditionError::NotHost); }
        if matches!(row.state.as_str(), "completed" | "cancelled") { return Err(ExpeditionError::NotActive); }
        sqlx::query("UPDATE expedition_participants SET is_active = FALSE, left_at = COALESCE(left_at, now()) WHERE expedition_run_id = $1 AND is_active")
            .bind(id).execute(&mut *tx).await?;
        sqlx::query("UPDATE expedition_runs SET state = 'cancelled', cancelled_at = now(), row_version = row_version + 1, updated_at = now() WHERE expedition_run_id = $1")
            .bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
        self.snapshot(player, guild, id).await
    }
}

struct RunRow {
    route_id: String, content_version: i16, guild_id: Option<String>, host: Option<String>,
    shared: bool, state: String, checkpoint_index: i16, row_version: i64,
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
    let values = sqlx::query_as::<_, (String, i16, Option<String>, Option<String>, bool, String, i16, i64)>(sql)
        .bind(id).fetch_optional(pool).await?.ok_or(ExpeditionError::RunNotFound)?;
    Ok(to_run_row(values))
}
fn to_run_row(v: (String, i16, Option<String>, Option<String>, bool, String, i16, i64)) -> RunRow {
    RunRow { route_id:v.0, content_version:v.1, guild_id:v.2, host:v.3, shared:v.4, state:v.5, checkpoint_index:v.6, row_version:v.7 }
}
async fn active_members(tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<Vec<String>, ExpeditionError> {
    Ok(sqlx::query_scalar("SELECT discord_user_id FROM expedition_participants WHERE expedition_run_id = $1 AND is_active ORDER BY joined_at ASC, discord_user_id ASC FOR UPDATE")
        .bind(id).fetch_all(&mut **tx).await?)
}
async fn is_active_member(tx: &mut Transaction<'_, Postgres>, id: i64, player: &str) -> Result<bool, ExpeditionError> {
    Ok(sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM expedition_participants WHERE expedition_run_id = $1 AND discord_user_id = $2 AND is_active)")
        .bind(id).bind(player).fetch_one(&mut **tx).await?)
}
async fn ensure_at_start(tx: &mut Transaction<'_, Postgres>, player: &str) -> Result<(), ExpeditionError> {
    let current: Option<String> = sqlx::query_scalar("SELECT current_location_id FROM player_world_state WHERE discord_user_id = $1 FOR UPDATE")
        .bind(player).fetch_optional(&mut **tx).await?;
    if current.as_deref() != Some("lantern-square") { return Err(ExpeditionError::NotAtStart); }
    Ok(())
}
fn scope(saved: &Option<String>, requested: Option<&str>) -> Result<(), ExpeditionError> {
    if saved.as_deref() == requested { Ok(()) } else { Err(ExpeditionError::WrongGuild) }
}
fn validate_choice(index: usize, choice: &str) -> Result<(), ExpeditionError> {
    let checkpoint = LANTERN_ROAD.get(index).ok_or(ExpeditionError::UnknownRoute)?;
    let choice = choice.trim().to_ascii_lowercase();
    if !checkpoint.choices.contains(&choice.as_str()) { return Err(ExpeditionError::InvalidChoice); }
    if checkpoint.id == "ember-archive" && choice != "star" { return Err(ExpeditionError::IncorrectAnswer); }
    Ok(())
}

pub fn render_catalogue(prefix: &str) -> String {
    let p = safe_prefix(prefix);
    format!("**Expedition Routes**\n\n**The Lantern Road** — Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory\nFour checkpoints · Solo or up to four players in one server · No deadline or attendance penalty\n\nStart solo: {p}expedition start lantern-road solo\nCreate a group: {p}expedition start lantern-road group")
}
pub fn render(snapshot: &Snapshot, prefix: &str) -> String {
    let p = safe_prefix(prefix);
    let participants = snapshot.participants.iter().map(|member| {
        let active = if member.active { "active" } else { "left / finished" };
        let host = if member.is_host { " · host" } else { "" };
        format!("• <@{}>{host} · {active}", member.discord_user_id)
    }).collect::<Vec<_>>().join("\n");
    let next = snapshot.next.map_or_else(|| "The final checkpoint is recorded.".to_owned(), |checkpoint| {
        format!("**Next — {}**\n{}\nChoices: {}", checkpoint.title, checkpoint.prompt, checkpoint.choices.join(" · "))
    });
    let actions = match snapshot.state.as_str() {
        "recruiting" => format!("{p}expedition begin {}\n{p}expedition join {}", snapshot.run_id, snapshot.run_id),
        "active" if snapshot.next.is_some() => format!("{p}expedition advance {} {} <choice>\n{p}expedition pause {}\n{p}expedition leave {}", snapshot.run_id, snapshot.next.map(|c| c.id).unwrap_or(""), snapshot.run_id, snapshot.run_id),
        "paused" => format!("{p}expedition resume {}\n{p}expedition leave {}", snapshot.run_id, snapshot.run_id),
        "completed" => "The Lantern Road is complete. Your Atlas and private Chronicle remain yours.".to_owned(),
        "cancelled" => "This run was cancelled. Completed discoveries remain.".to_owned(),
        _ => "No action is available.".to_owned(),
    };
    let place = LOCATION_CATALOGUE.iter().find(|p| p.id == snapshot.current.location_id).map(|p| p.name).unwrap_or("Unknown");
    format!("**The Lantern Road · Expedition #{}**\n\nState: **{}** · Checkpoint: **{}/{}** · Route v{}\nCurrent place: **{}**\nParticipants:\n{}\n\n{}\n\n{}", snapshot.run_id, snapshot.state, snapshot.checkpoint_index + 1, LANTERN_ROAD.len(), snapshot.content_version, place, participants, next, actions)
}
fn safe_prefix(prefix: &str) -> String {
    prefix.replace('@', "@\u{200B}").replace(char::from(96), "ˋ").replace('\\', "＼")
}

#[cfg(test)]
mod tests {
    use super::{LANTERN_ROAD, ROUTE_ID, ExpeditionError, validate_choice};
    use crate::atlas::LOCATION_CATALOGUE;

    #[test]
    fn lantern_road_uses_existing_reciprocal_atlas_edges() {
        assert_eq!(ROUTE_ID, "lantern-road");
        for pair in LANTERN_ROAD.windows(2) {
            let from = LOCATION_CATALOGUE.iter().find(|p| p.id == pair[0].location_id).unwrap();
            let to = LOCATION_CATALOGUE.iter().find(|p| p.id == pair[1].location_id).unwrap();
            assert!(from.routes_to.contains(&pair[1].location_id));
            assert!(to.routes_to.contains(&pair[0].location_id));
        }
    }

    #[test]
    fn archive_clue_requires_star() {
        assert!(validate_choice(2, "star").is_ok());
        assert!(matches!(validate_choice(2, "lantern"), Err(ExpeditionError::IncorrectAnswer)));
        assert!(matches!(validate_choice(1, "star"), Err(ExpeditionError::InvalidChoice)));
    }
}
