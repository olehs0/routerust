-- Your SQL goes here
CREATE TABLE engines (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  CHECK (name <> '')
);
