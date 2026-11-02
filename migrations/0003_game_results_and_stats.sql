CREATE TABLE IF NOT EXISTS game_results (
    session_id TEXT PRIMARY KEY REFERENCES game_sessions(session_id) ON DELETE CASCADE,
    game_id TEXT NOT NULL,
    outcome TEXT NOT NULL CHECK (outcome IN ('won', 'draw', 'completed')),
    winner_player_id TEXT,
    player_ids JSONB NOT NULL,
    action_count BIGINT NOT NULL CHECK (action_count >= 0),
    completed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (jsonb_typeof(player_ids) = 'array'),
    CHECK ((outcome = 'won') = (winner_player_id IS NOT NULL)),
    CHECK (winner_player_id IS NULL OR player_ids @> jsonb_build_array(winner_player_id))
);

CREATE INDEX IF NOT EXISTS game_results_player_ids_idx
    ON game_results USING GIN (player_ids jsonb_path_ops);

CREATE INDEX IF NOT EXISTS game_sessions_players_idx
    ON game_sessions USING GIN ((session_state -> 'players') jsonb_path_ops);

CREATE TABLE IF NOT EXISTS player_game_stats (
    discord_user_id TEXT NOT NULL CHECK (length(discord_user_id) BETWEEN 1 AND 32),
    game_id TEXT NOT NULL,
    games_played BIGINT NOT NULL DEFAULT 0 CHECK (games_played >= 0),
    games_won BIGINT NOT NULL DEFAULT 0 CHECK (games_won >= 0 AND games_won <= games_played),
    draws BIGINT NOT NULL DEFAULT 0 CHECK (draws >= 0 AND draws <= games_played),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (discord_user_id, game_id)
);
