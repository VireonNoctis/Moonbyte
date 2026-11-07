-- Expand optional daily objectives to cover more of the Wayfarer Thread.
-- Existing daily boards and claimed rewards remain unchanged.
ALTER TABLE player_daily_quests
    DROP CONSTRAINT IF EXISTS player_daily_quests_quest_id_check;

ALTER TABLE player_daily_quests
    ADD CONSTRAINT player_daily_quests_quest_id_check
    CHECK (
        quest_id IN (
            'play_one_game',
            'play_three_games',
            'win_one_game',
            'play_two_game_types',
            'discover_one_place',
            'craft_one_item',
            'complete_one_expedition'
        )
    );
