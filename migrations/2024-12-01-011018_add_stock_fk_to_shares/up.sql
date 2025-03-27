-- Your SQL goes here
ALTER TABLE orders
ADD CONSTRAINT orders_stock_id_fkey FOREIGN KEY (stock_id) REFERENCES stocks (id);
