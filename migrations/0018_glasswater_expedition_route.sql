ALTER TABLE expedition_runs
    DROP CONSTRAINT IF EXISTS expedition_runs_route_id_check;
ALTER TABLE expedition_runs
    ADD CONSTRAINT expedition_runs_route_id_check
    CHECK (route_id IN ('lantern-road', 'glasswater-crossing'));

ALTER TABLE expedition_checkpoint_history
    DROP CONSTRAINT IF EXISTS expedition_checkpoint_history_checkpoint_id_check;
ALTER TABLE expedition_checkpoint_history
    ADD CONSTRAINT expedition_checkpoint_history_checkpoint_id_check
    CHECK (
        checkpoint_id IN (
            'mosslight-grove',
            'ember-archive',
            'starfall-observatory',
            'glasswater-pier',
            'glasswater-star-chart',
            'mosslight-return'
        )
    );

ALTER TABLE expedition_checkpoint_history
    DROP CONSTRAINT IF EXISTS expedition_checkpoint_history_choice_id_check;
ALTER TABLE expedition_checkpoint_history
    ADD CONSTRAINT expedition_checkpoint_history_choice_id_check
    CHECK (
        choice_id IN (
            'listen',
            'inspect',
            'star',
            'lantern',
            'chart',
            'survey',
            'east',
            'west',
            'record'
        )
    );
