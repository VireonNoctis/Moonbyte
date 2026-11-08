CREATE TABLE IF NOT EXISTS player_wallets (
    discord_user_id TEXT PRIMARY KEY CHECK (length(discord_user_id) BETWEEN 1 AND 32),
    balance BIGINT NOT NULL DEFAULT 0 CHECK (balance >= 0),
    version BIGINT NOT NULL DEFAULT 0 CHECK (version >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS wallet_ledger_entries (
    entry_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    discord_user_id TEXT NOT NULL
        REFERENCES player_wallets(discord_user_id) ON DELETE CASCADE,
    amount_delta BIGINT NOT NULL CHECK (amount_delta <> 0),
    reason TEXT NOT NULL CHECK (reason IN ('first_game_completion')),
    source_id TEXT NOT NULL,
    idempotency_key TEXT NOT NULL
        CHECK (length(idempotency_key) BETWEEN 1 AND 128),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (discord_user_id, idempotency_key)
);

CREATE INDEX IF NOT EXISTS wallet_ledger_player_history_idx
    ON wallet_ledger_entries (discord_user_id, created_at DESC, entry_id DESC);
