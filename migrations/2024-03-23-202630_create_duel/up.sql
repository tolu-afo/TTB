-- Your SQL goes here
CREATE TABLE duels (
    id SERIAL PRIMARY KEY,
    accepted BOOLEAN NOT NULL DEFAULT false,
    points INTEGER NOT NULL,
    challenger VARCHAR NOT NULL,
    challenged VARCHAR NOT NULL,
    winner VARCHAR
)