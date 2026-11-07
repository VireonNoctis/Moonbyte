//! Player journey, optional daily quests, companion milestones and mutual rivalries.
//!
//! PostgreSQL generates a per-player, per-day UUID seed. SHA-512 provides
//! domain-separated deterministic selection and a rotation fingerprint; it is
//! not an entropy source or a claim that a game outcome is provably fair.

use std::fmt;

use sha2::{Digest, Sha512};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuestDefinition {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub target: i16,
    pub reward_moonbits: i64,
}

pub const DAILY_QUEST_CATALOGUE: [QuestDefinition; 7] = [
    QuestDefinition {
        id: "play_one_game",
        title: "Warm Up the Arcade",
        description: "Complete one game today.",
        target: 1,
        reward_moonbits: 5,
    },
    QuestDefinition {
        id: "play_three_games",
        title: "A Few More Rounds",
        description: "Complete three games today.",
        target: 3,
        reward_moonbits: 8,
    },
    QuestDefinition {
        id: "win_one_game",
        title: "Find Your Rhythm",
        description: "Win one game today.",
        target: 1,
        reward_moonbits: 8,
    },
    QuestDefinition {
        id: "play_two_game_types",
        title: "Try Another Table",
        description: "Complete games from two different game types today.",
        target: 2,
        reward_moonbits: 10,
    },
    QuestDefinition {
        id: "discover_one_place",
        title: "A New Corner of the Map",
        description: "Discover one new Atlas location today.",
        target: 1,
        reward_moonbits: 7,
    },
    QuestDefinition {
        id: "craft_one_item",
        title: "Light the Workshop",
        description: "Craft one keepsake at the Wayfarer Workshop today.",
        target: 1,
        reward_moonbits: 7,
    },
    QuestDefinition {
        id: "complete_one_expedition",
        title: "Walk the Whole Road",
        description: "Complete an authored Expedition today.",
        target: 1,
        reward_moonbits: 10,
    },
];

const DAILY_QUEST_COUNT: usize = 3;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DailyQuest {
    pub quest_date: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub progress: i16,
    pub target: i16,
    pub reward_moonbits: i64,
    pub rotation_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WayfarerSnapshot {
    pub active_days: i64,
    pub companion_name: &'static str,
    pub companion_stage: &'static str,
    pub next_milestone: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RivalryRecord {
    pub rivalry_id: i64,
    pub opponent_id: String,
    pub status: String,
    pub invited_by: String,
    pub matches_played: i64,
    pub your_wins: i64,
    pub opponent_wins: i64,
    pub draws: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub enum WayfarerError {
    Database(sqlx::Error),
    UnknownQuest,
    QuestUnavailable,
    QuestNotComplete,
    QuestAlreadyClaimed,
    RivalSelf,
    RivalNotRegistered,
    RivalAlreadyPending,
    RivalAlreadyActive,
    RivalPreviouslyDeclined,
    InvitationUnavailable,
}

impl fmt::Display for WayfarerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(_) => formatter.write_str("The Wayfarer service is temporarily unavailable."),
            Self::UnknownQuest => formatter.write_str("That daily quest is not part of today's board."),
            Self::QuestUnavailable => formatter.write_str("That quest is not available to accept. Check its current status with .quests."),
            Self::QuestNotComplete => formatter.write_str("That quest is not complete yet. Check .quests for its current progress."),
            Self::QuestAlreadyClaimed => formatter.write_str("That quest reward has already been claimed."),
            Self::RivalSelf => formatter.write_str("You cannot open a rivalry with yourself."),
            Self::RivalNotRegistered => formatter.write_str("That player has not opened Moonbyte yet. They need to use a Moonbyte command before receiving an invitation."),
            Self::RivalAlreadyPending => formatter.write_str("A rivalry invitation for that player is already waiting for a response."),
            Self::RivalAlreadyActive => formatter.write_str("You already have an active rivalry with that player."),
            Self::RivalPreviouslyDeclined => formatter.write_str("That rivalry was previously declined. The other player must initiate a new invitation."),
            Self::InvitationUnavailable => formatter.write_str("That invitation is not available to accept or decline. Check .rivals for current invitations."),
        }
    }
}

