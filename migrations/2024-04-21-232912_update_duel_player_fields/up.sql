-- Your SQL goes here
ALTER TABLE duels
    ADD COLUMN challenger_new integer;

UPDATE duels
SET challenger_new=CAST(challenger AS integer);

ALTER TABLE duels
    DROP COLUMN challenger;

ALTER TABLE duels
    RENAME COLUMN challenger_new TO challenger;

ALTER TABLE duels
    ALTER COLUMN challenger SET NOT NULL;

-- Updating challenged column

ALTER TABLE duels
    ADD COLUMN challenged_new integer;

UPDATE duels
SET challenged_new=CAST(challenged AS integer);

ALTER TABLE duels
    DROP COLUMN challenged;

ALTER TABLE duels
    RENAME COLUMN challenged_new TO challenged;

ALTER TABLE duels
    ALTER COLUMN challenged SET NOT NULL;