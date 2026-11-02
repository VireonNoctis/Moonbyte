//! Moonbyte application foundation, game domain and Discord command modules.

pub mod achievement;
pub mod atlas;
pub mod commands;
pub mod config;
pub mod database;
pub mod emoji;
pub mod game;
pub mod profile;
pub mod tavern;
pub mod wallet;

use std::time::{Duration, Instant};

/// Shared error type for the Discord adapter and command handlers.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Data shared with every Poise command invocation.
pub struct Data {
    prefix: String,
    started_at: Instant,
    games: game::GameRuntime,
    profiles: profile::ProfileStore,
    wallets: wallet::WalletStore,
    achievements: achievement::AchievementStore,
    atlas: atlas::AtlasStore,
}

impl Data {
    pub fn new(
        prefix: String,
        games: game::GameRuntime,
        profiles: profile::ProfileStore,
        wallets: wallet::WalletStore,
        achievements: achievement::AchievementStore,
        atlas: atlas::AtlasStore,
    ) -> Self {
        Self {
            prefix,
            started_at: Instant::now(),
            games,
            profiles,
            wallets,
            achievements,
            atlas,
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

    pub fn profiles(&self) -> &profile::ProfileStore {
        &self.profiles
    }

    pub fn wallets(&self) -> &wallet::WalletStore {
        &self.wallets
    }

    pub fn achievements(&self) -> &achievement::AchievementStore {
        &self.achievements
    }

    pub fn atlas(&self) -> &atlas::AtlasStore {
        &self.atlas
    }
}
