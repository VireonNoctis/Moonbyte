-- Wayfarer activity, optional daily quests and mutual rivalries.
-- Quest rotation seeds come from PostgreSQL's strong random UUID generator.
-- SHA-512 is used for deterministic selection and a fingerprint, not as entropy.

CREATE TABLE IF NOT EXISTS player_wayfarer_days (
    discord_user_id TEXT NOT NULL
        REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    activity_date DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (discord_user_id, activity_date)
);

CREATE INDEX IF NOT EXISTS player_wayfarer_days_history_idx
    ON player_wayfarer_days (discord_user_id, activity_date DESC);

CREATE TABLE IF NOT EXISTS player_quest_rotations (
    discord_user_id TEXT NOT NULL
        REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    quest_date DATE NOT NULL,
    rotation_seed TEXT NOT NULL CHECK (length(rotation_seed) BETWEEN 1 AND 64),
    commitment_sha512 TEXT NOT NULL CHECK (length(commitment_sha512) = 128),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (discord_user_id, quest_date)
);

CREATE TABLE IF NOT EXISTS player_daily_quests (
    discord_user_id TEXT NOT NULL,
    quest_date DATE NOT NULL,
    quest_id TEXT NOT NULL CHECK (
        quest_id IN (
            'play_one_game',
            'play_three_games',
            'win_one_game',
            'play_two_game_types',
            'discover_one_place'
        )
    ),
    status TEXT NOT NULL DEFAULT 'available'
        CHECK (status IN ('available', 'active', 'completed', 'claimed')),
    progress SMALLINT NOT NULL DEFAULT 0 CHECK (progress >= 0),
    target SMALLINT NOT NULL CHECK (target > 0 AND progress <= target),
    reward_moonbits BIGINT NOT NULL CHECK (reward_moonbits > 0),
    completed_game_ids TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ,
    claimed_at TIMESTAMPTZ,
    PRIMARY KEY (discord_user_id, quest_date, quest_id),
    CONSTRAINT player_daily_quests_rotation_fk
        FOREIGN KEY (discord_user_id, quest_date)
        REFERENCES player_quest_rotations (discord_user_id, quest_date)
        ON DELETE CASCADE,
    CHECK (
        (status IN ('completed', 'claimed') AND progress = target)
        OR status IN ('available', 'active')
    )
);

CREATE INDEX IF NOT EXISTS player_daily_quests_status_idx
    ON player_daily_quests (discord_user_id, quest_date, status);

CREATE TABLE IF NOT EXISTS player_rivalries (
    rivalry_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    player_a_id TEXT NOT NULL
        REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    player_b_id TEXT NOT NULL
        REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    invited_by TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'active', 'declined')),
    matches_played BIGINT NOT NULL DEFAULT 0 CHECK (matches_played >= 0),
    player_a_wins BIGINT NOT NULL DEFAULT 0 CHECK (player_a_wins >= 0),
    player_b_wins BIGINT NOT NULL DEFAULT 0 CHECK (player_b_wins >= 0),
    draws BIGINT NOT NULL DEFAULT 0 CHECK (draws >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (player_a_id < player_b_id),
    CHECK (invited_by IN (player_a_id, player_b_id)),
    CHECK (player_a_wins + player_b_wins + draws <= matches_played),
    UNIQUE (player_a_id, player_b_id)
);

CREATE INDEX IF NOT EXISTS player_rivalries_a_status_idx
    ON player_rivalries (player_a_id, status, updated_at DESC);
CREATE INDEX IF NOT EXISTS player_rivalries_b_status_idx
    ON player_rivalries (player_b_id, status, updated_at DESC);

-- Keep currency origins explicit: only fixed game milestones and fixed quest completions are allowed.
ALTER TABLE wallet_ledger_entries
    DROP CONSTRAINT IF EXISTS wallet_ledger_entries_reason_check;
ALTER TABLE wallet_ledger_entries
    ADD CONSTRAINT wallet_ledger_entries_reason_check
    CHECK (reason IN ('first_game_completion', 'daily_quest_completion'));
