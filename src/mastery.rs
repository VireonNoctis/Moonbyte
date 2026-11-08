//! Read-only game mastery projection derived from existing per-game statistics.
//!
//! One completed game earns one mastery point; each win adds two bonus points.
//! Ranks are descriptive milestones only. They do not alter game rules, rewards,
//! matchmaking or access to content that has not shipped.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct RankTier {
    threshold: i64,
    name: &'static str,
}

const RANKS: [RankTier; 6] = [
    RankTier {
        threshold: 0,
        name: "Newcomer",
    },
    RankTier {
        threshold: 5,
        name: "Novice",
    },
    RankTier {
        threshold: 15,
        name: "Adept",
    },
    RankTier {
        threshold: 35,
        name: "Veteran",
    },
    RankTier {
        threshold: 75,
        name: "Virtuoso",
    },
    RankTier {
        threshold: 150,
        name: "Mythic",
    },
];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MasteryProgress {
    pub points: i64,
    pub rank: &'static str,
    pub next_rank: Option<&'static str>,
    pub next_rank_threshold: Option<i64>,
    pub points_to_next_rank: i64,
}

/// Calculate a player's current rank from completed games and wins.
/// Inconsistent counters are clamped rather than creating negative progress.
pub fn summarize(games_played: i64, games_won: i64) -> MasteryProgress {
    let completed = games_played.max(0);
    let wins = games_won.clamp(0, completed);
    let points = completed.saturating_add(wins.saturating_mul(2));

    let rank_index = RANKS
        .iter()
        .rposition(|tier| points >= tier.threshold)
        .unwrap_or(0);
    let current = RANKS[rank_index];
    let next = RANKS.get(rank_index + 1).copied();

    MasteryProgress {
        points,
        rank: current.name,
        next_rank: next.map(|tier| tier.name),
        next_rank_threshold: next.map(|tier| tier.threshold),
        points_to_next_rank: next
            .map(|tier| tier.threshold.saturating_sub(points))
            .unwrap_or(0),
    }
}

#[cfg(test)]
mod tests {
    use super::summarize;

    #[test]
    fn mastery_rank_boundaries_are_stable_and_monotonic() {
        for (points, expected) in [
            (0, "Newcomer"),
            (4, "Newcomer"),
            (5, "Novice"),
            (14, "Novice"),
            (15, "Adept"),
            (34, "Adept"),
            (35, "Veteran"),
            (74, "Veteran"),
            (75, "Virtuoso"),
            (149, "Virtuoso"),
            (150, "Mythic"),
        ] {
            assert_eq!(summarize(points, 0).rank, expected, "at {points} points");
        }
    }

    #[test]
    fn each_completion_and_win_contributes_a_transparent_fixed_amount() {
        let progress = summarize(10, 4);
        assert_eq!(progress.points, 18);
        assert_eq!(progress.rank, "Adept");
        assert_eq!(progress.next_rank, Some("Veteran"));
        assert_eq!(progress.next_rank_threshold, Some(35));
        assert_eq!(progress.points_to_next_rank, 17);
    }

    #[test]
    fn inconsistent_or_negative_counters_cannot_create_extra_progress() {
        assert_eq!(summarize(-5, 3).points, 0);
        assert_eq!(summarize(5, -3).points, 5);
        assert_eq!(summarize(5, 99).points, 15);
    }

    #[test]
    fn highest_rank_has_no_phantom_next_rank() {
        let progress = summarize(150, 0);
        assert_eq!(progress.rank, "Mythic");
        assert_eq!(progress.next_rank, None);
        assert_eq!(progress.next_rank_threshold, None);
        assert_eq!(progress.points_to_next_rank, 0);
    }
}
