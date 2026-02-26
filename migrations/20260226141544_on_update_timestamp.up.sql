-- Add up migration script here
CREATE OR REPLACE FUNCTION on_update_timestamp()
RETURNS trigger AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ language 'plpgsql';
