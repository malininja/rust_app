-- Add up migration script here

CREATE TABLE users (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  role_id INTEGER NOT NULL REFERENCES roles(id),
  username VARCHAR(20) NOT NULL,
  password VARCHAR(200) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TRIGGER users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION on_update_timestamp();
