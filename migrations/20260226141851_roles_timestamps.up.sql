-- Add up migration script here
ALTER TABLE roles
  ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE TRIGGER roles_updated_at
BEFORE UPDATE ON roles
FOR EACH ROW
EXECUTE FUNCTION on_update_timestamp();