-- This file should undo anything in `up.sql`
ALTER TABLE chatters ALTER COLUMN points TYPE INTEGER;

ALTER TABLE duels ALTER COLUMN points TYPE integer;

ALTER TABLE losers_pool ALTER COLUMN amount TYPE INTEGER;