impl std::error::Error for WayfarerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for WayfarerError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Clone)]
pub struct WayfarerStore {
    pool: PgPool,
}

impl WayfarerStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn daily_quests(&self, player_id: &str) -> Result<Vec<DailyQuest>, WayfarerError> {
        let mut transaction = self.pool.begin().await?;
        ensure_daily_quests(&mut transaction, player_id).await?;
        let rows = sqlx::query_as::<_, (String, String, String, i16, i16, i64, String)>(
            "SELECT q.quest_date::text, q.quest_id, q.status, q.progress, q.target, \
             q.reward_moonbits, r.commitment_sha512 \
             FROM player_daily_quests q \
             JOIN player_quest_rotations r USING (discord_user_id, quest_date) \
             WHERE q.discord_user_id = $1 AND q.quest_date = CURRENT_DATE \
             ORDER BY q.created_at ASC, q.quest_id ASC",
        )
        .bind(player_id)
        .fetch_all(&mut *transaction)
        .await?;
        transaction.commit().await?;

        Ok(rows
            .into_iter()
            .filter_map(
                |(quest_date, id, status, progress, target, reward_moonbits, fingerprint)| {
                    quest_definition(&id).map(|definition| DailyQuest {
                        quest_date,
                        id,
                        title: definition.title.to_owned(),
                        description: definition.description.to_owned(),
                        status,
                        progress,
                        target,
                        reward_moonbits,
                        rotation_fingerprint: fingerprint,
                    })
                },
            )
            .collect())
    }

    pub async fn accept_quest(&self, player_id: &str, quest_id: &str) -> Result<(), WayfarerError> {
        if quest_definition(quest_id).is_none() {
            return Err(WayfarerError::UnknownQuest);
        }
        let mut transaction = self.pool.begin().await?;
        ensure_daily_quests(&mut transaction, player_id).await?;
        let changed = sqlx::query(
            "UPDATE player_daily_quests SET status = 'active', updated_at = now() \
             WHERE discord_user_id = $1 AND quest_date = CURRENT_DATE \
               AND quest_id = $2 AND status = 'available'",
        )
        .bind(player_id)
        .bind(quest_id)
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if changed != 1 {
            return Err(WayfarerError::QuestUnavailable);
        }
        transaction.commit().await?;
        Ok(())
    }

    pub async fn claim_quest(&self, player_id: &str, quest_id: &str) -> Result<i64, WayfarerError> {
        if quest_definition(quest_id).is_none() {
            return Err(WayfarerError::UnknownQuest);
        }
        let mut transaction = self.pool.begin().await?;
        let row = sqlx::query_as::<_, (String, i64, String)>(
            "SELECT status, reward_moonbits, quest_date::text \
             FROM player_daily_quests \
             WHERE discord_user_id = $1 AND quest_id = $2 AND status = 'completed' \
             ORDER BY quest_date ASC LIMIT 1 FOR UPDATE",
        )
        .bind(player_id)
        .bind(quest_id)
        .fetch_optional(&mut *transaction)
        .await?;

        let Some((status, reward, quest_date)) = row else {
            let latest_status: Option<String> = sqlx::query_scalar(
                "SELECT status FROM player_daily_quests \
                 WHERE discord_user_id = $1 AND quest_id = $2 \
                 ORDER BY quest_date DESC LIMIT 1",
            )
            .bind(player_id)
            .bind(quest_id)
            .fetch_optional(&mut *transaction)
            .await?;
            return Err(match latest_status.as_deref() {
                Some("claimed") => WayfarerError::QuestAlreadyClaimed,
                Some(_) => WayfarerError::QuestNotComplete,
                None => WayfarerError::UnknownQuest,
            });
        };
        if status == "claimed" {
            return Err(WayfarerError::QuestAlreadyClaimed);
        }
        if status != "completed" {
            return Err(WayfarerError::QuestNotComplete);
        }

        let credited = crate::wallet::award_daily_quest_completion(
            &mut transaction,
            player_id,
            &quest_date,
            quest_id,
            reward,
        )
        .await?;
        if !credited {
            return Err(WayfarerError::QuestAlreadyClaimed);
        }

        sqlx::query(
            "UPDATE player_daily_quests SET status = 'claimed', claimed_at = now(), updated_at = now() \
             WHERE discord_user_id = $1 AND quest_date = $2::date AND quest_id = $3",
        )
        .bind(player_id)
        .bind(&quest_date)
        .bind(quest_id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;
        Ok(reward)
    }

    pub async fn snapshot(&self, player_id: &str) -> Result<WayfarerSnapshot, WayfarerError> {
        let active_days: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_wayfarer_days WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let (stage, next_milestone) = companion_stage(active_days);
        Ok(WayfarerSnapshot {
            active_days,
            companion_name: "Mallow",
            companion_stage: stage,
            next_milestone,
        })
    }

    pub async fn rivalries(&self, player_id: &str) -> Result<Vec<RivalryRecord>, WayfarerError> {
        let rows = sqlx::query_as::<
            _,
            (
                i64,
                String,
                String,
                String,
                i64,
                i64,
                i64,
                i64,
                String,
                String,
            ),
        >(
            "SELECT rivalry_id, \
             CASE WHEN player_a_id = $1 THEN player_b_id ELSE player_a_id END, \
             status, invited_by, matches_played, \
             CASE WHEN player_a_id = $1 THEN player_a_wins ELSE player_b_wins END, \
             CASE WHEN player_a_id = $1 THEN player_b_wins ELSE player_a_wins END, \
             draws, created_at::text, updated_at::text \
             FROM player_rivalries WHERE player_a_id = $1 OR player_b_id = $1 \
             ORDER BY updated_at DESC, rivalry_id DESC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(
                |(
                    rivalry_id,
                    opponent_id,
                    status,
                    invited_by,
                    matches_played,
                    your_wins,
                    opponent_wins,
                    draws,
                    created_at,
                    updated_at,
                )| {
                    RivalryRecord {
                        rivalry_id,
                        opponent_id,
                        status,
                        invited_by,
                        matches_played,
                        your_wins,
                        opponent_wins,
                        draws,
                        created_at,
                        updated_at,
                    }
                },
            )
            .collect())
    }

    pub async fn declare_rival(
        &self,
        player_id: &str,
        opponent_id: &str,
    ) -> Result<bool, WayfarerError> {
        if player_id == opponent_id {
            return Err(WayfarerError::RivalSelf);
        }
        let registered: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM player_profiles WHERE discord_user_id = $1)",
        )
        .bind(opponent_id)
        .fetch_one(&self.pool)
        .await?;
        if !registered {
            return Err(WayfarerError::RivalNotRegistered);
        }

        let (player_a, player_b) = if player_id < opponent_id {
            (player_id, opponent_id)
        } else {
            (opponent_id, player_id)
        };
        let mut transaction = self.pool.begin().await?;
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1 || ':' || $2, 0))")
            .bind(player_a)
            .bind(player_b)
            .execute(&mut *transaction)
            .await?;

        let existing = sqlx::query_as::<_, (String, String)>(
            "SELECT status, invited_by FROM player_rivalries \
             WHERE player_a_id = $1 AND player_b_id = $2 FOR UPDATE",
        )
        .bind(player_a)
        .bind(player_b)
        .fetch_optional(&mut *transaction)
        .await?;

        let became_active = match existing {
            Some((status, _)) if status == "active" => {
                return Err(WayfarerError::RivalAlreadyActive);
            }
            Some((status, invited_by)) if status == "declined" && invited_by != player_id => {
                // The player who originally declined may later opt in by initiating a fresh invite.
                sqlx::query(
                    "UPDATE player_rivalries SET status = 'pending', invited_by = $3, updated_at = now() \
                     WHERE player_a_id = $1 AND player_b_id = $2",
                )
                .bind(player_a)
                .bind(player_b)
                .bind(player_id)
                .execute(&mut *transaction)
                .await?;
                false
            }
            Some((status, _)) if status == "declined" => {
                return Err(WayfarerError::RivalPreviouslyDeclined);
            }
            Some((status, invited_by)) if status == "pending" && invited_by == player_id => {
                return Err(WayfarerError::RivalAlreadyPending);
            }
            Some((status, invited_by)) if status == "pending" && invited_by == opponent_id => {
                sqlx::query(
                    "UPDATE player_rivalries SET status = 'active', updated_at = now() \
                     WHERE player_a_id = $1 AND player_b_id = $2",
                )
                .bind(player_a)
                .bind(player_b)
                .execute(&mut *transaction)
                .await?;
                true
            }
            Some(_) => return Err(WayfarerError::RivalAlreadyPending),
            None => {
                sqlx::query(
                    "INSERT INTO player_rivalries (player_a_id, player_b_id, invited_by) \
                     VALUES ($1, $2, $3)",
                )
                .bind(player_a)
                .bind(player_b)
                .bind(player_id)
                .execute(&mut *transaction)
                .await?;
                false
            }
        };
        transaction.commit().await?;
        Ok(became_active)
    }

    pub async fn accept_rival(
        &self,
        player_id: &str,
        rivalry_id: i64,
    ) -> Result<(), WayfarerError> {
        let changed = sqlx::query(
            "UPDATE player_rivalries SET status = 'active', updated_at = now() \
             WHERE rivalry_id = $1 AND status = 'pending' AND invited_by <> $2 \
               AND (player_a_id = $2 OR player_b_id = $2)",
        )
        .bind(rivalry_id)
        .bind(player_id)
        .execute(&self.pool)
        .await?
        .rows_affected();
        if changed == 1 {
            Ok(())
        } else {
            Err(WayfarerError::InvitationUnavailable)
        }
    }

    pub async fn decline_rival(
        &self,
        player_id: &str,
        rivalry_id: i64,
    ) -> Result<(), WayfarerError> {
        let changed = sqlx::query(
            "UPDATE player_rivalries SET status = 'declined', updated_at = now() \
             WHERE rivalry_id = $1 AND status = 'pending' AND invited_by <> $2 \
               AND (player_a_id = $2 OR player_b_id = $2)",
        )
        .bind(rivalry_id)
        .bind(player_id)
        .execute(&self.pool)
        .await?
        .rows_affected();
        if changed == 1 {
            Ok(())
        } else {
            Err(WayfarerError::InvitationUnavailable)
        }
    }
}

