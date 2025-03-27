-- This file should undo anything in `up.sql`
ALTER TABLE orders DROP CONSTRAINT orders_stock_id_fkey;
