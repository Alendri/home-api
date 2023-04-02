-- Add migration script here
CREATE TABLE IF NOT EXISTS people (
  name TEXT NOT NULL,
  last_seen TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS demo_numbers (
  id TEXT PRIMARY KEY NOT NULL,
  int8 INTEGER NOT NULL DEFAULT 0,
  f64 REAL NOT NULL DEFAULT + 0,
  collection TEXT
) STRICT;

INSERT INTO
  demo_numbers(id, int8, f64, collection)
VALUES
  ("abc", 1, 1.1, "demo");

INSERT INTO
  demo_numbers(id, int8, f64, collection)
VALUES
  ("bcd", 2, 2.2, "demo");

INSERT INTO
  demo_numbers(id, int8, f64, collection)
VALUES
  ("cde", 3, 3.3, "demo");

INSERT INTO
  demo_numbers(id, int8, f64)
VALUES
  ("def", 10, 10.0);