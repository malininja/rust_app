-- Add down migration script here
DROP TRIGGER articles_updated_at ON articles;

DROP TABLE articles;

DROP TYPE unit_of_measure;