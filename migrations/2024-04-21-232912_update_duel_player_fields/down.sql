-- This file should undo anything in `up.sql`
ALTER TABLE duels
    ADD COLUMN challenger_new varchar;

UPDATE duels
SET challenger_new=CAST(challenger AS varchar);

ALTER TABLE duels
    DROP COLUMN challenger;

ALTER TABLE duels
    RENAME COLUMN challenger_new TO challenger;

ALTER TABLE duels
    ALTER COLUMN challenger SET NOT NULL;

-- Undoing Challenged migration

ALTER TABLE duels
    ADD COLUMN challenged_new varchar;

UPDATE duels
SET challenged_new=CAST(challenged AS varchar);

ALTER TABLE duels
    DROP COLUMN challenged;

ALTER TABLE duels
    RENAME COLUMN challenged_new TO challenged;

ALTER TABLE duels
    ALTER COLUMN challenged SET NOT NULL;