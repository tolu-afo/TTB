-- Your SQL goes here
ALTER TABLE shares
ADD CONSTRAINT fk_shares_chatter_id FOREIGN KEY (owner_id) REFERENCES chatters (id);
