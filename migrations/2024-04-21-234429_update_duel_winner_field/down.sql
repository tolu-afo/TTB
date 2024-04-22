-- This file should undo anything in `up.sql`

ALTER TABLE duels
    ADD COLUMN winner_new varchar;

UPDATE duels
SET winner_new=CAST(winner AS varchar);

ALTER TABLE duels
    DROP COLUMN winner;

ALTER TABLE duels
    RENAME COLUMN winner_new TO winner;