fn quest_definition(id: &str) -> Option<QuestDefinition> {
    DAILY_QUEST_CATALOGUE
        .iter()
        .copied()
        .find(|quest| quest.id == id)
}

fn companion_stage(active_days: i64) -> (&'static str, Option<i64>) {
    let milestones = [
        (100, "Elder Star"),
        (60, "Constellation Keeper"),
        (30, "Starbound"),
        (14, "Wayfinder"),
        (7, "Trail Scout"),
        (3, "Pathfinder"),
        (1, "New Companion"),
    ];
    let stage = milestones
        .iter()
        .find(|(threshold, _)| active_days >= *threshold)
        .map(|(_, stage)| *stage)
        .unwrap_or("Waiting at Lantern Square");
    let next = milestones
        .iter()
        .rev()
        .find(|(threshold, _)| active_days < *threshold)
        .map(|(threshold, _)| *threshold);
    (stage, next)
}

fn commitment(seed: &str, day: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(b"moonbyte/quest-rotation-commit/v1\0");
    hasher.update(seed.as_bytes());
    hasher.update([0]);
    hasher.update(day.as_bytes());
    hex_digest(&hasher.finalize())
}

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn sample_index(seed: &str, day: &str, upper: usize, counter: &mut u64) -> usize {
    let upper = u64::try_from(upper).expect("quest catalogue size fits u64");
    assert!(upper > 0);
    let threshold = upper.wrapping_neg() % upper;
    loop {
        let mut hasher = Sha512::new();
        hasher.update(b"moonbyte/quest-selection/v1\0");
        hasher.update(seed.as_bytes());
        hasher.update([0]);
        hasher.update(day.as_bytes());
        hasher.update(counter.to_be_bytes());
        *counter = (*counter).wrapping_add(1);
        let digest = hasher.finalize();
        let mut word = [0_u8; 8];
        word.copy_from_slice(&digest[..8]);
        let value = u64::from_be_bytes(word);
        if value >= threshold {
            return (value % upper) as usize;
        }
    }
}

