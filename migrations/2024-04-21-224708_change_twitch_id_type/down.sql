-- This file should undo anything in `up.sql`
ALTER TABLE chatters
    ADD COLUMN twitch_id_new varchar UNIQUE;

UPDATE chatters
SET twitch_id_new=CAST(twitch_id AS varchar);

ALTER TABLE chatters
    DROP COLUMN twitch_id;

ALTER TABLE chatters
    RENAME COLUMN twitch_id_new TO twitch_id;

ALTER TABLE chatters
    ALTER COLUMN twitch_id SET NOT NULL;