-- Your SQL goes here
CREATE TABLE chatters (
   id SERIAL PRIMARY KEY,
   username VARCHAR NOT NULL,
   points INTEGER DEFAULT 0,
   wins INTEGER DEFAULT 0,
   losses INTEGER DEFAULT 0
)