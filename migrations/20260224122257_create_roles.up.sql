-- Add up migration script here
CREATE TABLE roles (
  id UUID DEFAULT(gen_random_uuid()) PRIMARY KEY,
  CODE VARCHAR(1) NOT NULL,
  NAME VARCHAR(20) NOT NULL
);
