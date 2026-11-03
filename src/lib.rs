//! Moonbyte application foundation, game domain and Discord command modules.

pub mod achievement;
pub mod atlas;
pub mod commands;
pub mod config;
pub mod consent;
pub mod database;
pub mod emoji;
pub mod game;
pub mod guild_settings;
pub mod profile;
pub mod tavern;
pub mod wallet;

use std::time::{Duration, Instant};

/// Shared error type for the Discord adapter and command handlers.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Services shared by the Discord command adapter.
pub struct DataServices {
    pub games: game::GameRuntime,
    pub profiles: profile::ProfileStore,
    pub wallets: wallet::WalletStore,
    pub achievements: achievement::AchievementStore,
    pub atlas: atlas::AtlasStore,
    pub guild_settings: guild_settings::GuildSettingsStore,
    pub consents: consent::ConsentStore,
}

/// Data shared with every Poise command invocation.
pub struct Data {
    prefix: String,
    started_at: Instant,
    games: game::GameRuntime,
    profiles: profile::ProfileStore,
    wallets: wallet::WalletStore,
    achievements: achievement::AchievementStore,
    atlas: atlas::AtlasStore,
    guild_settings: guild_settings::GuildSettingsStore,
    consents: consent::ConsentStore,
}

impl Data {
    pub fn new(prefix: String, services: DataServices) -> Self {
        let DataServices {
            games,
            profiles,
            wallets,
            achievements,
            atlas,
            guild_settings,
            consents,
        } = services;
        Self {
            prefix,
            started_at: Instant::now(),
            games,
            profiles,
            wallets,
            achievements,
            atlas,
            guild_settings,
            consents,
        }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Resolve the active guild prefix, falling back to DEFAULT_PREFIX.
    pub fn command_prefix(&self, guild_id: Option<poise::serenity_prelude::GuildId>) -> String {
        guild_id
            .and_then(|id| self.guild_settings.get(&id.to_string()))
            .map(|settings| settings.prefix)
            .unwrap_or_else(|| self.prefix.clone())
    }

    pub fn guild_settings(&self) -> &guild_settings::GuildSettingsStore {
        &self.guild_settings
    }

    pub fn consents(&self) -> &consent::ConsentStore {
        &self.consents
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
