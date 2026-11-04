//! Persistent player identity and privacy controls.
//!
//! Only fields required by the player-facing profile are stored. Discord remains
//! the identity provider; Moonbyte does not request email, real name, or location.

use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug)]
pub enum ProfileError {
    Database(sqlx::Error),
    Serialization(serde_json::Error),
    InvalidDisplayName,
    InvalidBio,
    InvalidVisibility,
}

impl fmt::Display for ProfileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(_) => {
                formatter.write_str("The player store is temporarily unavailable.")
            }
            Self::Serialization(_) => formatter.write_str("The player data could not be exported."),
            Self::InvalidDisplayName => {
                formatter.write_str("Display names must contain 1 to 32 visible characters.")
            }
            Self::InvalidBio => {
                formatter.write_str("Profile bios must be 160 characters or fewer.")
            }
            Self::InvalidVisibility => {
                formatter.write_str("Visibility must be either public or private.")
            }
        }
    }
}

impl std::error::Error for ProfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for ProfileError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<serde_json::Error> for ProfileError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub discord_user_id: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub visibility: String,
    pub show_statistics: bool,
    pub quiet_mode: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileInventory {
    pub profile_record: bool,
    pub consent_record: bool,
    pub game_session_records: i64,
    pub game_result_records: i64,
    pub game_stat_rows: i64,
    pub achievement_rows: i64,
    pub wallet_record: bool,
    pub wallet_ledger_records: i64,
    pub world_state_record: bool,
    pub atlas_discovery_records: i64,
    pub wayfarer_active_days: i64,
    pub quest_rotation_records: i64,
    pub daily_quest_records: i64,
    pub rivalry_records: i64,
    pub archive_fragment_records: i64,
    pub expedition_run_records: i64,
    pub expedition_checkpoint_records: i64,
    pub session_retention: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerGameStat {
    pub game_id: String,
    pub games_played: i64,
    pub games_won: i64,
    pub draws: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PlayerDeletion {
    pub sessions_removed: u64,
    pub results_removed: u64,
    pub stat_rows_removed: u64,
    pub achievement_rows_removed: u64,
    pub wallet_ledger_records_removed: u64,
    pub wallet_removed: bool,
    pub world_state_removed: bool,
    pub atlas_discoveries_removed: u64,
    pub archive_fragments_removed: u64,
    pub wayfarer_days_removed: u64,
    pub quest_rotations_removed: u64,
    pub daily_quest_records_removed: u64,
    pub rivalry_records_removed: u64,
    pub expedition_participations_removed: u64,
    pub expedition_runs_host_transferred: u64,
    pub expedition_runs_cancelled: u64,
    pub expedition_runs_removed: u64,
    pub consent_record_removed: bool,
    pub profile_removed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedSession {
    session_id: String,
    game_id: String,
    phase: String,
    action_count: i64,
    created_at: String,
    updated_at: String,
    expires_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedResult {
    session_id: String,
    game_id: String,
    outcome: String,
    action_count: i64,
    completed_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedWalletEntry {
    entry_id: i64,
    amount_delta: i64,
    reason: String,
    source_id: String,
    created_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedAchievement {
    achievement_id: String,
    title: String,
    description: String,
    category: String,
    source_session_id: String,
    unlocked_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedWorldState {
    current_location_id: String,
    updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedAtlasDiscovery {
    location_id: String,
    discovered_at: String,
    last_visited_at: String,
    visits: i64,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedArchiveFragment {
    fragment_id: String,
    location_id: String,
    unlocked_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedConsent {
    terms_version: String,
    privacy_version: String,
    accepted_at: String,
    accepted_in_guild_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedDailyQuest {
    quest_date: String,
    quest_id: String,
    status: String,
    progress: i16,
    target: i16,
    reward_moonbits: i64,
    completed_at: Option<String>,
    claimed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedQuestRotation {
    quest_date: String,
    commitment_sha512: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedRivalry {
    rivalry_id: i64,
    opponent_discord_user_id: String,
    status: String,
    invitation_sent_by_you: bool,
    matches_played: i64,
    your_wins: i64,
    opponent_wins: i64,
    draws: i64,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedExpeditionRun {
    expedition_run_id: i64,
    route_id: String,
    content_version: i16,
    guild_id: Option<String>,
    state: String,
    checkpoint_index: i16,
    was_host: bool,
    joined_at: String,
    left_at: Option<String>,
    created_at: String,
    started_at: Option<String>,
    completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ExportedExpeditionCheckpoint {
    expedition_run_id: i64,
    checkpoint_id: String,
    checkpoint_index: i16,
    choice_id: String,
    completed_by_you: bool,
    completed_at: String,
}

#[derive(Debug, Serialize)]
struct PlayerExport {
    format_version: u16,
    exported_at: String,
    data_categories: [&'static str; 17],
    profile: Option<PlayerProfile>,
    consent_record: Option<ExportedConsent>,
    game_sessions: Vec<ExportedSession>,
    game_results: Vec<ExportedResult>,
    game_statistics: Vec<PlayerGameStat>,
    achievements: Vec<ExportedAchievement>,
    wallet_balance: i64,
    wallet_ledger_entries: Vec<ExportedWalletEntry>,
    world_state: Option<ExportedWorldState>,
    atlas_discoveries: Vec<ExportedAtlasDiscovery>,
    archive_fragments: Vec<ExportedArchiveFragment>,
    wayfarer_activity_days: Vec<String>,
    daily_quests: Vec<ExportedDailyQuest>,
    quest_rotation_fingerprints: Vec<ExportedQuestRotation>,
    rivalries: Vec<ExportedRivalry>,
    expedition_runs: Vec<ExportedExpeditionRun>,
    expedition_checkpoints: Vec<ExportedExpeditionCheckpoint>,
    limitations: [&'static str; 3],
}

/// Owns the persistent player profile boundary. Game rules never depend on it.
#[derive(Clone)]
pub struct ProfileStore {
    pool: PgPool,
}

impl ProfileStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_or_create(
        &self,
        player_id: &str,
        default_name: &str,
    ) -> Result<PlayerProfile, ProfileError> {
        let display_name = normalise_display_name(default_name)?;
        sqlx::query(
            "INSERT INTO player_profiles (discord_user_id, display_name) \
             VALUES ($1, $2) ON CONFLICT (discord_user_id) DO NOTHING",
        )
        .bind(player_id)
        .bind(display_name)
        .execute(&self.pool)
        .await?;
        self.get(player_id)
            .await?
            .ok_or_else(|| ProfileError::Database(sqlx::Error::RowNotFound))
    }

    pub async fn get(&self, player_id: &str) -> Result<Option<PlayerProfile>, ProfileError> {
        let row = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                String,
                bool,
                bool,
                String,
                String,
            ),
        >(
            "SELECT discord_user_id, display_name, bio, visibility, show_statistics, quiet_mode, \
             created_at::text, updated_at::text FROM player_profiles WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(
            |(
                discord_user_id,
                display_name,
                bio,
                visibility,
                show_statistics,
                quiet_mode,
                created_at,
                updated_at,
            )| {
                PlayerProfile {
                    discord_user_id,
                    display_name,
                    bio,
                    visibility,
                    show_statistics,
                    quiet_mode,
                    created_at,
                    updated_at,
                }
            },
        ))
    }

    pub async fn update_identity(
        &self,
        player_id: &str,
        display_name: &str,
        bio: &str,
    ) -> Result<PlayerProfile, ProfileError> {
        let display_name = normalise_display_name(display_name)?;
        let bio = normalise_bio(bio)?;
        sqlx::query(
            "UPDATE player_profiles SET display_name = $2, bio = $3, updated_at = now() \
             WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .bind(display_name)
        .bind(bio)
        .execute(&self.pool)
        .await?;
        self.get(player_id)
            .await?
            .ok_or_else(|| ProfileError::Database(sqlx::Error::RowNotFound))
    }

    pub async fn update_preferences(
        &self,
        player_id: &str,
        visibility: &str,
        show_statistics: bool,
        quiet_mode: bool,
    ) -> Result<PlayerProfile, ProfileError> {
        let visibility = match visibility.trim().to_ascii_lowercase().as_str() {
            "public" => "public",
            "private" => "private",
            _ => return Err(ProfileError::InvalidVisibility),
        };
        sqlx::query(
            "UPDATE player_profiles SET visibility = $2, show_statistics = $3, \
             quiet_mode = $4, updated_at = now() WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .bind(visibility)
        .bind(show_statistics)
        .bind(quiet_mode)
        .execute(&self.pool)
        .await?;
        self.get(player_id)
            .await?
            .ok_or_else(|| ProfileError::Database(sqlx::Error::RowNotFound))
    }

    pub async fn inventory(&self, player_id: &str) -> Result<ProfileInventory, ProfileError> {
        let profile_record: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM player_profiles WHERE discord_user_id = $1)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let consent_record: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM user_consents WHERE discord_user_id = $1)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        // Include expired-but-not-yet-cleaned rows: they still physically exist.
        let game_session_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM game_sessions \
             WHERE (session_state -> 'players') @> jsonb_build_array($1::text)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;

        let game_result_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM game_results \
             WHERE player_ids @> jsonb_build_array($1::text)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let game_stat_rows: i64 =
            sqlx::query_scalar("SELECT count(*) FROM player_game_stats WHERE discord_user_id = $1")
                .bind(player_id)
                .fetch_one(&self.pool)
                .await?;
        let achievement_rows: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_achievements WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let wallet_record: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM player_wallets WHERE discord_user_id = $1)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let wallet_ledger_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM wallet_ledger_entries WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let world_state_record: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM player_world_state WHERE discord_user_id = $1)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let atlas_discovery_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_atlas_discoveries WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let archive_fragment_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_archive_fragments WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;

        let wayfarer_active_days: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_wayfarer_days WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let quest_rotation_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_quest_rotations WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let daily_quest_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_daily_quests WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let rivalry_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_rivalries WHERE player_a_id = $1 OR player_b_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let expedition_run_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM expedition_participants WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let expedition_checkpoint_records: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM expedition_checkpoint_history h
             JOIN expedition_participants p USING (expedition_run_id)
             WHERE p.discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(ProfileInventory {
            profile_record,
            consent_record,
            game_session_records,
            game_result_records,
            game_stat_rows,
            achievement_rows,
            wallet_record,
            wallet_ledger_records,
            world_state_record,
            atlas_discovery_records,
            wayfarer_active_days,
            quest_rotation_records,
            daily_quest_records,
            rivalry_records,
            archive_fragment_records,
            expedition_run_records,
            expedition_checkpoint_records,
            session_retention: "Game sessions and their result records expire after 24 hours of inactivity. Expedition participation and route history remain until account deletion; active runs are safely cancelled when no active participants remain.",
        })
    }

    pub async fn game_stats(&self, player_id: &str) -> Result<Vec<PlayerGameStat>, ProfileError> {
        let rows = sqlx::query_as::<_, (String, i64, i64, i64, String)>(
            "SELECT game_id, games_played, games_won, draws, updated_at::text \
             FROM player_game_stats WHERE discord_user_id = $1 \
             ORDER BY games_played DESC, game_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(game_id, games_played, games_won, draws, updated_at)| PlayerGameStat {
                    game_id,
                    games_played,
                    games_won,
                    draws,
                    updated_at,
                },
            )
            .collect())
    }

    pub async fn export_json(&self, player_id: &str) -> Result<String, ProfileError> {
        let profile = self.get(player_id).await?;
        let consent_record = sqlx::query_as::<_, (String, String, String, Option<String>)>(
            "SELECT terms_version, privacy_version, accepted_at::text, accepted_in_guild_id \
             FROM user_consents WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?
        .map(
            |(terms_version, privacy_version, accepted_at, accepted_in_guild_id)| ExportedConsent {
                terms_version,
                privacy_version,
                accepted_at,
                accepted_in_guild_id,
            },
        );
        let rows = sqlx::query_as::<_, (String, String, String, i64, String, String, String)>(
            "SELECT session_id, game_id, phase, action_count, created_at::text, \
             updated_at::text, expires_at::text FROM game_sessions \
             WHERE (session_state -> 'players') @> jsonb_build_array($1::text) \
             ORDER BY created_at DESC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;

        let result_rows = sqlx::query_as::<_, (String, String, String, i64, String)>(
            "SELECT session_id, game_id, outcome, action_count, completed_at::text \
             FROM game_results WHERE player_ids @> jsonb_build_array($1::text) \
             ORDER BY completed_at DESC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;
        let game_results = result_rows
            .into_iter()
            .map(
                |(session_id, game_id, outcome, action_count, completed_at)| ExportedResult {
                    session_id,
                    game_id,
                    outcome,
                    action_count,
                    completed_at,
                },
            )
            .collect();
        let game_statistics = self.game_stats(player_id).await?;
        let achievements = sqlx::query_as::<_, (String, String, String, String, String, String)>(
            "SELECT d.achievement_id, d.title, d.description, d.category, \
             p.source_session_id, p.unlocked_at::text \
             FROM player_achievements p \
             JOIN achievement_definitions d USING (achievement_id) \
             WHERE p.discord_user_id = $1 \
             ORDER BY p.unlocked_at ASC, d.achievement_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(
            |(achievement_id, title, description, category, source_session_id, unlocked_at)| {
                ExportedAchievement {
                    achievement_id,
                    title,
                    description,
                    category,
                    source_session_id,
                    unlocked_at,
                }
            },
        )
        .collect();

        let wallet_balance: i64 = sqlx::query_scalar(
            "SELECT COALESCE((SELECT balance FROM player_wallets WHERE discord_user_id = $1), 0)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;
        let wallet_rows = sqlx::query_as::<_, (i64, i64, String, String, String)>(
            "SELECT entry_id, amount_delta, reason, source_id, created_at::text FROM wallet_ledger_entries WHERE discord_user_id = $1 ORDER BY entry_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;
        let wallet_ledger_entries = wallet_rows
            .into_iter()
            .map(
                |(entry_id, amount_delta, reason, source_id, created_at)| ExportedWalletEntry {
                    entry_id,
                    amount_delta,
                    reason,
                    source_id,
                    created_at,
                },
            )
            .collect();

        let world_state = sqlx::query_as::<_, (String, String)>(
            "SELECT current_location_id, updated_at::text FROM player_world_state WHERE discord_user_id = $1",
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?
        .map(|(current_location_id, updated_at)| ExportedWorldState {
            current_location_id,
            updated_at,
        });
        let atlas_discoveries = sqlx::query_as::<_, (String, String, String, i64)>(
            "SELECT location_id, discovered_at::text, last_visited_at::text, visit_count \
             FROM player_atlas_discoveries WHERE discord_user_id = $1 \
             ORDER BY discovered_at ASC, location_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(
            |(location_id, discovered_at, last_visited_at, visits)| ExportedAtlasDiscovery {
                location_id,
                discovered_at,
                last_visited_at,
                visits,
            },
        )
        .collect();

        let archive_fragments = sqlx::query_as::<_, (String, String, String)>(
            "SELECT fragment_id, location_id, unlocked_at::text FROM player_archive_fragments \
             WHERE discord_user_id = $1 ORDER BY unlocked_at ASC, fragment_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(
            |(fragment_id, location_id, unlocked_at)| ExportedArchiveFragment {
                fragment_id,
                location_id,
                unlocked_at,
            },
        )
        .collect();

        let wayfarer_activity_days: Vec<String> = sqlx::query_scalar(
            "SELECT activity_date::text FROM player_wayfarer_days \
             WHERE discord_user_id = $1 ORDER BY activity_date ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;

        let daily_quests = sqlx::query_as::<
            _,
            (
                String,
                String,
                String,
                i16,
                i16,
                i64,
                Option<String>,
                Option<String>,
            ),
        >(
            "SELECT quest_date::text, quest_id, status, progress, target, reward_moonbits, \
             completed_at::text, claimed_at::text FROM player_daily_quests \
             WHERE discord_user_id = $1 ORDER BY quest_date DESC, quest_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(
            |(
                quest_date,
                quest_id,
                status,
                progress,
                target,
                reward_moonbits,
                completed_at,
                claimed_at,
            )| {
                ExportedDailyQuest {
                    quest_date,
                    quest_id,
                    status,
                    progress,
                    target,
                    reward_moonbits,
                    completed_at,
                    claimed_at,
                }
            },
        )
        .collect();

        // Export only the SHA-512 rotation fingerprint. The private seed is never
        // included in player exports.
        let quest_rotation_fingerprints = sqlx::query_as::<_, (String, String)>(
            "SELECT quest_date::text, commitment_sha512 FROM player_quest_rotations \
             WHERE discord_user_id = $1 ORDER BY quest_date DESC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|(quest_date, commitment_sha512)| ExportedQuestRotation {
            quest_date,
            commitment_sha512,
        })
        .collect();

        let rivalries = sqlx::query_as::<
            _,
            (
                i64,
                String,
                String,
                bool,
                i64,
                i64,
                i64,
                i64,
                String,
                String,
            ),
        >(
            "SELECT rivalry_id, CASE WHEN player_a_id = $1 THEN player_b_id ELSE player_a_id END, \
             status, invited_by = $1, matches_played, \
             CASE WHEN player_a_id = $1 THEN player_a_wins ELSE player_b_wins END, \
             CASE WHEN player_a_id = $1 THEN player_b_wins ELSE player_a_wins END, \
             draws, created_at::text, updated_at::text \
             FROM player_rivalries WHERE player_a_id = $1 OR player_b_id = $1 \
             ORDER BY created_at ASC, rivalry_id ASC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(
            |(
                rivalry_id,
                opponent_discord_user_id,
                status,
                invitation_sent_by_you,
                matches_played,
                your_wins,
                opponent_wins,
                draws,
                created_at,
                updated_at,
            )| {
                ExportedRivalry {
                    rivalry_id,
                    opponent_discord_user_id,
                    status,
                    invitation_sent_by_you,
                    matches_played,
                    your_wins,
                    opponent_wins,
                    draws,
                    created_at,
                    updated_at,
                }
            },
        )
        .collect();

        let exported_at: String = sqlx::query_scalar("SELECT now()::text")
            .fetch_one(&self.pool)
            .await?;

        let game_sessions = rows
            .into_iter()
            .map(
                |(session_id, game_id, phase, action_count, created_at, updated_at, expires_at)| {
                    ExportedSession {
                        session_id,
                        game_id,
                        phase,
                        action_count,
                        created_at,
                        updated_at,
                        expires_at,
                    }
                },
            )
            .collect();

        let expedition_runs = sqlx::query_as::<
            _,
            (
                i64,
                String,
                i16,
                Option<String>,
                String,
                i16,
                bool,
                String,
                Option<String>,
                String,
                Option<String>,
                Option<String>,
            ),
        >(
            "SELECT r.expedition_run_id, r.route_id, r.content_version, r.guild_id,
             r.state, r.current_checkpoint_index,
             COALESCE(r.host_discord_user_id = p.discord_user_id, FALSE),
             p.joined_at::text, p.left_at::text, r.created_at::text,
             r.started_at::text, r.completed_at::text
             FROM expedition_runs r
             JOIN expedition_participants p USING (expedition_run_id)
             WHERE p.discord_user_id = $1
             ORDER BY r.created_at DESC",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(
            |(
                expedition_run_id,
                route_id,
                content_version,
                guild_id,
                state,
                checkpoint_index,
                was_host,
                joined_at,
                left_at,
                created_at,
                started_at,
                completed_at,
            )| ExportedExpeditionRun {
                expedition_run_id,
                route_id,
                content_version,
                guild_id,
                state,
                checkpoint_index,
                was_host,
                joined_at,
                left_at,
                created_at,
                started_at,
                completed_at,
            },
        )
        .collect();
        let expedition_checkpoints =
            sqlx::query_as::<_, (i64, String, i16, String, bool, String)>(
                "SELECT h.expedition_run_id, h.checkpoint_id, h.checkpoint_index,
                 h.choice_id, COALESCE(h.completed_by = $2, FALSE), h.completed_at::text
                 FROM expedition_checkpoint_history h
                 JOIN expedition_participants p USING (expedition_run_id)
                 WHERE p.discord_user_id = $1
                 ORDER BY h.expedition_run_id ASC, h.checkpoint_index ASC",
            )
            .bind(player_id)
            .bind(player_id)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(
                |(
                    expedition_run_id,
                    checkpoint_id,
                    checkpoint_index,
                    choice_id,
                    completed_by_you,
                    completed_at,
                )| ExportedExpeditionCheckpoint {
                    expedition_run_id,
                    checkpoint_id,
                    checkpoint_index,
                    choice_id,
                    completed_by_you,
                    completed_at,
                },
            )
            .collect();

        let export = PlayerExport {
            format_version: 9,
            exported_at,
            data_categories: [
                "player profile and preferences",
                "Terms of Service and privacy-policy acceptance record",
                "game session metadata",
                "completed game results",
                "per-game statistics",
                "unlocked achievements",
                "Moonbits wallet balance",
                "Moonbits wallet ledger entries",
                "current Atlas location",
                "Atlas discoveries and visit history",
                "Chronicle archive fragments and first-discovery timestamps",
                "Wayfarer activity days",
                "daily quest selections and progress",
                "SHA-512 daily quest rotation fingerprints",
                "mutual rivalry invitations and match records",
                "your Expedition participation and run history",
                "shared Expedition checkpoint history",
            ],
            profile,
            consent_record,
            game_sessions,
            game_results,
            game_statistics,
            achievements,
            wallet_balance,
            wallet_ledger_entries,
            world_state,
            atlas_discoveries,
            archive_fragments,
            wayfarer_activity_days,
            daily_quests,
            quest_rotation_fingerprints,
            rivalries,
            expedition_runs,
            expedition_checkpoints,
            limitations: [
                "Discord independently retains interactions under its own policies.",
                "This export includes records in the current Moonbyte PostgreSQL schema only.",
                "Expedition details include only your own participation and shared checkpoint history, not other players' private profiles.",
            ],
        };
        Ok(serde_json::to_string_pretty(&export)?)
    }

    /// Deletes the profile, session snapshots, cascading results and aggregate stats.
    /// Ending shared sessions is intentional: each snapshot contains every participant ID.
    pub async fn delete_account(&self, player_id: &str) -> Result<PlayerDeletion, ProfileError> {
        let mut transaction = self.pool.begin().await?;
        // Serialize against a concurrent acceptance so account deletion cannot
        // leave a policy-consent row behind after its transaction commits.
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('moonbyte:user:' || $1, 0))")
            .bind(player_id)
            .execute(&mut *transaction)
            .await?;

        // Lock matching sessions first. A concurrent completion must finish
        // its result/stat transaction before the privacy deletion can proceed.
        let session_ids: Vec<String> = sqlx::query_scalar(
            "SELECT session_id FROM game_sessions \
             WHERE (session_state -> 'players') @> jsonb_build_array($1::text) \
             FOR UPDATE",
        )
        .bind(player_id)
        .fetch_all(&mut *transaction)
        .await?;
        let results_removed =
            sqlx::query("DELETE FROM game_results WHERE session_id = ANY($1::text[])")
                .bind(&session_ids)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let sessions_removed =
            sqlx::query("DELETE FROM game_sessions WHERE session_id = ANY($1::text[])")
                .bind(&session_ids)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let stat_rows_removed =
            sqlx::query("DELETE FROM player_game_stats WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let achievement_rows_removed =
            sqlx::query("DELETE FROM player_achievements WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let wallet_ledger_records_removed =
            sqlx::query("DELETE FROM wallet_ledger_entries WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let wallet_removed = sqlx::query("DELETE FROM player_wallets WHERE discord_user_id = $1")
            .bind(player_id)
            .execute(&mut *transaction)
            .await?
            .rows_affected()
            > 0;
        let affected_expedition_runs: Vec<i64> = sqlx::query_scalar(
            "SELECT expedition_run_id FROM expedition_participants
             WHERE discord_user_id = $1 ORDER BY expedition_run_id",
        )
        .bind(player_id)
        .fetch_all(&mut *transaction)
        .await?;
        let _locked_expedition_runs = sqlx::query_scalar::<_, i64>(
            "SELECT expedition_run_id FROM expedition_runs
             WHERE expedition_run_id = ANY($1::bigint[])
             ORDER BY expedition_run_id FOR UPDATE",
        )
        .bind(&affected_expedition_runs)
        .fetch_all(&mut *transaction)
        .await?;
        // Atlas travel locks the player's world-state row. Lock it before
        // deletion so an in-flight move cannot recreate state after this commit.
        let _world_state_lock: Option<String> = sqlx::query_scalar(
            "SELECT current_location_id FROM player_world_state WHERE discord_user_id = $1 FOR UPDATE",
        )
        .bind(player_id)
        .fetch_optional(&mut *transaction)
        .await?;
        let world_state_removed =
            sqlx::query("DELETE FROM player_world_state WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected()
                > 0;
        let archive_fragments_removed =
            sqlx::query("DELETE FROM player_archive_fragments WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();

        let atlas_discoveries_removed =
            sqlx::query("DELETE FROM player_atlas_discoveries WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();

        let rivalry_records_removed =
            sqlx::query("DELETE FROM player_rivalries WHERE player_a_id = $1 OR player_b_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let daily_quest_records_removed =
            sqlx::query("DELETE FROM player_daily_quests WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let quest_rotations_removed =
            sqlx::query("DELETE FROM player_quest_rotations WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let wayfarer_days_removed =
            sqlx::query("DELETE FROM player_wayfarer_days WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();

        let expedition_runs_cancelled: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM expedition_runs r
             WHERE r.expedition_run_id = ANY($2::bigint[])
               AND r.state IN ('recruiting', 'active', 'paused')
               AND NOT EXISTS (
                 SELECT 1 FROM expedition_participants p
                 WHERE p.expedition_run_id = r.expedition_run_id
                   AND p.discord_user_id <> $1 AND p.is_active
               )",
        )
        .bind(player_id)
        .bind(&affected_expedition_runs)
        .fetch_one(&mut *transaction)
        .await?;
        let expedition_runs_host_transferred: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM expedition_runs r
             WHERE r.expedition_run_id = ANY($2::bigint[])
               AND r.state IN ('recruiting', 'active', 'paused')
               AND r.host_discord_user_id = $1
               AND EXISTS (
                 SELECT 1 FROM expedition_participants p
                 WHERE p.expedition_run_id = r.expedition_run_id
                   AND p.discord_user_id <> $1 AND p.is_active
               )",
        )
        .bind(player_id)
        .bind(&affected_expedition_runs)
        .fetch_one(&mut *transaction)
        .await?;

        sqlx::query(
            "UPDATE expedition_runs r SET
             host_discord_user_id = CASE WHEN r.host_discord_user_id = $1 THEN (
                 SELECT p.discord_user_id FROM expedition_participants p
                 WHERE p.expedition_run_id = r.expedition_run_id
                   AND p.discord_user_id <> $1 AND p.is_active
                 ORDER BY p.joined_at ASC, p.discord_user_id ASC LIMIT 1
             ) ELSE r.host_discord_user_id END,
             state = CASE WHEN r.state IN ('recruiting', 'active', 'paused')
               AND NOT EXISTS (
                 SELECT 1 FROM expedition_participants p
                 WHERE p.expedition_run_id = r.expedition_run_id
                   AND p.discord_user_id <> $1 AND p.is_active
               ) THEN 'cancelled' ELSE r.state END,
             cancelled_at = CASE WHEN r.state IN ('recruiting', 'active', 'paused')
               AND NOT EXISTS (
                 SELECT 1 FROM expedition_participants p
                 WHERE p.expedition_run_id = r.expedition_run_id
                   AND p.discord_user_id <> $1 AND p.is_active
               ) THEN COALESCE(r.cancelled_at, now()) ELSE r.cancelled_at END,
             row_version = row_version + 1, updated_at = now()
             WHERE r.expedition_run_id = ANY($2::bigint[])
               AND r.state IN ('recruiting', 'active', 'paused')",
        )
        .bind(player_id)
        .bind(&affected_expedition_runs)
        .execute(&mut *transaction)
        .await?;
        let expedition_participations_removed =
            sqlx::query("DELETE FROM expedition_participants WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
        let expedition_runs_removed = sqlx::query(
            "DELETE FROM expedition_runs r
             WHERE r.expedition_run_id = ANY($1::bigint[])
               AND NOT EXISTS (
                 SELECT 1 FROM expedition_participants p
                 WHERE p.expedition_run_id = r.expedition_run_id
               )",
        )
        .bind(&affected_expedition_runs)
        .execute(&mut *transaction)
        .await?
        .rows_affected();

        let consent_record_removed =
            sqlx::query("DELETE FROM user_consents WHERE discord_user_id = $1")
                .bind(player_id)
                .execute(&mut *transaction)
                .await?
                .rows_affected()
                > 0;
        let profile_removed = sqlx::query("DELETE FROM player_profiles WHERE discord_user_id = $1")
            .bind(player_id)
            .execute(&mut *transaction)
            .await?
            .rows_affected()
            > 0;
        transaction.commit().await?;
        Ok(PlayerDeletion {
            sessions_removed,
            results_removed,
            stat_rows_removed,
            achievement_rows_removed,
            wallet_ledger_records_removed,
            wallet_removed,
            world_state_removed,
            atlas_discoveries_removed,
            archive_fragments_removed,
            wayfarer_days_removed,
            quest_rotations_removed,
            daily_quest_records_removed,
            rivalry_records_removed,
            expedition_participations_removed,
            expedition_runs_host_transferred: u64::try_from(expedition_runs_host_transferred).unwrap_or_default(),
            expedition_runs_cancelled: u64::try_from(expedition_runs_cancelled).unwrap_or_default(),
            expedition_runs_removed,
            consent_record_removed,
            profile_removed,
        })
    }
}

fn normalise_display_name(value: &str) -> Result<String, ProfileError> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > 32 || value.chars().any(char::is_control) {
        return Err(ProfileError::InvalidDisplayName);
    }
    Ok(value.to_owned())
}

fn normalise_bio(value: &str) -> Result<Option<String>, ProfileError> {
    let value = value.trim();
    if value.chars().count() > 160 || value.chars().any(char::is_control) {
        return Err(ProfileError::InvalidBio);
    }
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::{ProfileError, normalise_bio, normalise_display_name};

    #[test]
    fn display_name_requires_one_to_thirty_two_visible_characters() {
        assert_eq!(
            normalise_display_name("Moon Explorer").unwrap(),
            "Moon Explorer"
        );
        assert!(matches!(
            normalise_display_name("  "),
            Err(ProfileError::InvalidDisplayName)
        ));
        assert!(matches!(
            normalise_display_name(&"x".repeat(33)),
            Err(ProfileError::InvalidDisplayName)
        ));
        assert!(matches!(
            normalise_display_name("Moon\nByte"),
            Err(ProfileError::InvalidDisplayName)
        ));
    }

    #[test]
    fn bio_is_optional_and_bounded() {
        assert_eq!(normalise_bio(" ").unwrap(), None);
        assert_eq!(
            normalise_bio("Finding tiny worlds").unwrap().as_deref(),
            Some("Finding tiny worlds")
        );
        assert!(matches!(
            normalise_bio(&"x".repeat(161)),
            Err(ProfileError::InvalidBio)
        ));
        assert!(matches!(
            normalise_bio("line one\nline two"),
            Err(ProfileError::InvalidBio)
        ));
    }
}

#[cfg(test)]
mod postgres_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        atlas::AtlasStore,
        consent::ConsentStore,
        database::Database,
        game::{GameKind, GameRuntime},
    };

    use super::ProfileStore;

    #[tokio::test]
    async fn profile_export_and_deletion_cover_current_player_records() {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must point to the CI PostgreSQL service");
        let database = Database::connect(&database_url)
            .await
            .expect("connect and migrate PostgreSQL");
        let store = ProfileStore::new(database.pool());
        let runtime = GameRuntime::new(database.pool());
        let atlas = AtlasStore::new(database.pool());
        let consents = ConsentStore::new(database.pool());
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
            % 1_000_000_000_000_000_000;
        let player_id = format!("profile-test-{nonce}");

        consents
            .accept(&player_id, Some("guild-consent-test"))
            .await
            .expect("record current policy acceptance");
        assert!(
            consents
                .is_current(&player_id)
                .await
                .expect("read consent state"),
            "the current document versions should be accepted"
        );
        sqlx::query("UPDATE user_consents SET terms_version = '0.9.0' WHERE discord_user_id = $1")
            .bind(&player_id)
            .execute(&database.pool())
            .await
            .expect("simulate a superseded Terms version");
        assert!(
            !consents
                .is_current(&player_id)
                .await
                .expect("check superseded consent"),
            "a previous Terms version must not unlock commands"
        );
        consents
            .accept(&player_id, Some("guild-consent-test"))
            .await
            .expect("accept the updated policy versions");
        assert!(
            consents
                .is_current(&player_id)
                .await
                .expect("check renewed consent")
        );

        let profile = store
            .get_or_create(&player_id, "Moon Explorer")
            .await
            .expect("create profile");
        assert_eq!(profile.visibility, "private");

        let profile = store
            .update_identity(&player_id, "Moonbyte Pilot", "Finding small worlds")
            .await
            .expect("update identity");
        assert_eq!(profile.display_name, "Moonbyte Pilot");
        assert_eq!(profile.bio.as_deref(), Some("Finding small worlds"));

        let profile = store
            .update_preferences(&player_id, "public", false, true)
            .await
            .expect("save preferences");
        assert_eq!(profile.visibility, "public");
        assert!(!profile.show_statistics);
        assert!(profile.quiet_mode);

        let session = runtime
            .start(GameKind::MemoryMatch, &player_id, None)
            .await
            .expect("create linked session");
        let completed_session = runtime
            .start(GameKind::WordScramble, &player_id, None)
            .await
            .expect("create result session");
        runtime
            .act(&completed_session.id, &player_id, None, "orbit")
            .await
            .expect("complete result session");
        let stats = store.game_stats(&player_id).await.expect("read stats");
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].games_played, 1);
        assert_eq!(stats[0].games_won, 1);
        let atlas_before = atlas.snapshot(&player_id).await.expect("open player Atlas");
        assert_eq!(atlas_before.current_location.id, "lantern-square");
        let atlas_trip = atlas
            .travel(&player_id, "mosslight-grove")
            .await
            .expect("discover grove");
        assert_eq!(atlas_trip.to.id, "mosslight-grove");

        let inventory = store.inventory(&player_id).await.expect("read inventory");
        assert!(inventory.profile_record);
        assert!(inventory.consent_record);
        assert_eq!(inventory.game_session_records, 2);
        assert_eq!(inventory.game_result_records, 1);
        assert_eq!(inventory.game_stat_rows, 1);
        assert_eq!(inventory.achievement_rows, 2);
        assert!(inventory.wallet_record);
        assert_eq!(inventory.wallet_ledger_records, 1);
        assert!(inventory.world_state_record);
        assert_eq!(inventory.atlas_discovery_records, 2);
        assert_eq!(inventory.wayfarer_active_days, 1);
        assert_eq!(inventory.quest_rotation_records, 1);
        assert_eq!(inventory.daily_quest_records, 3);
        assert_eq!(inventory.rivalry_records, 0);

        let export = store.export_json(&player_id).await.expect("export data");
        assert!(export.contains("Moonbyte Pilot"));
        assert!(export.contains(&session.id));
        assert!(export.contains("completed game results"));
        assert!(export.contains("per-game statistics"));
        assert!(export.contains("format_version"));
        assert!(export.contains("Terms of Service and privacy-policy acceptance record"));
        assert!(export.contains("terms_version"));
        assert!(export.contains("privacy_version"));
        assert!(export.contains("1.0.0"));
        assert!(export.contains("unlocked achievements"));
        assert!(export.contains("first_clear"));
        assert!(export.contains("First Steps"));
        assert!(export.contains("Bright Start"));
        assert!(export.contains("Moonbits wallet balance"));
        assert!(export.contains("wallet_balance"));
        assert!(export.contains("\"wallet_balance\": 10"));
        assert!(export.contains("first_game_completion"));
        assert!(export.contains("current Atlas location"));
        assert!(export.contains("atlas_discoveries"));
        assert!(export.contains("Chronicle archive fragments and first-discovery timestamps"));
        assert!(export.contains("archive_fragments"));
        assert!(export.contains("lantern_first_light"));
        assert!(export.contains("mosslight-grove"));
        assert!(export.contains("Wayfarer activity days"));
        assert!(export.contains("daily quest selections and progress"));
        assert!(export.contains("SHA-512 daily quest rotation fingerprints"));
        assert!(export.contains("mutual rivalry invitations and match records"));
        assert!(export.contains("wayfarer_activity_days"));
        assert!(export.contains("daily_quests"));
        assert!(export.contains("quest_rotation_fingerprints"));
        assert!(
            !export.contains("rotation_seed"),
            "private quest seed must not be exported"
        );
        assert!(export.contains("\"format_version\": 9"));
        assert!(export.contains(&completed_session.id));

        let deletion = store
            .delete_account(&player_id)
            .await
            .expect("delete player records");
        assert_eq!(deletion.sessions_removed, 2);
        assert_eq!(deletion.results_removed, 1);
        assert_eq!(deletion.stat_rows_removed, 1);
        assert_eq!(deletion.achievement_rows_removed, 2);
        assert_eq!(deletion.wallet_ledger_records_removed, 1);
        assert!(deletion.wallet_removed);
        assert!(deletion.world_state_removed);
        assert_eq!(deletion.atlas_discoveries_removed, 2);
        assert_eq!(deletion.archive_fragments_removed, 2);
        assert_eq!(deletion.wayfarer_days_removed, 1);
        assert_eq!(deletion.quest_rotations_removed, 1);
        assert_eq!(deletion.daily_quest_records_removed, 3);
        assert_eq!(deletion.rivalry_records_removed, 0);
        assert!(deletion.consent_record_removed);
        assert!(deletion.profile_removed);
        assert!(
            store
                .get(&player_id)
                .await
                .expect("read deleted profile")
                .is_none()
        );
        let deleted_inventory = store
            .inventory(&player_id)
            .await
            .expect("read deleted inventory");
        assert_eq!(deleted_inventory.game_session_records, 0);
        assert_eq!(deleted_inventory.achievement_rows, 0);
        assert!(!deleted_inventory.wallet_record);
        assert_eq!(deleted_inventory.wallet_ledger_records, 0);
        assert!(!deleted_inventory.world_state_record);
        assert_eq!(deleted_inventory.atlas_discovery_records, 0);
        assert_eq!(deleted_inventory.archive_fragment_records, 0);
        assert_eq!(deleted_inventory.wayfarer_active_days, 0);
        assert_eq!(deleted_inventory.quest_rotation_records, 0);
        assert_eq!(deleted_inventory.daily_quest_records, 0);
        assert_eq!(deleted_inventory.rivalry_records, 0);
        assert!(!deleted_inventory.consent_record);
        assert!(
            !consents
                .is_current(&player_id)
                .await
                .expect("read deleted consent")
        );
        assert!(
            runtime.get(&session.id, &player_id, None).await.is_err(),
            "deleted session state must no longer be recoverable"
        );
    }
}
