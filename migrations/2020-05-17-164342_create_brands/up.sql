-- Your SQL goes here
CREATE TABLE brands (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  CHECK (name <> '')
);
