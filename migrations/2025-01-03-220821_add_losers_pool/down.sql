-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_losers_pool_modtime on losers_pool;

DROP TABLE IF EXISTS losers_pool;
