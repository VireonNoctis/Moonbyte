//! The solo Mosslight Resonance Puzzle Vault.
//!
//! Room content and accepted answers are fixed, reviewed source data. Player guesses are
//! validated in memory and are never persisted; only solved room IDs and timestamps are saved.

use std::fmt;

use sqlx::{PgPool, Postgres, Transaction};

pub const BLUEPRINT_ID: &str = "mosslight-resonance";
pub const BLUEPRINT_VERSION: i16 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaultRoom {
    pub id: &'static str,
    pub title: &'static str,
    pub clue: &'static str,
    pub hint: &'static str,
    accepted_answers: &'static [&'static str],
    pub explanation: &'static str,
}

pub const ROOMS: [VaultRoom; 3] = [
    VaultRoom {
        id: "waystone-order",
        title: "Waystone Order",
        clue: "Set Fern, Star and Moon in order. Fern must come before Star; Star must come before Moon. Submit the three names joined by hyphens.",
        hint: "Start with Fern. The second rule fixes Star next, and the final rule places Moon last.",
        accepted_answers: &["fern-star-moon"],
        explanation: "The clues form one unbroken order: Fern → Star → Moon.",
    },
    VaultRoom {
        id: "tideglass-code",
        title: "Tideglass Code",
        clue: "Find a three-digit code using distinct digits from 1 through 5. The first digit is twice the second, the third is one more than the second, and all three digits sum to nine.",
        hint: "Choose the second digit first. Then apply the doubling and +1 rules before checking the sum.",
        accepted_answers: &["423"],
        explanation: "423 is the only fit: 4 is twice 2, 3 is one more than 2, and 4 + 2 + 3 = 9.",
    },
    VaultRoom {
        id: "keepers-seal",
        title: "Keeper's Seal",
        clue: "The seal remembers the order you set at the first waystone, then the three digits measured at Tideglass. Join those two answers with a hyphen.",
        hint: "Use the exact Waystone Order answer first, then append the Tideglass Code after one hyphen.",
        accepted_answers: &["fern-star-moon-423"],
        explanation: "The seal combines both saved solutions in order: FERN-STAR-MOON-423.",
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultSnapshot {
    pub run_id: i64,
    pub blueprint_id: String,
    pub content_version: i16,
    pub state: String,
    pub current_room_index: i16,
    pub row_version: i64,
    pub completed_room_ids: Vec<String>,
    pub current_room: Option<VaultRoom>,
    pub completed_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnterResult {
    pub snapshot: VaultSnapshot,
    pub created: bool,
}

#[derive(Debug)]
pub enum VaultError {
    Database(sqlx::Error),
    NotDiscovered,
    UnknownBlueprint,
    NoActiveRun,
    RunNotFound,
    NotActive,
    NotPaused,
    WrongRoom,
    IncorrectAnswer,
    InvalidState,
}

impl fmt::Display for VaultError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(_) => formatter.write_str("The Puzzle Vault is temporarily unavailable."),
            Self::NotDiscovered => formatter.write_str("Discover Mosslight Grove in the Atlas before entering this Vault."),
            Self::UnknownBlueprint => formatter.write_str("That Vault blueprint is not available."),
            Self::NoActiveRun => formatter.write_str("You do not have an open Mosslight Resonance run. Use /vault list to begin."),
            Self::RunNotFound => formatter.write_str("That Vault run could not be found for your account."),
            Self::NotActive => formatter.write_str("This run is not active. Use /vault status to check it."),
            Self::NotPaused => formatter.write_str("This run is not paused or left."),
            Self::WrongRoom => formatter.write_str("That is not the current chamber. Check /vault status for the saved chamber ID."),
            Self::IncorrectAnswer => formatter.write_str("That answer does not match this chamber. Your save stone is unchanged; try another answer."),
            Self::InvalidState => formatter.write_str("This Vault record is not in a valid state. Your saved progress has not been changed."),
        }
    }
}

impl std::error::Error for VaultError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for VaultError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Clone)]
pub struct VaultStore {
    pool: PgPool,
}

