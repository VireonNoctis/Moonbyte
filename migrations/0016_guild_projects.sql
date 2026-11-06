-- One fixed, optional cooperative project per Discord server.
-- Membership is personal and contribution rows are anonymous guild aggregates.
CREATE TABLE IF NOT EXISTS guild_projects (
    guild_project_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    guild_id TEXT NOT NULL,
    project_key TEXT NOT NULL CHECK (project_key = 'lantern_road_survey'),
    project_name TEXT NOT NULL CHECK (project_name = 'Lantern Road Survey'),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed')),
    contribution_count BIGINT NOT NULL DEFAULT 0 CHECK (contribution_count >= 0),
    contribution_goal BIGINT NOT NULL DEFAULT 2 CHECK (contribution_goal = 2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ,
    UNIQUE (guild_id, project_key),
    CHECK (contribution_count <= contribution_goal),
    CHECK ((status = 'completed') = (completed_at IS NOT NULL))
);

CREATE TABLE IF NOT EXISTS guild_project_members (
    guild_project_id BIGINT NOT NULL REFERENCES guild_projects(guild_project_id) ON DELETE CASCADE,
    discord_user_id TEXT NOT NULL REFERENCES player_profiles(discord_user_id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (guild_project_id, discord_user_id)
);

CREATE INDEX IF NOT EXISTS guild_project_members_player_idx
    ON guild_project_members (discord_user_id, guild_project_id);

CREATE TABLE IF NOT EXISTS guild_project_contributions (
    guild_project_id BIGINT NOT NULL REFERENCES guild_projects(guild_project_id) ON DELETE CASCADE,
    expedition_run_id BIGINT NOT NULL,
    checkpoint_id TEXT NOT NULL CHECK (
        checkpoint_id IN ('mosslight-grove', 'ember-archive', 'starfall-observatory')
    ),
    contributed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (guild_project_id, expedition_run_id, checkpoint_id)
);

COMMENT ON COLUMN guild_project_contributions.expedition_run_id IS
    'Opaque source locator without a foreign key and retains anonymous guild progress after a source Expedition is deleted.';
