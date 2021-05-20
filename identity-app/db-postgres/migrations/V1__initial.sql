CREATE TABLE IF NOT EXISTS author_initial
(
    id      SERIAL PRIMARY KEY,
    name    VARCHAR NOT NULL,
    country VARCHAR NOT NULL
)