fn select_daily_quests(seed: &str, day: &str) -> Vec<&'static str> {
    let mut indexes = (0..DAILY_QUEST_CATALOGUE.len()).collect::<Vec<_>>();
    let mut counter = 0_u64;
    for upper in (2..=indexes.len()).rev() {
        let index = sample_index(seed, day, upper, &mut counter);
        indexes.swap(upper - 1, index);
    }
    indexes
        .into_iter()
        .take(DAILY_QUEST_COUNT)
        .map(|index| DAILY_QUEST_CATALOGUE[index].id)
        .collect()
}

async fn ensure_daily_quests(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    let day: String = sqlx::query_scalar("SELECT CURRENT_DATE::text")
        .fetch_one(&mut **transaction)
        .await?;
    let stored_seed: Option<String> = sqlx::query_scalar(
        "SELECT rotation_seed FROM player_quest_rotations \
         WHERE discord_user_id = $1 AND quest_date = CURRENT_DATE FOR UPDATE",
    )
    .bind(player_id)
    .fetch_optional(&mut **transaction)
    .await?;

    let seed = if let Some(seed) = stored_seed {
        seed
    } else {
        // PostgreSQL 17's gen_random_uuid() uses its server-side strong random source.
        let seed: String = sqlx::query_scalar("SELECT gen_random_uuid()::text")
            .fetch_one(&mut **transaction)
            .await?;
        let fingerprint = commitment(&seed, &day);
        sqlx::query(
            "INSERT INTO player_quest_rotations \
             (discord_user_id, quest_date, rotation_seed, commitment_sha512) \
             VALUES ($1, CURRENT_DATE, $2, $3) ON CONFLICT DO NOTHING",
        )
        .bind(player_id)
        .bind(&seed)
        .bind(fingerprint)
        .execute(&mut **transaction)
        .await?;
        sqlx::query_scalar(
            "SELECT rotation_seed FROM player_quest_rotations \
             WHERE discord_user_id = $1 AND quest_date = CURRENT_DATE",
        )
        .bind(player_id)
        .fetch_one(&mut **transaction)
        .await?
    };

    for quest_id in select_daily_quests(&seed, &day) {
        let definition = quest_definition(quest_id).expect("selected quest belongs to catalogue");
        sqlx::query(
            "INSERT INTO player_daily_quests \
             (discord_user_id, quest_date, quest_id, target, reward_moonbits) \
             VALUES ($1, CURRENT_DATE, $2, $3, $4) ON CONFLICT DO NOTHING",
        )
        .bind(player_id)
        .bind(quest_id)
        .bind(definition.target)
        .bind(definition.reward_moonbits)
        .execute(&mut **transaction)
        .await?;
    }
    Ok(())
}

