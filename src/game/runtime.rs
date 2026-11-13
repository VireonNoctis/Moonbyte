use std::fmt;

use sqlx::{PgPool, Postgres, Transaction};

use super::{
    engine::{GameAction, GameError},
    model::{GameKind, GameSession, SessionSnapshot},
};

/// Database-backed session operations. Each mutation is committed atomically
/// before the Discord adapter sends a response.
#[derive(Clone)]
pub struct GameRuntime {
    pool: PgPool,
}

#[derive(Debug)]
pub enum GameRuntimeError {
    Domain(GameError),
    Database(sqlx::Error),
    Serialization(serde_json::Error),
    UnsupportedStateVersion(i16),
}

impl fmt::Display for GameRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domain(error) => fmt::Display::fmt(error, formatter),
            Self::Database(_) => formatter.write_str("The game store is temporarily unavailable."),
            Self::Serialization(_) => {
                formatter.write_str("The stored game state could not be read.")
            }
            Self::UnsupportedStateVersion(version) => {
                write!(
                    formatter,
                    "Stored game state version {version} is not supported."
                )
            }
        }
    }
}

impl std::error::Error for GameRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Domain(error) => Some(error),
            Self::Database(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::UnsupportedStateVersion(_) => None,
        }
    }
}

impl From<GameError> for GameRuntimeError {
    fn from(error: GameError) -> Self {
        Self::Domain(error)
    }
}
impl From<sqlx::Error> for GameRuntimeError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
impl From<serde_json::Error> for GameRuntimeError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}

