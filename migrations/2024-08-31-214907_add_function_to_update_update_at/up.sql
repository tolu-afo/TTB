CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$
 language 'plpgsql';

CREATE TRIGGER update_customer_modtime BEFORE
UPDATE ON duels FOR EACH ROW
EXECUTE FUNCTION update_modified_column ();
