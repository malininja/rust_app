-- Add down migration script here
DELETE FROM roles where code IN ('A', 'U');
