//! Earned-only Moonbits wallet and atomic milestone credits.
//!
//! The ledger is the audit source. Wallet balances are a transactional cache,
//! never an independent source of currency creation.

use sqlx::{PgPool, Postgres, Transaction};

/// Fixed, non-random reward for a player's first completion of each game type.
pub const FIRST_COMPLETION_REWARD: i64 = 10;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct WalletSnapshot {
    pub balance: i64,
    pub ledger_entries: i64,
}

#[derive(Clone)]
pub struct WalletStore {
    pool: PgPool,
}

impl WalletStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn snapshot(&self, player_id: &str) -> Result<WalletSnapshot, sqlx::Error> {
        let (balance, ledger_entries) = sqlx::query_as::<_, (i64, i64)>(
            "SELECT \
                COALESCE((SELECT balance FROM player_wallets WHERE discord_user_id = $1), 0), \
                (SELECT count(*) FROM wallet_ledger_entries WHERE discord_user_id = $1)",
        )
        .bind(player_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(WalletSnapshot {
            balance,
            ledger_entries,
        })
    }
}

/// Credit the first-completion milestone for one player/game pair.
///
/// The unique idempotency key makes the reward one-time per player and game
/// type, even when the player completes more sessions of that game later.
/// This function must run inside the same transaction as the completed game
/// result and aggregate-stat writes.
pub(crate) async fn award_first_game_completion(
    transaction: &mut Transaction<'_, Postgres>,
    player_id: &str,
    game_id: &str,
    session_id: &str,
) -> Result<bool, sqlx::Error> {
    let idempotency_key = format!("first-game-completion:{game_id}");

    sqlx::query(
        "INSERT INTO player_wallets (discord_user_id) VALUES ($1) \
         ON CONFLICT (discord_user_id) DO NOTHING",
    )
    .bind(player_id)
    .execute(&mut **transaction)
    .await?;

    let entry_id: Option<i64> = sqlx::query_scalar(
        "INSERT INTO wallet_ledger_entries \
         (discord_user_id, amount_delta, reason, source_id, idempotency_key) \
         VALUES ($1, $2, 'first_game_completion', $3, $4) \
         ON CONFLICT (discord_user_id, idempotency_key) DO NOTHING \
         RETURNING entry_id",
    )
    .bind(player_id)
    .bind(FIRST_COMPLETION_REWARD)
    .bind(session_id)
    .bind(idempotency_key)
    .fetch_optional(&mut **transaction)
    .await?;

    let Some(_entry_id) = entry_id else {
        return Ok(false);
    };

    sqlx::query(
        "UPDATE player_wallets \
         SET balance = balance + $2, version = version + 1, updated_at = now() \
         WHERE discord_user_id = $1",
    )
    .bind(player_id)
    .bind(FIRST_COMPLETION_REWARD)
    .execute(&mut **transaction)
    .await?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::FIRST_COMPLETION_REWARD;

    #[test]
    fn first_completion_reward_is_fixed_and_non_negative() {
        assert_eq!(FIRST_COMPLETION_REWARD, 10);
    }
}