async fn ensure_player(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    // Profile creation belongs to the consent-aware ProfileStore. Domain events
    // must never recreate a profile after deletion has removed consent.
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1 FROM player_profiles WHERE discord_user_id = $1
        )",
    )
    .bind(player_id)
    .fetch_one(&mut **transaction)
    .await?;
    if !exists {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

async fn record_activity_day(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO player_wayfarer_days (discord_user_id, activity_date) \
         VALUES ($1, CURRENT_DATE) ON CONFLICT DO NOTHING",
    )
    .bind(player_id)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

enum QuestEvent<'a> {
    Game { game_id: &'a str, won: bool },
    AtlasDiscovery,
    Crafting,
    ExpeditionCompletion,
}

async fn progress_active_quests(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    event: QuestEvent<'_>,
) -> Result<(), sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, i16, i16, Vec<String>)>(
        "SELECT quest_id, progress, target, completed_game_ids \
         FROM player_daily_quests WHERE discord_user_id = $1 \
           AND quest_date = CURRENT_DATE AND status = 'active' FOR UPDATE",
    )
    .bind(player_id)
    .fetch_all(&mut **transaction)
    .await?;

    for (quest_id, progress, target, mut game_ids) in rows {
        let should_advance = match (&event, quest_id.as_str()) {
            (QuestEvent::Game { .. }, "play_one_game" | "play_three_games") => true,
            (QuestEvent::Game { won: true, .. }, "win_one_game") => true,
            (QuestEvent::Game { game_id, .. }, "play_two_game_types") => {
                if game_ids.iter().any(|existing| existing == game_id) {
                    false
                } else {
                    game_ids.push((*game_id).to_owned());
                    true
                }
            }
            (QuestEvent::AtlasDiscovery, "discover_one_place") => true,
            (QuestEvent::Crafting, "craft_one_item") => true,
            (QuestEvent::ExpeditionCompletion, "complete_one_expedition") => true,
            _ => false,
        };
        if !should_advance {
            continue;
        }
        let next = progress.saturating_add(1).min(target);
        let status = if next >= target {
            "completed"
        } else {
            "active"
        };
        sqlx::query(
            "UPDATE player_daily_quests SET progress = $3, completed_game_ids = $4, \
             status = $5, completed_at = CASE WHEN $5 = 'completed' \
             THEN COALESCE(completed_at, now()) ELSE completed_at END, updated_at = now() \
             WHERE discord_user_id = $1 AND quest_date = CURRENT_DATE AND quest_id = $2 \
               AND status = 'active'",
        )
        .bind(player_id)
        .bind(quest_id)
        .bind(next)
        .bind(&game_ids)
        .bind(status)
        .execute(&mut **transaction)
        .await?;
    }
    Ok(())
}

