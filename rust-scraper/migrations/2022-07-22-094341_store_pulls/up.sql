-- Your SQL goes here

CREATE TABLE IF NOT EXISTS gh_pulls (
    conf_id INTEGER,
    pull_id INTEGER,
    number INTEGER NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    state TEXT NOT NULL,
    -- updated_at TIMESTAMP NOT NULL,
    -- statuses_url TIMESTAMP NOT NULL,
    -- user: GithubUser,
    PRIMARY KEY(conf_id, pull_id)
);


CREATE TABLE IF NOT EXISTS gh_statuses (
    conf_id INTEGER,
    pull_id INTEGER,
    status_id INTEGER,
    description TEXT NOT NULL,
    target_url TEXT NOT NULL,
    state TEXT NOT NULL,
    -- updated_at TIMESTAMP NOT NULL,
    PRIMARY KEY(conf_id, pull_id, status_id)
);


CREATE TABLE IF NOT EXISTS gh_reviews (
    conf_id INTEGER,
    pull_id INTEGER,
    review_id INTEGER,
    state TEXT NOT NULL,
    -- submitted_at TIMESTAMP NOT NULL,
    -- user_id: GithubUser,
    PRIMARY KEY(conf_id, pull_id, review_id)
)