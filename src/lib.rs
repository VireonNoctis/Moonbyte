//! Moonbyte application foundation, game domain and Discord command modules.

pub mod commands;
pub mod config;
pub mod game;

use std::time::{Duration, Instant};

/// Shared error type for the Discord adapter and command handlers.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Data shared with every Poise command invocation.
pub struct Data {
    prefix: String,
    started_at: Instant,
    games: game::GameRuntime,
}

impl Data {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            started_at: Instant::now(),
            games: game::GameRuntime::new(),
        }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn uptime(&self) -> Duration { self.started_at.elapsed() }

    pub fn games(&self) -> &game::GameRuntime { &self.games }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_data_starts_with_an_available_game_runtime() {
        let data = Data::new(".".to_owned());
        let session = data
            .games()
            .start(game::GameKind::WordScramble, "player", None)
            .expect("the runtime should accept a solo session");
        assert_eq!(session.game, game::GameKind::WordScramble);
        assert_eq!(session.phase, game::GamePhase::Active);
    }
}
