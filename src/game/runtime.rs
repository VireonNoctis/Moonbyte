use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use super::{
    engine::{GameAction, GameError},
    model::{GameKind, GameSession, SessionSnapshot},
};

#[derive(Clone, Default)]
pub struct GameRuntime {
    sessions: Arc<Mutex<SessionRegistry>>,
}

#[derive(Default)]
struct SessionRegistry {
    next_id: u64,
    sessions: BTreeMap<String, GameSession>,
}

impl GameRuntime {
    pub fn new() -> Self { Self::default() }

    pub fn start(
        &self,
        game: GameKind,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<SessionSnapshot, GameError> {
        if game.requires_opponent() && guild_id.is_none() {
            return Err(GameError::WrongGuild);
        }
        let mut registry = self.lock()?;
        registry.next_id = registry.next_id.saturating_add(1);
        let id = format!("MB-{:06}", registry.next_id);
        let session = GameSession::create(
            id.clone(),
            game,
            player_id.to_owned(),
            guild_id.map(str::to_owned),
        );
        let snapshot = session.snapshot();
        registry.sessions.insert(id, session);
        Ok(snapshot)
    }

    pub fn join(
        &self,
        session_id: &str,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<SessionSnapshot, GameError> {
        let mut registry = self.lock()?;
        let session = registry.sessions
            .get_mut(&normalise_session_id(session_id))
            .ok_or(GameError::NotJoinable)?;
        session.join(player_id, guild_id)?;
        Ok(session.snapshot())
    }

    pub fn act(
        &self,
        session_id: &str,
        player_id: &str,
        guild_id: Option<&str>,
        input: &str,
    ) -> Result<SessionSnapshot, GameError> {
        let mut registry = self.lock()?;
        let session = registry.sessions
            .get_mut(&normalise_session_id(session_id))
            .ok_or(GameError::NotJoinable)?;
        if session.guild_id() != guild_id { return Err(GameError::WrongGuild); }
        let action = GameAction::parse(session.game(), input)?;
        session.act(player_id, action)?;
        Ok(session.snapshot())
    }

    pub fn get(
        &self,
        session_id: &str,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<SessionSnapshot, GameError> {
        let registry = self.lock()?;
        let session = registry.sessions
            .get(&normalise_session_id(session_id))
            .ok_or(GameError::NotJoinable)?;
        if !session.is_visible_to(player_id, guild_id) { return Err(GameError::NotParticipant); }
        Ok(session.snapshot())
    }

    pub fn leave(
        &self,
        session_id: &str,
        player_id: &str,
        guild_id: Option<&str>,
    ) -> Result<SessionSnapshot, GameError> {
        let mut registry = self.lock()?;
        let session = registry.sessions
            .get_mut(&normalise_session_id(session_id))
            .ok_or(GameError::NotJoinable)?;
        if session.guild_id() != guild_id { return Err(GameError::WrongGuild); }
        session.leave(player_id)?;
        Ok(session.snapshot())
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, SessionRegistry>, GameError> {
        self.sessions.lock().map_err(|_| GameError::RuntimeUnavailable)
    }
}

fn normalise_session_id(id: &str) -> String { id.trim().to_ascii_uppercase() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_distinct_sessions_and_canonical_ids() {
        let runtime = GameRuntime::new();
        let first = runtime.start(GameKind::MemoryMatch, "player-1", Some("guild-1")).expect("start");
        let second = runtime.start(GameKind::WordScramble, "player-1", Some("guild-1")).expect("start");
        assert_eq!(first.id, "MB-000001");
        assert_eq!(second.id, "MB-000002");
        assert_eq!(first.phase.to_string(), "active");
    }

    #[test]
    fn multiplayer_lobbies_are_guild_scoped_and_reject_self_join() {
        let runtime = GameRuntime::new();
        let lobby = runtime.start(GameKind::TicTacToe, "owner", Some("guild-1")).expect("lobby");
        assert_eq!(lobby.phase.to_string(), "waiting for players");
        assert_eq!(runtime.join(&lobby.id, "owner", Some("guild-1")), Err(GameError::AlreadyJoined));
        assert_eq!(runtime.join(&lobby.id, "guest", Some("guild-2")), Err(GameError::WrongGuild));
        let started = runtime.join(&lobby.id.to_ascii_lowercase(), "guest", Some("guild-1")).expect("join");
        assert_eq!(started.phase.to_string(), "active");
        assert_eq!(started.players.len(), 2);
    }

    #[test]
    fn multiplayer_sessions_cannot_be_created_in_direct_messages() {
        let runtime = GameRuntime::new();
        assert_eq!(runtime.start(GameKind::ConnectFour, "owner", None), Err(GameError::WrongGuild));
    }

    #[test]
    fn private_session_reads_require_participant_membership() {
        let runtime = GameRuntime::new();
        let session = runtime.start(GameKind::MemoryMatch, "owner", None).expect("start");
        assert!(runtime.get(&session.id, "owner", None).is_ok());
        assert_eq!(runtime.get(&session.id, "intruder", None), Err(GameError::NotParticipant));
        assert_eq!(runtime.get(&session.id, "owner", Some("guild-1")), Err(GameError::NotParticipant));
    }

    #[test]
    fn actions_are_scoped_to_participants_and_guilds() {
        let runtime = GameRuntime::new();
        let lobby = runtime.start(GameKind::TicTacToe, "owner", Some("guild-1")).expect("lobby");
        runtime.join(&lobby.id, "guest", Some("guild-1")).expect("join");
        assert_eq!(runtime.act(&lobby.id, "intruder", Some("guild-1"), "1,1"), Err(GameError::NotParticipant));
        assert_eq!(runtime.act(&lobby.id, "guest", Some("guild-2"), "1,1"), Err(GameError::WrongGuild));
        assert!(runtime.act(&lobby.id, "owner", Some("guild-1"), "1,1").is_ok());
    }

    #[test]
    fn turns_and_duplicate_moves_are_enforced() {
        let runtime = GameRuntime::new();
        let lobby = runtime.start(GameKind::TicTacToe, "owner", Some("guild-1")).expect("lobby");
        runtime.join(&lobby.id, "guest", Some("guild-1")).expect("join");
        assert_eq!(runtime.act(&lobby.id, "guest", Some("guild-1"), "1,1"), Err(GameError::NotYourTurn));
        runtime.act(&lobby.id, "owner", Some("guild-1"), "1,1").expect("move");
        assert_eq!(runtime.act(&lobby.id, "guest", Some("guild-1"), "1,1"), Err(GameError::OccupiedCell));
    }

    #[test]
    fn leaving_ends_a_session_without_removing_the_snapshot() {
        let runtime = GameRuntime::new();
        let session = runtime.start(GameKind::WordScramble, "owner", None).expect("start");
        let ended = runtime.leave(&session.id, "owner", None).expect("leave");
        assert_eq!(ended.phase.to_string(), "ended");
        assert!(ended.render().contains("ended"));
        assert_eq!(runtime.act(&session.id, "owner", None, "orbit"), Err(GameError::NotActive));
    }

    #[test]
    fn completed_memory_match_rejects_further_actions() {
        let runtime = GameRuntime::new();
        let session = runtime.start(GameKind::MemoryMatch, "owner", None).expect("start");
        for (first, second) in [(1, 5), (2, 6), (3, 7), (4, 8)] {
            runtime.act(&session.id, "owner", None, &first.to_string()).expect("first reveal");
            let result = runtime.act(&session.id, "owner", None, &second.to_string()).expect("pair");
            if second == 8 { assert_eq!(result.phase, super::super::model::GamePhase::Completed); }
        }
        assert_eq!(runtime.act(&session.id, "owner", None, "1"), Err(GameError::NotActive));
    }
}
