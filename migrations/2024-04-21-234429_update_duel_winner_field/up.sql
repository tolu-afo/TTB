-- Your SQL goes here

ALTER TABLE duels
    ADD COLUMN winner_new integer;

UPDATE duels
SET winner_new=CAST(winner AS integer);

ALTER TABLE duels
    DROP COLUMN winner;

ALTER TABLE duels
    RENAME COLUMN winner_new TO winner;
