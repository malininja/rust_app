-- Add down migration script here

DROP TRIGGER users_updated_at ON users;

DROP TABLE users;
