CREATE TABLE IF NOT EXISTS achievement_definitions (
    achievement_id TEXT PRIMARY KEY
        CHECK (length(achievement_id) BETWEEN 1 AND 48),
    title VARCHAR(48) NOT NULL,
    description VARCHAR(180) NOT NULL,
    category TEXT NOT NULL DEFAULT 'milestone'
        CHECK (category IN ('milestone', 'mastery', 'collection'))
);

INSERT INTO achievement_definitions (achievement_id, title, description, category)
VALUES
    ('first_clear', 'First Steps', 'Complete your first Moonbyte game.', 'milestone'),
    ('first_victory', 'Bright Start', 'Earn your first game victory.', 'milestone'),
    ('game_collector', 'Game Hopper', 'Complete all four games currently in the library.', 'collection'),
    ('ten_games', 'Table Regular', 'Complete ten games.', 'milestone'),
    ('five_wins', 'Rising Champion', 'Win five games.', 'mastery')
ON CONFLICT (achievement_id) DO NOTHING;

CREATE TABLE IF NOT EXISTS player_achievements (
    discord_user_id TEXT NOT NULL
        CHECK (length(discord_user_id) BETWEEN 1 AND 32),
    achievement_id TEXT NOT NULL
        REFERENCES achievement_definitions(achievement_id) ON DELETE RESTRICT,
    source_session_id TEXT NOT NULL,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (discord_user_id, achievement_id)
);

CREATE INDEX IF NOT EXISTS player_achievements_history_idx
    ON player_achievements (discord_user_id, unlocked_at DESC, achievement_id);
