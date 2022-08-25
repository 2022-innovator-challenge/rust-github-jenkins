-- Your SQL goes here


CREATE TABLE IF NOT EXISTS jenkins_builds (
    conf_id INTEGER,
    pull_id INTEGER,
    build_id INTEGER,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    start_time_millis INTEGER NOT NULL,
    duration_millis INTEGER NOT NULL,
    PRIMARY KEY(conf_id, pull_id, build_id)
);


CREATE TABLE IF NOT EXISTS jenkins_stages (
    conf_id INTEGER,
    pull_id INTEGER,
    build_id INTEGER,
    stage_id INTEGER,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    start_time_millis INTEGER NOT NULL,
    duration_millis INTEGER NOT NULL,
    PRIMARY KEY(conf_id, pull_id, build_id, stage_id)
)