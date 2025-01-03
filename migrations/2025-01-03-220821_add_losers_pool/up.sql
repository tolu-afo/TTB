-- Your SQL goes here
CREATE TABLE losers_pool (
  id SERIAL PRIMARY KEY,
  amount INTEGER NOT NULL DEFAULT 0,
  winner INTEGER,
  created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

CREATE TRIGGER update_losers_pool_modtime BEFORE
UPDATE ON losers_pool FOR EACH ROW
EXECUTE FUNCTION update_modified_column ();