impl GameRuntime {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn start(
        &self,
        game: GameKind,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<SessionSnapshot, GameRuntimeError> {
        if game.requires_opponent() && guild_id.is_none() {
            return Err(GameError::WrongGuild.into());
        }

        let mut transaction = self.pool.begin().await?;
        lock_player_for_session_capacity(&mut transaction, player_id).await?;
        ensure_active_session_capacity(&mut transaction, player_id, guild_id).await?;
        let sequence: i64 = sqlx::query_scalar("SELECT nextval('moonbyte_session_id_seq')")
            .fetch_one(&mut *transaction)
            .await?;
        let id = format!("MB-{sequence:06}");
        let session =
            GameSession::create(id, game, player_id.to_owned(), guild_id.map(str::to_owned));
        let snapshot = session.snapshot();
        let state_json = serde_json::to_string(&session)?;

        sqlx::query(
            "INSERT INTO game_sessions              (session_id, game_id, guild_id, phase, state_version, action_count, session_state, expires_at)              VALUES ($1, $2, $3, $4, 1, 0, $5::jsonb, now() + interval '24 hours')",
        )
        .bind(&snapshot.id)
        .bind(snapshot.game.definition().id)
        .bind(snapshot.guild_id.as_deref())
        .bind(snapshot.phase.to_string())
        .bind(state_json)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;
        Ok(snapshot)
    }

    pub async fn join(
        &self,
        session_id: &str,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<SessionSnapshot, GameRuntimeError> {
        let mut transaction = self.pool.begin().await?;
        lock_player_for_session_capacity(&mut transaction, player_id).await?;
        let mut session = load_for_update(&mut transaction, session_id).await?;
        session.join(player_id, guild_id)?;
        ensure_active_session_capacity(&mut transaction, player_id, guild_id).await?;
        persist_locked(&mut transaction, &session).await?;
        transaction.commit().await?;
        Ok(session.snapshot())
    }

    pub async fn act(
        &self,
        session_id: &str,
        player_id: &str,
        guild_id: Option<&str>,
        input: &str,
    ) -> Result<SessionSnapshot, GameRuntimeError> {
        let mut transaction = self.pool.begin().await?;
        let mut session = load_for_update(&mut transaction, session_id).await?;
        if session.guild_id() != guild_id {
            return Err(GameError::WrongGuild.into());
        }
        let action = GameAction::parse(session.game(), input)?;
        session.act(player_id, action)?;
        persist_locked(&mut transaction, &session).await?;
        transaction.commit().await?;
        Ok(session.snapshot())
    }

    pub async fn get(
        &self,
        session_id: &str,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<SessionSnapshot, GameRuntimeError> {
        let stored: Option<(i16, String)> = sqlx::query_as(
            "SELECT state_version, session_state::text FROM game_sessions \
             WHERE session_id = $1 AND expires_at > now()",
        )
        .bind(normalise_session_id(session_id))
        .fetch_optional(&self.pool)
        .await?;

        let (version, state_json) = stored.ok_or(GameError::NotJoinable)?;
        let session = decode_session(version, &state_json)?;
        if !session.is_visible_to(player_id, guild_id) {
            return Err(GameError::NotParticipant.into());
        }
        Ok(session.snapshot())
    }

    /// List the requester's non-expired sessions in the current guild or DM scope.
    /// The JSONB participant predicate is repeated by a domain-level visibility check.
    pub async fn list_active_for_player(
        &self,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<Vec<SessionSnapshot>, GameRuntimeError> {
        let stored: Vec<(i16, String)> = sqlx::query_as(
            "SELECT state_version, session_state::text FROM game_sessions \
             WHERE expires_at > now() \
               AND phase IN ('waiting for players', 'active') \
               AND guild_id IS NOT DISTINCT FROM $2 \
               AND (session_state -> 'players') @> jsonb_build_array($1::text) \
             ORDER BY updated_at DESC, session_id ASC \
             LIMIT 10",
        )
        .bind(player_id)
        .bind(guild_id)
        .fetch_all(&self.pool)
        .await?;

        let mut snapshots = Vec::with_capacity(stored.len());
        for (version, state_json) in stored {
            let session = decode_session(version, &state_json)?;
            if session.is_visible_to(player_id, guild_id) {
                snapshots.push(session.snapshot());
            }
        }
        Ok(snapshots)
    }

    /// List the requester's non-expired completed or ended sessions in the current scope.
    /// The JSONB participant predicate is repeated by a domain-level visibility check.
    pub async fn list_recent_finished_for_player(
        &self,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<Vec<SessionSnapshot>, GameRuntimeError> {
        let stored: Vec<(i16, String)> = sqlx::query_as(
            "SELECT state_version, session_state::text FROM game_sessions \
             WHERE expires_at > now() \
               AND phase IN ('completed', 'ended') \
               AND guild_id IS NOT DISTINCT FROM $2 \
               AND (session_state -> 'players') @> jsonb_build_array($1::text) \
             ORDER BY updated_at DESC, session_id ASC \
             LIMIT 10",
        )
        .bind(player_id)
        .bind(guild_id)
        .fetch_all(&self.pool)
        .await?;

        let mut snapshots = Vec::with_capacity(stored.len());
        for (version, state_json) in stored {
            let session = decode_session(version, &state_json)?;
            if session.is_visible_to(player_id, guild_id) {
                snapshots.push(session.snapshot());
            }
        }
        Ok(snapshots)
    }

    pub async fn leave(
        &self,
        session_id: &str,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<SessionSnapshot, GameRuntimeError> {
        let mut transaction = self.pool.begin().await?;
        let mut session = load_for_update(&mut transaction, session_id).await?;
        if session.guild_id() != guild_id {
            return Err(GameError::WrongGuild.into());
        }
        session.leave(player_id)?;
        persist_locked(&mut transaction, &session).await?;
        transaction.commit().await?;
        Ok(session.snapshot())
    }
}

const MAX_ACTIVE_SESSIONS_PER_PLAYER: i64 = 10;

async fn lock_player_for_session_capacity(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('moonbyte:user:' || $1, 0))")
        .bind(player_id)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

async fn ensure_active_session_capacity(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    guild_id: Option<&str>,
) -> Result<(), GameRuntimeError> {
    let active_sessions: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM game_sessions \
         WHERE expires_at > now() \
           AND phase IN ('waiting for players', 'active') \
           AND guild_id IS NOT DISTINCT FROM $2 \
           AND (session_state -> 'players') @> jsonb_build_array($1::text)",
    )
    .bind(player_id)
    .bind(guild_id)
    .fetch_one(&mut **transaction)
    .await?;

    if active_sessions >= MAX_ACTIVE_SESSIONS_PER_PLAYER {
        return Err(GameError::SessionLimitReached.into());
    }
    Ok(())
}

async fn load_for_update(
    transaction: &mut Transaction<'_, Postgres>,
    session_id: &str,
) -> Result<GameSession, GameRuntimeError> {
    let stored: Option<(i16, String)> = sqlx::query_as(
        "SELECT state_version, session_state::text FROM game_sessions \
         WHERE session_id = $1 AND expires_at > now() FOR UPDATE",
    )
    .bind(normalise_session_id(session_id))
    .fetch_optional(&mut **transaction)
    .await?;

    let (version, state_json) = stored.ok_or(GameError::NotJoinable)?;
    decode_session(version, &state_json)
}

async fn persist_locked(
    transaction: &mut Transaction<'_, Postgres>,
    session: &GameSession,
) -> Result<(), GameRuntimeError> {
    let snapshot = session.snapshot();
    let state_json = serde_json::to_string(session)?;

    let result = sqlx::query(
        "UPDATE game_sessions \
         SET game_id = $2, guild_id = $3, phase = $4, action_count = $5, \
             session_state = $6::jsonb, updated_at = now(), \
             expires_at = now() + interval '24 hours' \
         WHERE session_id = $1 AND state_version = 1",
    )
    .bind(session.id())
    .bind(snapshot.game.definition().id)
    .bind(snapshot.guild_id.as_deref())
    .bind(snapshot.phase.to_string())
    .bind(i64::from(snapshot.action_count))
    .bind(state_json)
    .execute(&mut **transaction)
    .await?;

    if result.rows_affected() != 1 {
        return Err(GameError::NotJoinable.into());
    }
    Ok(())
}

fn decode_session(version: i16, state_json: &str) -> Result<GameSession, GameRuntimeError> {
    if version != 1 {
        return Err(GameRuntimeError::UnsupportedStateVersion(version));
    }
    Ok(serde_json::from_str(state_json)?)
}

fn normalise_session_id(id: &str) -> String {
    id.trim().to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    async fn runtime() -> GameRuntime {
        let database_url = std::env::var("DATABASE_URL")
            .expect("PostgreSQL integration tests require DATABASE_URL");
        let database = Database::connect(&database_url)
            .await
            .expect("PostgreSQL must be available and migrations must apply");
        GameRuntime::new(database.pool())
    }

    #[tokio::test]
    async fn creates_distinct_durable_sessions_and_canonical_ids() {
        let runtime = runtime().await;
        let first = runtime
            .start(GameKind::MemoryMatch, "player-1", Some("guild-1"))
            .await
            .expect("start");
        let second = runtime
            .start(GameKind::WordScramble, "player-1", Some("guild-1"))
            .await
            .expect("start");

        assert_ne!(first.id, second.id);
        assert!(first.id.starts_with("MB-"));
        assert_eq!(first.phase.to_string(), "active");
    }

    #[tokio::test]
    async fn multiplayer_lobbies_are_guild_scoped_and_reject_self_join() {
        let runtime = runtime().await;
        let lobby = runtime
            .start(GameKind::TicTacToe, "owner", Some("guild-1"))
            .await
            .expect("lobby");
        assert_eq!(lobby.phase.to_string(), "waiting for players");
        assert!(matches!(
            runtime.join(&lobby.id, "owner", Some("guild-1")).await,
            Err(GameRuntimeError::Domain(GameError::AlreadyJoined))
        ));
        assert!(matches!(
            runtime.join(&lobby.id, "guest", Some("guild-2")).await,
            Err(GameRuntimeError::Domain(GameError::WrongGuild))
        ));
        let started = runtime
            .join(&lobby.id.to_ascii_lowercase(), "guest", Some("guild-1"))
            .await
            .expect("join");
        assert_eq!(started.phase.to_string(), "active");
        assert_eq!(started.players.len(), 2);
    }

    #[tokio::test]
    async fn multiplayer_sessions_cannot_be_created_in_direct_messages() {
        let runtime = runtime().await;
        assert!(matches!(
            runtime.start(GameKind::ConnectFour, "owner", None).await,
            Err(GameRuntimeError::Domain(GameError::WrongGuild))
        ));
    }

    #[tokio::test]
    async fn private_session_reads_require_participant_membership() {
        let runtime = runtime().await;
        let session = runtime
            .start(GameKind::MemoryMatch, "owner", None)
            .await
            .expect("start");
        assert!(runtime.get(&session.id, "owner", None).await.is_ok());
        assert!(matches!(
            runtime.get(&session.id, "intruder", None).await,
            Err(GameRuntimeError::Domain(GameError::NotParticipant))
        ));
        assert!(matches!(
            runtime.get(&session.id, "owner", Some("guild-1")).await,
            Err(GameRuntimeError::Domain(GameError::NotParticipant))
        ));
    }

    #[tokio::test]
    async fn active_session_listing_is_participant_and_guild_scoped() {
        let runtime = runtime().await;
        let guild_session = runtime
            .start(GameKind::TicTacToe, "listing-owner", Some("listing-guild"))
            .await
            .expect("guild lobby");
        runtime
            .start(
                GameKind::WordScramble,
                "listing-other",
                Some("listing-guild"),
            )
            .await
            .expect("another player's session");
        runtime
            .start(
                GameKind::WordScramble,
                "listing-owner",
                Some("another-guild"),
            )
            .await
            .expect("session in another guild");
        let dm_session = runtime
            .start(GameKind::MemoryMatch, "listing-owner", None)
            .await
            .expect("DM session");
        let completed = runtime
            .start(GameKind::WordScramble, "listing-owner", None)
            .await
            .expect("completed session");
        runtime
            .act(&completed.id, "listing-owner", None, "orbit")
            .await
            .expect("complete word scramble");
        let abandoned = runtime
            .start(GameKind::MemoryMatch, "listing-owner", None)
            .await
            .expect("abandoned session");
        runtime
            .leave(&abandoned.id, "listing-owner", None)
            .await
            .expect("end session");

        let guild_sessions = runtime
            .list_active_for_player("listing-owner", Some("listing-guild"))
            .await
            .expect("list guild sessions");
        assert_eq!(guild_sessions.len(), 1);
        assert_eq!(guild_sessions[0].id, guild_session.id);

        let dm_sessions = runtime
            .list_active_for_player("listing-owner", None)
            .await
            .expect("list DM sessions");
        assert_eq!(dm_sessions.len(), 1);
        assert_eq!(dm_sessions[0].id, dm_session.id);
    }

    #[tokio::test]
    async fn recent_finished_sessions_are_participant_and_scope_filtered() {
        let runtime = runtime().await;
        let player = "history-owner";
        let guild = "history-guild";

        let completed = runtime
            .start(GameKind::WordScramble, player, Some(guild))
            .await
            .expect("completed guild session");
        runtime
            .act(&completed.id, player, Some(guild), "orbit")
            .await
            .expect("complete Word Scramble");

        let ended = runtime
            .start(GameKind::MemoryMatch, player, Some(guild))
            .await
            .expect("ended guild session");
        runtime
            .leave(&ended.id, player, Some(guild))
            .await
            .expect("end Memory Match");

        let active = runtime
            .start(GameKind::MemoryMatch, player, Some(guild))
            .await
            .expect("active guild session");
        let other_player = runtime
            .start(GameKind::WordScramble, "history-other", Some(guild))
            .await
            .expect("another player's completed session");
        runtime
            .act(&other_player.id, "history-other", Some(guild), "orbit")
            .await
            .expect("complete another player's session");

        let other_guild = runtime
            .start(GameKind::WordScramble, player, Some("other-history-guild"))
            .await
            .expect("other-guild session");
        runtime
            .act(
                &other_guild.id,
                player,
                Some("other-history-guild"),
                "orbit",
            )
            .await
            .expect("complete other-guild session");

        let dm = runtime
            .start(GameKind::WordScramble, player, None)
            .await
            .expect("DM session");
        runtime
            .act(&dm.id, player, None, "orbit")
            .await
            .expect("complete DM session");

        let guild_history = runtime
            .list_recent_finished_for_player(player, Some(guild))
            .await
            .expect("list guild history");
        assert_eq!(guild_history.len(), 2);
        assert!(guild_history.iter().any(|session| {
            session.id == completed.id && session.phase.to_string() == "completed"
        }));
        assert!(
            guild_history
                .iter()
                .any(|session| { session.id == ended.id && session.phase.to_string() == "ended" })
        );
        assert!(!guild_history.iter().any(|session| {
            session.id == active.id || session.id == other_guild.id || session.id == dm.id
        }));

        let dm_history = runtime
            .list_recent_finished_for_player(player, None)
            .await
            .expect("list DM history");
        assert_eq!(dm_history.len(), 1);
        assert_eq!(dm_history[0].id, dm.id);
    }

    #[tokio::test]
    async fn active_session_capacity_is_scoped_and_released_after_leave() {
        let runtime = runtime().await;
        let player = "session-capacity-scope-player";
        let guild = "session-capacity-scope-guild";
        let mut sessions = Vec::new();

        for _ in 0..10 {
            sessions.push(
                runtime
                    .start(GameKind::MemoryMatch, player, Some(guild))
                    .await
                    .expect("start within active-session capacity"),
            );
        }

        assert!(matches!(
            runtime
                .start(GameKind::MemoryMatch, player, Some(guild))
                .await,
            Err(GameRuntimeError::Domain(GameError::SessionLimitReached))
        ));

        runtime
            .leave(&sessions[0].id, player, Some(guild))
            .await
            .expect("leaving a session releases one slot");
        runtime
            .start(GameKind::MemoryMatch, player, Some(guild))
            .await
            .expect("start after a slot is released");
        runtime
            .start(GameKind::MemoryMatch, player, Some("another-guild"))
            .await
            .expect("capacity is independent per guild");
        runtime
            .start(GameKind::MemoryMatch, player, None)
            .await
            .expect("DM sessions have their own capacity scope");
    }

    #[tokio::test]
    async fn joining_respects_the_joiners_active_session_capacity() {
        let runtime = runtime().await;
        let guest = "session-capacity-join-guest";
        let guild = "session-capacity-join-guild";

        for _ in 0..10 {
            runtime
                .start(GameKind::MemoryMatch, guest, Some(guild))
                .await
                .expect("fill guest active-session capacity");
        }

        let lobby = runtime
            .start(
                GameKind::TicTacToe,
                "session-capacity-lobby-owner",
                Some(guild),
            )
            .await
            .expect("create waiting lobby");
        assert!(matches!(
            runtime.join(&lobby.id, guest, Some(guild)).await,
            Err(GameRuntimeError::Domain(GameError::SessionLimitReached))
        ));

        let unchanged = runtime
            .get(&lobby.id, "session-capacity-lobby-owner", Some(guild))
            .await
            .expect("read lobby after rejected join");
        assert_eq!(unchanged.phase.to_string(), "waiting for players");
        assert_eq!(unchanged.players.len(), 1);
    }

    #[tokio::test]
    async fn concurrent_starts_cannot_exceed_active_session_capacity() {
        let runtime = runtime().await;
        let mut starts = Vec::new();

        for _ in 0..12 {
            let runtime = runtime.clone();
            starts.push(tokio::spawn(async move {
                runtime
                    .start(
                        GameKind::MemoryMatch,
                        "session-capacity-concurrent-player",
                        Some("session-capacity-concurrent-guild"),
                    )
                    .await
            }));
        }

        let mut started = 0;
        let mut limited = 0;
        for task in starts {
            match task.await.expect("join session-start task") {
                Ok(_) => started += 1,
                Err(GameRuntimeError::Domain(GameError::SessionLimitReached)) => limited += 1,
                Err(error) => panic!("unexpected session-start error: {error}"),
            }
        }

        assert_eq!(started, 10);
        assert_eq!(limited, 2);
        let active = runtime
            .list_active_for_player(
                "session-capacity-concurrent-player",
                Some("session-capacity-concurrent-guild"),
            )
            .await
            .expect("list sessions after concurrent starts");
        assert_eq!(active.len(), 10);
    }

    #[tokio::test]
    async fn active_session_listing_is_limited_to_ten_results() {
        let runtime = runtime().await;
        for _ in 0..10 {
            runtime
                .start(
                    GameKind::MemoryMatch,
                    "listing-limit",
                    Some("listing-limit-guild"),
                )
                .await
                .expect("create session");
        }

        let sessions = runtime
            .list_active_for_player("listing-limit", Some("listing-limit-guild"))
            .await
            .expect("list sessions");
        assert_eq!(sessions.len(), 10);
    }

    #[tokio::test]
    async fn actions_are_scoped_to_participants_and_guilds() {
        let runtime = runtime().await;
        let lobby = runtime
            .start(GameKind::TicTacToe, "owner", Some("guild-1"))
            .await
            .expect("lobby");
        runtime
            .join(&lobby.id, "guest", Some("guild-1"))
            .await
            .expect("join");
        assert!(matches!(
            runtime
                .act(&lobby.id, "intruder", Some("guild-1"), "1,1")
                .await,
            Err(GameRuntimeError::Domain(
                GameError::NotYourTurn | GameError::NotParticipant
            ))
        ));
        assert!(matches!(
            runtime
                .act(&lobby.id, "owner", Some("guild-2"), "1,1")
                .await,
            Err(GameRuntimeError::Domain(GameError::WrongGuild))
        ));
        assert!(
            runtime
                .act(&lobby.id, "owner", Some("guild-1"), "1,1")
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn rejected_actions_do_not_mutate_persisted_state() {
        let runtime = runtime().await;
        let session = runtime
            .start(GameKind::MemoryMatch, "owner", None)
            .await
            .expect("start");
        let before = runtime
            .get(&session.id, "owner", None)
            .await
            .expect("read before");
        assert!(
            runtime
                .act(&session.id, "owner", None, "not-a-position")
                .await
                .is_err()
        );
        let after = runtime
            .get(&session.id, "owner", None)
            .await
            .expect("read after");
        assert_eq!(before, after);
    }

    #[tokio::test]
    async fn session_state_survives_runtime_reconstruction() {
        let runtime = runtime().await;
        let session = runtime
            .start(GameKind::WordScramble, "owner", None)
            .await
            .expect("start");
        let restarted = GameRuntime::new(runtime.pool.clone());

        let recovered = restarted
            .get(&session.id, "owner", None)
            .await
            .expect("recover the saved session from PostgreSQL");
        assert_eq!(recovered.id, session.id);
        assert_eq!(recovered.board, session.board);
    }

    #[tokio::test]
    async fn leaving_and_completion_are_durable_and_reject_further_actions() {
        let runtime = runtime().await;
        let session = runtime
            .start(GameKind::WordScramble, "owner", None)
            .await
            .expect("start");
        let ended = runtime
            .leave(&session.id, "owner", None)
            .await
            .expect("leave");
        assert_eq!(ended.phase.to_string(), "ended");
        assert!(matches!(
            runtime.act(&session.id, "owner", None, "orbit").await,
            Err(GameRuntimeError::Domain(GameError::NotActive))
        ));
    }
    #[tokio::test]
    async fn concurrent_duplicate_moves_are_serialized_by_the_session_row_lock() {
        let runtime = runtime().await;
        let lobby = runtime
            .start(GameKind::TicTacToe, "owner", Some("guild-race"))
            .await
            .expect("lobby");
        runtime
            .join(&lobby.id, "guest", Some("guild-race"))
            .await
            .expect("join");

        let (first, second) = tokio::join!(
            runtime.act(&lobby.id, "owner", Some("guild-race"), "1,1"),
            runtime.act(&lobby.id, "owner", Some("guild-race"), "1,1")
        );
        let successes = first.is_ok() as usize + second.is_ok() as usize;
        assert_eq!(successes, 1);

        let snapshot = runtime
            .get(&lobby.id, "owner", Some("guild-race"))
            .await
            .expect("read final session");
        assert_eq!(snapshot.action_count, 1);
    }

    #[tokio::test]
    async fn expired_sessions_are_removed_by_the_retention_cleanup() {
        let runtime = runtime().await;
        let session = runtime
            .start(GameKind::WordScramble, "owner", None)
            .await
            .expect("start");

        sqlx::query("UPDATE game_sessions SET expires_at = now() - interval '1 second' WHERE session_id = $1")
            .bind(&session.id)
            .execute(&runtime.pool)
            .await
            .expect("expire the test session");

        let database = Database::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL"))
            .await
            .expect("database");
        assert!(database.remove_expired_sessions().await.expect("cleanup") >= 1);
        assert!(matches!(
            runtime.get(&session.id, "owner", None).await,
            Err(GameRuntimeError::Domain(GameError::NotJoinable))
        ));
    }

    #[tokio::test]
    async fn rejects_unknown_persisted_state_versions() {
        let runtime = runtime().await;
        let session = runtime
            .start(GameKind::WordScramble, "owner", None)
            .await
            .expect("start");

        sqlx::query("UPDATE game_sessions SET state_version = 42 WHERE session_id = $1")
            .bind(&session.id)
            .execute(&runtime.pool)
            .await
            .expect("change version for the test");

        assert!(matches!(
            runtime.get(&session.id, "owner", None).await,
            Err(GameRuntimeError::UnsupportedStateVersion(42))
        ));
    }
}
