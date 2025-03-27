-- This file should undo anything in `up.sql`
ALTER TABLE orders DROP CONSTRAINT fk_orders_chatter_id;