impl VaultStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn is_unlocked(&self, player_id: &str) -> Result<bool, VaultError> {
        Ok(sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM player_atlas_discoveries
                WHERE discord_user_id = $1 AND location_id = 'mosslight-grove'
            )",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?)
    }

    /// Start a run after explicit command confirmation, or return the existing open run.
    /// Completed/left runs remain in history; a practice replay creates a new run.
    pub async fn enter(&self, player_id: &str) -> Result<EnterResult, VaultError> {
        let mut transaction = self.pool.begin().await?;
        lock_player(&mut transaction, player_id).await?;

        let discovered: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM player_atlas_discoveries
                WHERE discord_user_id = $1 AND location_id = 'mosslight-grove'
            )",
        )
        .bind(player_id)
        .fetch_one(&mut *transaction)
        .await?;
        if !discovered {
            transaction.rollback().await?;
            return Err(VaultError::NotDiscovered);
        }

        let existing: Option<i64> = sqlx::query_scalar(
            "SELECT vault_run_id FROM player_vault_runs
             WHERE discord_user_id = $1 AND blueprint_id = $2
               AND content_version = $3 AND state IN ('active', 'paused', 'left')
             ORDER BY created_at DESC, vault_run_id DESC
             LIMIT 1",
        )
        .bind(player_id)
        .bind(BLUEPRINT_ID)
        .bind(BLUEPRINT_VERSION)
        .fetch_optional(&mut *transaction)
        .await?;

        let (run_id, created) = match existing {
            Some(run_id) => (run_id, false),
            None => {
                let run_id: i64 = sqlx::query_scalar(
                    "INSERT INTO player_vault_runs
                     (discord_user_id, blueprint_id, content_version, state)
                     VALUES ($1, $2, $3, 'active')
                     RETURNING vault_run_id",
                )
                .bind(player_id)
                .bind(BLUEPRINT_ID)
                .bind(BLUEPRINT_VERSION)
                .fetch_one(&mut *transaction)
                .await?;
                (run_id, true)
            }
        };

        transaction.commit().await?;
        Ok(EnterResult {
            snapshot: self.snapshot(player_id, run_id).await?,
            created,
        })
    }

    pub async fn latest_snapshot(
        &self,
        player_id: &str,
    ) -> Result<Option<VaultSnapshot>, VaultError> {
        let run_id: Option<i64> = sqlx::query_scalar(
            "SELECT vault_run_id FROM player_vault_runs
             WHERE discord_user_id = $1 AND blueprint_id = $2
             ORDER BY created_at DESC, vault_run_id DESC LIMIT 1",
        )
        .bind(player_id)
        .bind(BLUEPRINT_ID)
        .fetch_optional(&self.pool)
        .await?;
        match run_id {
            Some(run_id) => Ok(Some(self.snapshot(player_id, run_id).await?)),
            None => Ok(None),
        }
    }

    pub async fn active_snapshot(
        &self,
        player_id: &str,
    ) -> Result<Option<VaultSnapshot>, VaultError> {
        let run_id: Option<i64> = sqlx::query_scalar(
            "SELECT vault_run_id FROM player_vault_runs
             WHERE discord_user_id = $1 AND blueprint_id = $2
               AND state IN ('active', 'paused', 'left')
             ORDER BY created_at DESC, vault_run_id DESC LIMIT 1",
        )
        .bind(player_id)
        .bind(BLUEPRINT_ID)
        .fetch_optional(&self.pool)
        .await?;
        match run_id {
            Some(run_id) => Ok(Some(self.snapshot(player_id, run_id).await?)),
            None => Ok(None),
        }
    }

    pub async fn snapshot(
        &self,
        player_id: &str,
        run_id: i64,
    ) -> Result<VaultSnapshot, VaultError> {
        // Read the run and its save stones from one database snapshot. Without this,
        // a concurrent solve could advance the run between the two SELECT statements.
        let mut transaction = self.pool.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
            .execute(&mut *transaction)
            .await?;
        let row = sqlx::query_as::<_, (String, i16, String, i16, i64, Option<String>)>(
            "SELECT blueprint_id, content_version, state, current_room_index,
                    row_version, completed_at::text
             FROM player_vault_runs
             WHERE vault_run_id = $1 AND discord_user_id = $2",
        )
        .bind(run_id)
        .bind(player_id)
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(VaultError::RunNotFound)?;

        let (blueprint_id, content_version, state, current_room_index, row_version, completed_at) =
            row;
        if blueprint_id != BLUEPRINT_ID || content_version != BLUEPRINT_VERSION {
            return Err(VaultError::UnknownBlueprint);
        }
        let completed_room_ids: Vec<String> = sqlx::query_scalar(
            "SELECT room_id FROM player_vault_checkpoints
             WHERE vault_run_id = $1 ORDER BY room_index ASC",
        )
        .bind(run_id)
        .fetch_all(&mut *transaction)
        .await?;
        transaction.commit().await?;

        let current_room = usize::try_from(current_room_index)
            .ok()
            .and_then(|index| ROOMS.get(index).copied());

        Ok(VaultSnapshot {
            run_id,
            blueprint_id,
            content_version,
            state,
            current_room_index,
            row_version,
            completed_room_ids,
            current_room,
            completed_at,
        })
    }

    pub async fn solve(
        &self,
        player_id: &str,
        room_id: &str,
        answer: &str,
    ) -> Result<VaultSnapshot, VaultError> {
        let mut transaction = self.pool.begin().await?;
        lock_player(&mut transaction, player_id).await?;
        let (run_id, state, current_room_index) = lock_active_run(&mut transaction, player_id).await?;

        if state != "active" {
            return Err(VaultError::NotActive);
        }
        let room = usize::try_from(current_room_index)
            .ok()
            .and_then(|index| ROOMS.get(index))
            .copied()
            .ok_or(VaultError::InvalidState)?;
        if room.id != room_id {
            return Err(VaultError::WrongRoom);
        }
        if !room.accepted_answers.iter().any(|accepted| answers_match(answer, accepted)) {
            return Err(VaultError::IncorrectAnswer);
        }

        sqlx::query(
            "INSERT INTO player_vault_checkpoints (vault_run_id, room_id, room_index)
             VALUES ($1, $2, $3)",
        )
        .bind(run_id)
        .bind(room.id)
        .bind(current_room_index)
        .execute(&mut *transaction)
        .await?;

        let next_index = current_room_index + 1;
        let completed = usize::try_from(next_index).ok() == Some(ROOMS.len());
        sqlx::query(
            "UPDATE player_vault_runs
             SET current_room_index = $2,
                 state = CASE WHEN $3 THEN 'completed' ELSE 'active' END,
                 completed_at = CASE WHEN $3 THEN now() ELSE NULL END,
                 row_version = row_version + 1,
                 updated_at = now()
             WHERE vault_run_id = $1",
        )
        .bind(run_id)
        .bind(next_index)
        .bind(completed)
        .execute(&mut *transaction)
        .await?;

        if completed {
            // These effects commit together with the final save stone. Inventory checks
            // the entire player catalogue before granting the one-time keepsake and
            // records this completing run as the provenance source.
            crate::inventory::record_vault_keepsake(
                &mut transaction,
                player_id,
                run_id,
                BLUEPRINT_VERSION,
            )
            .await?;
            crate::chronicle::record_vault_completion(&mut transaction, player_id).await?;
            crate::wayfarer::record_vault_completion(&mut transaction, player_id).await?;
        }

        transaction.commit().await?;
        self.snapshot(player_id, run_id).await
    }

    pub async fn pause(&self, player_id: &str) -> Result<VaultSnapshot, VaultError> {
        self.transition(player_id, "active", "paused").await
    }

    pub async fn resume(&self, player_id: &str) -> Result<VaultSnapshot, VaultError> {
        self.transition(player_id, "resumable", "active").await
    }

    /// Leave is non-destructive: checkpoints remain and the run can be resumed later.
    pub async fn leave(&self, player_id: &str) -> Result<VaultSnapshot, VaultError> {
        self.transition(player_id, "open", "left").await
    }

    async fn transition(
        &self,
        player_id: &str,
        expected_state: &str,
        next_state: &str,
    ) -> Result<VaultSnapshot, VaultError> {
        let mut transaction = self.pool.begin().await?;
        lock_player(&mut transaction, player_id).await?;
        let (run_id, state, _) = lock_active_run(&mut transaction, player_id).await?;
        let allowed = match expected_state {
            "open" => state == "active" || state == "paused",
            "resumable" => state == "paused" || state == "left",
            _ => state == expected_state,
        };
        if !allowed {
            return Err(match expected_state {
                "active" => VaultError::NotActive,
                "paused" => VaultError::NotPaused,
                "resumable" => VaultError::NotPaused,
                _ => VaultError::NoActiveRun,
            });
        }

        sqlx::query(
            "UPDATE player_vault_runs
             SET state = $2, row_version = row_version + 1, updated_at = now()
             WHERE vault_run_id = $1",
        )
        .bind(run_id)
        .bind(next_state)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        self.snapshot(player_id, run_id).await
    }
}

