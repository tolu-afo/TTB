-- Your SQL goes here
CREATE TABLE lurkers (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    twitch_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);