/// Record a completed game within the same transaction as result/stat/reward writes.
pub(crate) async fn record_game_completion(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    game_id: &str,
    won: bool,
) -> Result<(), sqlx::Error> {
    ensure_player(transaction, player_id).await?;
    ensure_daily_quests(transaction, player_id).await?;
    record_activity_day(transaction, player_id).await?;
    progress_active_quests(transaction, player_id, QuestEvent::Game { game_id, won }).await
}

/// A first-time Atlas discovery counts as meaningful activity; revisits do not.
pub(crate) async fn record_atlas_discovery(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    ensure_player(transaction, player_id).await?;
    ensure_daily_quests(transaction, player_id).await?;
    record_activity_day(transaction, player_id).await?;
    progress_active_quests(transaction, player_id, QuestEvent::AtlasDiscovery).await
}

/// Count a completed authored Expedition and progress its optional daily objective.
pub(crate) async fn record_expedition_completion(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    ensure_player(transaction, player_id).await?;
    ensure_daily_quests(transaction, player_id).await?;
    record_activity_day(transaction, player_id).await?;
    progress_active_quests(transaction, player_id, QuestEvent::ExpeditionCompletion).await
}

/// Count a successful first-time craft and progress its optional daily objective.
pub(crate) async fn record_crafting_completion(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    ensure_player(transaction, player_id).await?;
    ensure_daily_quests(transaction, player_id).await?;
    record_activity_day(transaction, player_id).await?;
    progress_active_quests(transaction, player_id, QuestEvent::Crafting).await
}