async fn lock_player(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('moonbyte:user:' || $1, 0))")
        .bind(player_id)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

async fn lock_active_run(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
) -> Result<(i64, String, i16), VaultError> {
    sqlx::query_as::<_, (i64, String, i16)>(
        "SELECT vault_run_id, state, current_room_index
         FROM player_vault_runs
         WHERE discord_user_id = $1 AND blueprint_id = $2
           AND state IN ('active', 'paused', 'left')
         ORDER BY created_at DESC, vault_run_id DESC
         LIMIT 1 FOR UPDATE",
    )
    .bind(player_id)
    .bind(BLUEPRINT_ID)
    .fetch_optional(&mut **transaction)
    .await?
    .ok_or(VaultError::NoActiveRun)
}

fn answers_match(input: &str, accepted: &str) -> bool {
    normalise_answer(input) == normalise_answer(accepted)
}

fn normalise_answer(input: &str) -> String {
    input
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect()
}

pub fn resolve_blueprint_id(input: &str) -> Option<&'static str> {
    let input = input.trim();
    (BLUEPRINT_ID.eq_ignore_ascii_case(input)
        || "Mosslight Resonance".eq_ignore_ascii_case(input))
    .then_some(BLUEPRINT_ID)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{BLUEPRINT_ID, BLUEPRINT_VERSION, ROOMS, answers_match, resolve_blueprint_id};

    #[test]
    fn authored_blueprint_has_three_unique_complete_chambers() {
        assert_eq!(ROOMS.len(), 3);
        assert_eq!(BLUEPRINT_VERSION, 1);
        assert_eq!(resolve_blueprint_id("Mosslight Resonance"), Some(BLUEPRINT_ID));
        assert_eq!(resolve_blueprint_id("unknown"), None);

        let ids = ROOMS.map(|room| room.id);
        let unique = ids.into_iter().collect::<HashSet<_>>();
        assert_eq!(unique.len(), ROOMS.len());
        for room in ROOMS {
            assert!(!room.title.is_empty());
            assert!(!room.clue.is_empty());
            assert!(!room.accepted_answers.is_empty());
            assert!(!room.explanation.is_empty());
        }
    }

    #[test]
    fn tideglass_code_has_exactly_one_solution_in_the_published_digit_range() {
        let mut candidates = Vec::new();
        for first in 1..=5 {
            for second in 1..=5 {
                for third in 1..=5 {
                    if first != second
                        && first != third
                        && second != third
                        && first == second * 2
                        && third == second + 1
                        && first + second + third == 9
                    {
                        candidates.push(first * 100 + second * 10 + third);
                    }
                }
            }
        }
        assert_eq!(candidates, vec![423]);
        assert!(answers_match("423", "423"));
        assert!(!answers_match("432", "423"));
    }

    #[test]
    fn combined_seal_matches_the_two_saved_answers_in_order() {
        assert!(answers_match("FERN → STAR → MOON-423", "fern-star-moon-423"));
        assert!(!answers_match("423-fern-star-moon", "fern-star-moon-423"));
    }

    #[test]
    fn accepted_answers_normalise_case_spacing_and_punctuation_without_fuzzy_matching() {
        assert!(answers_match(" Fern Star Moon ", "fern-star-moon"));
        assert!(answers_match("4 2 3", "423"));
        assert!(answers_match("FERN-STAR-MOON-423", "fern-star-moon-423"));
        assert!(!answers_match("fer-star-moon", "fern-star-moon"));
        assert!(!answers_match("424", "423"));
    }
}

