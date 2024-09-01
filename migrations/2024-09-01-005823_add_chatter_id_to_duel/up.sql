-- Your SQL goes here
ALTER TABLE duels ADD COLUMN challenger_id VARCHAR(255);

ALTER TABLE duels ADD COLUMN challenged_id VARCHAR(255);

ALTER TABLE duels ADD COLUMN challenger_guesses INTEGER default 5;

ALTER TABLE duels ADD COLUMN challenged_guesses INTEGER default 5;
