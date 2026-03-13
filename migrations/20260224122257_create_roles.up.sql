-- Add up migration script here
CREATE TABLE roles (
  id INTEGER PRIMARY KEY,
  code VARCHAR(1) NOT NULL,
  name VARCHAR(20) NOT NULL
);