#[cfg(test)]
mod postgres_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        atlas::AtlasStore,
        chronicle::ChronicleStore,
        database::Database,
        inventory::InventoryStore,
        profile::ProfileStore,
        wayfarer::WayfarerStore,
    };

    use super::{VaultError, VaultStore};

    #[tokio::test]
    async fn mosslight_resonance_is_private_resumable_and_rewards_once_across_replays() {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must point to the CI PostgreSQL service");
        let database = Database::connect(&database_url)
            .await
            .expect("connect and migrate PostgreSQL");
        let pool = database.pool();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let player_id = format!("vault-{nonce:x}");
        let profiles = ProfileStore::new(pool.clone());
        let atlas = AtlasStore::new(pool.clone());
        let store = VaultStore::new(pool.clone());

        profiles
            .get_or_create(&player_id, "Vault Test")
            .await
            .expect("create player profile");
        atlas.snapshot(&player_id).await.expect("create starting Atlas state");

        assert!(matches!(store.enter(&player_id).await, Err(VaultError::NotDiscovered)));
        atlas
            .travel(&player_id, "mosslight-grove")
            .await
            .expect("discover the required Atlas location");

        let entry = store.enter(&player_id).await.expect("enter the Vault");
        assert!(entry.created);
        assert_eq!(entry.snapshot.current_room_index, 0);
        assert_eq!(
            entry.snapshot.current_room.expect("first room").id,
            "waystone-order"
        );
        let existing = store.enter(&player_id).await.expect("return to open run");
        assert!(!existing.created);
        assert_eq!(existing.snapshot.run_id, entry.snapshot.run_id);

        assert!(matches!(
            store
                .solve(&player_id, "waystone-order", "not the answer")
                .await,
            Err(VaultError::IncorrectAnswer)
        ));
        let stored_wrong_guess_count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM player_vault_checkpoints WHERE vault_run_id = $1",
        )
        .bind(entry.snapshot.run_id)
        .fetch_one(&pool)
        .await
        .expect("inspect saved checkpoints");
        assert_eq!(stored_wrong_guess_count, 0);

        // Simultaneous submissions for one save stone must commit exactly once.
        let (first, second) = tokio::join!(
            store.solve(&player_id, "waystone-order", "Fern → Star → Moon"),
            store.solve(&player_id, "waystone-order", "fern-star-moon"),
        );
        let first_snapshot = match (first, second) {
            (Ok(snapshot), Err(VaultError::WrongRoom))
            | (Err(VaultError::WrongRoom), Ok(snapshot)) => snapshot,
            (left, right) => panic!(
                "exactly one concurrent solve should advance the Vault; left={left:?}, right={right:?}"
            ),
        };
        assert_eq!(first_snapshot.current_room_index, 1);

        // A newly constructed service sees the same durable checkpoint after recovery.
        let recovered_store = VaultStore::new(pool.clone());
        let recovered = recovered_store
            .active_snapshot(&player_id)
            .await
            .expect("load the persisted run after service reconstruction")
            .expect("active run survives service reconstruction");
        assert_eq!(recovered.current_room_index as usize, recovered.completed_room_ids.len());
        assert_eq!(recovered.current_room.expect("recovered room").id, "tideglass-code");

        let other_player = format!("{player_id}-other");
        assert!(store.latest_snapshot(&other_player).await.expect("other player's history").is_none());
        assert!(matches!(
            store.snapshot(&other_player, entry.snapshot.run_id).await,
            Err(VaultError::RunNotFound)
        ));
        assert!(matches!(
            store.solve(&other_player, "waystone-order", "fern-star-moon").await,
            Err(VaultError::NoActiveRun)
        ));

        let left = store.leave(&player_id).await.expect("leave without deleting the save stone");
        assert_eq!(left.state, "left");
        let returned = store.enter(&player_id).await.expect("return to the left run");
        assert!(!returned.created);
        assert_eq!(returned.snapshot.run_id, entry.snapshot.run_id);
        assert_eq!(returned.snapshot.completed_room_ids.len(), 1);
        let resumed = store.resume(&player_id).await.expect("resume the left run");
        assert_eq!(resumed.state, "active");

        let paused = store.pause(&player_id).await.expect("pause the run");
        assert_eq!(paused.state, "paused");
        let resumed = store.resume(&player_id).await.expect("resume saved progress");
        assert_eq!(resumed.state, "active");
        assert_eq!(
            resumed.current_room.expect("second room").id,
            "tideglass-code"
        );

        store
            .solve(&player_id, "tideglass-code", "423")
            .await
            .expect("solve the second chamber");
        let completed = store
            .solve(&player_id, "keepers-seal", "FERN-STAR-MOON-423")
            .await
            .expect("solve the final chamber");
        assert_eq!(completed.state, "completed");
        assert_eq!(completed.completed_room_ids.len(), 3);
        assert!(completed.current_room.is_none());

        let inventory = InventoryStore::new(pool.clone());
        let keepsakes = inventory.snapshot(&player_id).await.expect("read keepsakes");
        assert_eq!(
            keepsakes
                .entries
                .iter()
                .filter(|item| item.item_id == "keepsake.mosslight-seal")
                .count(),
            1
        );
        let keepsake_source = keepsakes.entries.iter()
            .find(|item| item.item_id == "keepsake.mosslight-seal")
            .expect("keepsake source");
        assert_eq!(
            keepsake_source.source_reference,
            format!("vault-run:{}:mosslight-resonance:v1", completed.run_id)
        );
        let chronicle = ChronicleStore::new(pool.clone())
            .snapshot(&player_id)
            .await
            .expect("read private Chronicle");
        assert!(chronicle.entries.iter().any(|entry| {
            entry.fragment.id == "mosslight_resonance_vault" && entry.unlocked_at.is_some()
        }));
        assert!(
            WayfarerStore::new(pool.clone())
                .snapshot(&player_id)
                .await
                .expect("read Wayfarer progress")
                .active_days
                >= 1
        );

        // Replays are allowed, but the same fixed provenance key prevents a second keepsake.
        let replay = store.enter(&player_id).await.expect("start a practice replay");
        assert!(replay.created);
        store
            .solve(&player_id, "waystone-order", "fern-star-moon")
            .await
            .expect("replay chamber one");
        store
            .solve(&player_id, "tideglass-code", "423")
            .await
            .expect("replay chamber two");
        store
            .solve(&player_id, "keepers-seal", "fern-star-moon-423")
            .await
            .expect("replay chamber three");
        let replay_keepsakes = inventory.snapshot(&player_id).await.expect("read replay reward");
        assert_eq!(
            replay_keepsakes
                .entries
                .iter()
                .filter(|item| item.item_id == "keepsake.mosslight-seal")
                .count(),
            1
        );

        let profiles_inventory = profiles
            .inventory(&player_id)
            .await
            .expect("inspect Vault data inventory");
        assert_eq!(profiles_inventory.vault_run_records, 2);
        assert_eq!(profiles_inventory.vault_checkpoint_records, 6);
        let export = profiles.export_json(&player_id).await.expect("export Vault progress");
        assert!(export.contains("\"format_version\": 11"));
        assert!(export.contains("mosslight-resonance"));
        assert!(export.contains("waystone-order"));

        let deletion = profiles
            .delete_account(&player_id)
            .await
            .expect("delete all Vault progress with the player account");
        assert_eq!(deletion.vault_runs_removed, 2);
        assert_eq!(deletion.vault_checkpoints_removed, 6);
        assert!(
            store
                .latest_snapshot(&player_id)
                .await
                .expect("check deleted Vault data")
                .is_none()
        );
    }
}
