//! Moonbyte application foundation, game domain and Discord command modules.

pub mod commands;
pub mod config;
pub mod database;
pub mod game;
pub mod profile;

use std::time::{Duration, Instant};

/// Shared error type for the Discord adapter and command handlers.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Data shared with every Poise command invocation.
pub struct Data {
    prefix: String,
    started_at: Instant,
    games: game::GameRuntime,
    profiles: profile::ProfileRepository,
}

impl Data {
    pub fn new(
        prefix: String,
        games: game::GameRuntime,
        profiles: profile::ProfileRepository,
    ) -> Self {
        Self {
            prefix,
            started_at: Instant::now(),
            games,
            profiles,
        }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn uptime(&self) -> Duration {
        self.started_at.elapsed()
    }

    pub fn games(&self) -> &game::GameRuntime {
        &self.games
    }

    pub fn profiles(&self) -> &profile::ProfileRepository {
        &self.profiles
    }
}
