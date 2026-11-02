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
    pub emoji: EmojiKey,
    pub fallback: &'static str,
}

/// Real destinations currently backed by registered commands.
pub const AVAILABLE_DESTINATIONS: [TavernDestination; 6] = [
    TavernDestination {
        id: "arcade",
        title: "The Common Tables",
        description: "Browse the playable games and start a solo session or an open lobby.",
        command: "games",
        emoji: EmojiKey::Chat,
        fallback: "🎮",
    },
    TavernDestination {
        id: "player-profile",
        title: "Your Place",
        description: "View your private player profile and saved preferences.",
        command: "profile",
        emoji: EmojiKey::Treasure,
        fallback: "📜",
    },
    TavernDestination {
        id: "player-record",
        title: "The Chronicle",
        description: "Review completed games, wins and draws from your own record.",
        command: "stats",
        emoji: EmojiKey::Crystal,
        fallback: "🏆",
    },
    TavernDestination {
        id: "moonbits-ledger",
        title: "The Ledger",
        description: "View earned-only Moonbits and first-completion milestones.",
        command: "wallet",
        emoji: EmojiKey::Rocket,
        fallback: "🪙",
    },
    TavernDestination {
        id: "achievement-gallery",
        title: "The Gallery",
        description: "Browse achievements unlocked through completed games.",
        command: "achievements",
        emoji: EmojiKey::Crystal,
        fallback: "🏅",
    },
    TavernDestination {
        id: "world-atlas",
        title: "The Atlas",
        description: "Inspect your current place, charted discoveries and directly connected routes.",
        command: "atlas",
        emoji: EmojiKey::Rocket,
        fallback: "🧭",
    },
];

/// Render the initial Tavern Wayfinder using only live commands.
pub fn render(prefix: &str) -> String {
    let prefix = safe_prefix(prefix);
    let brand = emoji::render(EmojiKey::Moonbyte, "🌙");
    let destinations = AVAILABLE_DESTINATIONS
        .iter()
        .map(|destination| {
            let icon = emoji::render(destination.emoji, destination.fallback);
            format!(
                "{icon} **{}**\n{}\n→ `{prefix}{}`",
                destination.title, destination.description, destination.command
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "{brand} **The Tavern — Lantern and Ladle**\n\n\
         A quiet place to choose your next stop. The Wayfinder shows destinations that are \
         available in this build; browsing never joins a game or changes your progress.\n\n\
         {destinations}\n\n\
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
        assert_eq!(AVAILABLE_DESTINATIONS.len(), 6);
        assert_eq!(
            AVAILABLE_DESTINATIONS.map(|destination| destination.command),
            [
                "games",
                "profile",
                "stats",
                "wallet",
                "achievements",
                "atlas"
            ]
        );

        let home = render(".");
        assert!(home.contains(".games"));
        assert!(home.contains(".profile"));
        assert!(home.contains(".stats"));
        assert!(home.contains(".wallet"));
        assert!(home.contains("The Ledger"));
        assert!(home.contains(".achievements"));
        assert!(home.contains("The Gallery"));
        assert!(home.contains(".atlas"));
        assert!(home.contains("The Atlas"));
        assert!(home.contains("The Common Tables"));
        assert!(home.contains("The Chronicle"));
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
