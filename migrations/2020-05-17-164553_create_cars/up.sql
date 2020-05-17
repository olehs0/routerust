-- Your SQL goes here
CREATE TABLE cars (
  id SERIAL PRIMARY KEY,
  engine_id int REFERENCES engines(id),
  brand_id int REFERENCES brands(id),
  price FLOAT NOT NULL,
  release_year int NOT NULL,
  CHECK (price > 0)
);
