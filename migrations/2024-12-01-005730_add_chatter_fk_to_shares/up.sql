-- Your SQL goes here
ALTER TABLE orders
ADD CONSTRAINT fk_orders_chatter_id FOREIGN KEY (owner_id) REFERENCES chatters (id);
