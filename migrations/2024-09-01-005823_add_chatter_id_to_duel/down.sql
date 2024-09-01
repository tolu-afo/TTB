-- This file should undo anything in `up.sql`
ALTER TABLE duels DROP COLUMN IF EXISTS challenger_id;

ALTER TABLE duels DROP COLUMN IF EXISTS challenged_id;

ALTER TABLE duels DROP COLUMN IF EXISTS challenger_guesses;

ALTER TABLE duels DROP COLUMN IF EXISTS challenged_guesses;
