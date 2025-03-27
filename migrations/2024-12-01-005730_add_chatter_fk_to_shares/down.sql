-- This file should undo anything in `up.sql`
ALTER TABLE shares DROP CONSTRAINT fk_shares_chatter_id;
