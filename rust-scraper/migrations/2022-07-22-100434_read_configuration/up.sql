-- Your SQL goes here

CREATE TABLE IF NOT EXISTS gh_configurations (
    conf_id SERIAL PRIMARY KEY,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    server TEXT NOT NULL,
    org TEXT NOT NULL,
    repo TEXT NOT NULL,
    token TEXT NULL,
    jenkins TEXT NULL -- needs 1:n   
)