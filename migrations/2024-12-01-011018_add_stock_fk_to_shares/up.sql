-- Your SQL goes here
ALTER TABLE shares
ADD CONSTRAINT fk_stock_chatter_id FOREIGN KEY (stock_id) REFERENCES stocks (id);
