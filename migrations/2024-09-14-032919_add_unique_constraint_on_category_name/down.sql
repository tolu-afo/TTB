-- This file should undo anything in `up.sql`
ALTER TABLE categories DROP CONSTRAINT categories_name_unique;
