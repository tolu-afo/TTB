-- Your SQL goes here
ALTER TABLE duels ADD COLUMN challenger_id VARCHAR(255);

ALTER TABLE duels ADD COLUMN challenged_id VARCHAR(255);

ALTER TABLE duels
ADD COLUMN challenger_guesses INTEGER NOT NULL default 5;

ALTER TABLE duels
ADD COLUMN challenged_guesses INTEGER NOT NULL default 5;
