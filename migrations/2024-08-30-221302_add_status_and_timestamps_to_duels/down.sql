-- This file should undo anything in `up.sql`
ALTER TABLE duels DROP COLUMN status;

ALTER TABLE duels DROP COLUMN created_at;

ALTER TABLE duels DROP COLUMN updated_at;