/// Record completion of a verified Puzzle Vault as meaningful activity. No current
/// daily quest consumes this event, so completion does not change quest progress.
pub(crate) async fn record_vault_completion(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    ensure_player(transaction, player_id).await?;
    ensure_daily_quests(transaction, player_id).await?;
    record_activity_day(transaction, player_id).await
}

/// Record post-acceptance head-to-head results only for a two-player active rivalry.
pub(crate) async fn record_rivalry_match(
    transaction: &mut Transaction<'_, Postgres>,
    players: &[String],
    winner: Option<&str>,
) -> Result<(), sqlx::Error> {
    if players.len() != 2 {
        return Ok(());
    }
    let first = &players[0];
    let second = &players[1];
    sqlx::query(
        "UPDATE player_rivalries SET matches_played = matches_played + 1, \
         player_a_wins = player_a_wins + CASE WHEN player_a_id = $3 THEN 1 ELSE 0 END, \
         player_b_wins = player_b_wins + CASE WHEN player_b_id = $3 THEN 1 ELSE 0 END, \
         draws = draws + CASE WHEN $3::text IS NULL THEN 1 ELSE 0 END, updated_at = now() \
         WHERE status = 'active' AND ((player_a_id = $1 AND player_b_id = $2) \
             OR (player_a_id = $2 AND player_b_id = $1))",
    )
    .bind(first)
    .bind(second)
    .bind(winner)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{DAILY_QUEST_CATALOGUE, commitment, companion_stage, select_daily_quests};

    #[test]
    fn daily_quest_rotation_is_deterministic_and_contains_unique_entries() {
        let first = select_daily_quests("test-seed", "2026-11-03");
        let repeated = select_daily_quests("test-seed", "2026-11-03");
        let next_day = select_daily_quests("test-seed", "2026-11-04");
        assert_eq!(first, repeated);
        assert_eq!(first.len(), 3);
        assert_eq!(
            first.iter().collect::<std::collections::HashSet<_>>().len(),
            3
        );
        assert_ne!(first, next_day);
        assert!(
            first
                .iter()
                .all(|id| DAILY_QUEST_CATALOGUE.iter().any(|quest| quest.id == *id))
        );
    }

    #[test]
    fn commitment_is_sha512_and_uses_a_day_domain() {
        let hash = commitment("seed", "2026-11-03");
        assert_eq!(hash.len(), 128);
        assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_ne!(hash, commitment("seed", "2026-11-04"));
    }

    #[tokio::test]
    async fn daily_quest_rotation_is_persisted_and_reward_is_credited_once() {
        use crate::{database::Database, profile::ProfileStore};
        use std::time::{SystemTime, UNIX_EPOCH};

        let url = std::env::var("DATABASE_URL").expect("PostgreSQL integration test database");
        let database = Database::connect(&url).await.expect("connect and migrate");
        let profiles = ProfileStore::new(database.pool());
        let store = super::WayfarerStore::new(database.pool());
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
            % 100_000_000_000_000_000;
        let player_id = format!("1{nonce:017}");
        crate::profile::create_test_player(database.pool(), &player_id, "Quest Tester")
            .await
            .expect("create consented profile");

        let first = store
            .daily_quests(&player_id)
            .await
            .expect("create daily board");
        let second = store
            .daily_quests(&player_id)
            .await
            .expect("load saved daily board");
        assert_eq!(
            first, second,
            "the same player must receive one stable board each day"
        );
        assert_eq!(first.len(), 3);
        assert_eq!(
            first
                .iter()
                .map(|quest| quest.id.as_str())
                .collect::<std::collections::HashSet<_>>()
                .len(),
            3
        );
        assert!(
            first
                .iter()
                .all(|quest| quest.rotation_fingerprint.len() == 128)
        );

        let quest = &first[0];
        store
            .accept_quest(&player_id, &quest.id)
            .await
            .expect("accept quest");
        sqlx::query(
            "UPDATE player_daily_quests SET progress = target, status = 'completed', completed_at = now() \
             WHERE discord_user_id = $1 AND quest_date = CURRENT_DATE AND quest_id = $2",
        )
        .bind(&player_id)
        .bind(&quest.id)
        .execute(&database.pool())
        .await
        .expect("prepare completed quest fixture");

        let reward = store
            .claim_quest(&player_id, &quest.id)
            .await
            .expect("claim reward");
        assert_eq!(reward, quest.reward_moonbits);
        assert!(matches!(
            store.claim_quest(&player_id, &quest.id).await,
            Err(super::WayfarerError::QuestAlreadyClaimed)
        ));
        let balance: i64 =
            sqlx::query_scalar("SELECT balance FROM player_wallets WHERE discord_user_id = $1")
                .bind(&player_id)
                .fetch_one(&database.pool())
                .await
                .expect("read wallet");
        assert_eq!(balance, reward);

        profiles
            .delete_account(&player_id)
            .await
            .expect("remove test player data");
    }

    #[tokio::test]
    async fn rivalries_require_mutual_acceptance_and_record_only_later_games() {
        use crate::{database::Database, profile::ProfileStore};
        use std::time::{SystemTime, UNIX_EPOCH};

        let url = std::env::var("DATABASE_URL").expect("PostgreSQL integration test database");
        let database = Database::connect(&url).await.expect("connect and migrate");
        let profiles = ProfileStore::new(database.pool());
        let store = super::WayfarerStore::new(database.pool());
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
            % 100_000_000_000_000_000;
        let first_id = format!("1{nonce:017}");
        let second_id = format!("2{nonce:017}");
        crate::profile::create_test_player(database.pool(), &first_id, "First Pilot")
            .await
            .expect("create consented first profile");
        crate::profile::create_test_player(database.pool(), &second_id, "Second Pilot")
            .await
            .expect("create consented second profile");

        assert!(
            !store
                .declare_rival(&first_id, &second_id)
                .await
                .expect("send invite")
        );
        let invitation = store
            .rivalries(&first_id)
            .await
            .expect("read invitation")
            .into_iter()
            .next()
            .expect("one pending invitation");
        assert_eq!(invitation.status, "pending");
        assert!(
            store
                .accept_rival(&first_id, invitation.rivalry_id)
                .await
                .is_err()
        );
        store
            .accept_rival(&second_id, invitation.rivalry_id)
            .await
            .expect("recipient accepts");

        let mut transaction = database
            .pool()
            .begin()
            .await
            .expect("begin match transaction");
        super::record_rivalry_match(
            &mut transaction,
            &[first_id.clone(), second_id.clone()],
            Some(&first_id),
        )
        .await
        .expect("record active rivalry match");
        transaction.commit().await.expect("commit match");

        let first_record = store.rivalries(&first_id).await.expect("first record");
        let second_record = store.rivalries(&second_id).await.expect("second record");
        assert_eq!(first_record[0].status, "active");
        assert_eq!(first_record[0].matches_played, 1);
        assert_eq!(first_record[0].your_wins, 1);
        assert_eq!(second_record[0].opponent_wins, 1);

        profiles
            .delete_account(&first_id)
            .await
            .expect("delete first test profile");
        profiles
            .delete_account(&second_id)
            .await
            .expect("delete second test profile");
    }

    #[test]
    fn companion_milestones_are_monotonic_and_non_punitive() {
        assert_eq!(companion_stage(0).0, "Waiting at Lantern Square");
        assert_eq!(companion_stage(1).0, "New Companion");
        assert_eq!(companion_stage(3).0, "Pathfinder");
        assert_eq!(companion_stage(30).0, "Starbound");
        assert_eq!(companion_stage(100).0, "Elder Star");
        assert_eq!(companion_stage(100).0, companion_stage(120).0);
    }
}
