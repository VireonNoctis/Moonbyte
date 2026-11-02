//! Tavern home and Wayfinder presentation.
//!
//! This first slice links only to player-facing features that are implemented.
//! The broader World Loom destinations are added when their domain services exist.

use crate::emoji::{self, EmojiKey};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TavernDestination {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub command: &'static str,
    pub symbol: &'static str,
}

/// Real destinations currently backed by registered commands.
pub const AVAILABLE_DESTINATIONS: [TavernDestination; 3] = [
    TavernDestination {
        id: "arcade",
        title: "The Common Tables",
        description: "Browse the four playable games and start a solo game or lobby.",
        command: "games",
        symbol: "🎮",
    },
    TavernDestination {
        id: "player-profile",
        title: "Your Place",
        description: "View your private player profile and saved preferences.",
        command: "profile",
        symbol: "📜",
    },
    TavernDestination {
        id: "player-record",
        title: "The Chronicle",
        description: "Review completed games, wins and draws from your own record.",
        command: "stats",
        symbol: "🏆",
    },
];

/// Render the initial Tavern Wayfinder using only live commands.
pub fn render(prefix: &str, available_games: usize) -> String {
    let prefix = safe_prefix(prefix);
    let brand = emoji::render(EmojiKey::Moonbyte, "🌙");
    let tables = emoji::render(EmojiKey::Chat, "🎮");
    let profile = emoji::render(EmojiKey::Treasure, "📜");
    let records = emoji::render(EmojiKey::Crystal, "🏆");

    format!(
        "{brand} **The Tavern — Lantern and Ladle**\n\n\
         A quiet place to choose your next stop. The Wayfinder shows destinations that are \
         available in this build; nothing joins a game or changes your progress until you \
         choose a command.\n\n\
         {tables} **The Common Tables**\n\
         Browse {available_games} playable games, then start a solo session or an open lobby.\n\
         → `{prefix}games`\n\n\
         {profile} **Your Place**\n\
         Open your private profile and preferences.\n\
         → `{prefix}profile`\n\n\
         {records} **The Chronicle**\n\
         Review your completed-game record and per-game results.\n\
         → `{prefix}stats`\n\n\
         Use `{prefix}help` for every available command. The wider Atlas, Expeditions, Vaults, \
         Guilds and Bazaar will appear here only as their systems become playable."
    )
}

fn safe_prefix(prefix: &str) -> String {
    prefix
        .replace('@', "@\u{200B}")
        .replace('`', "ˋ")
        .replace('\\', "＼")
}

#[cfg(test)]
mod tests {
    use super::{AVAILABLE_DESTINATIONS, render, safe_prefix};

    #[test]
    fn wayfinder_only_lists_real_registered_destinations() {
        assert_eq!(AVAILABLE_DESTINATIONS.len(), 3);
        assert_eq!(
            AVAILABLE_DESTINATIONS.map(|destination| destination.command),
            ["games", "profile", "stats"]
        );

        let home = render(".", 4);
        assert!(home.contains(".games"));
        assert!(home.contains(".profile"));
        assert!(home.contains(".stats"));
        assert!(home.contains("4 playable games"));
        assert!(home.contains("only as their systems become playable"));
    }

    #[test]
    fn custom_prefix_is_safe_to_display() {
        let prefix = format!("@everyone{} ", char::from(96));
        let safe = safe_prefix(&prefix);
        assert!(!safe.contains("@everyone"));
        assert!(!safe.contains('`'));
    }
}
