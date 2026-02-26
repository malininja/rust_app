-- Add down migration script here
DROP TRIGGER roles_updated_at ON roles;

ALTER TABLE roles
  DROP COLUMN created_at,
  DROP COLUMN updated_at;
