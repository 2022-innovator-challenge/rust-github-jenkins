use super::schema::gh_pulls;
use super::schema::gh_statuses;
use super::schema::gh_reviews;
use super::schema::jenkins_builds;
use super::schema::jenkins_stages;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Queryable)]
#[diesel(table_name="gh_pulls")]
pub struct Pull {
    pub conf_id: i32,
    pub pull_id: i32,
    pub number: i32,
    pub title: String,
    pub url: String,
    pub state: String,
}

#[derive(Serialize)]
#[derive(Queryable)]
#[diesel(table_name="gh_statuses")]
pub struct Status {
    pub conf_id: i32,
    pub pull_id: i32,
    pub status_id: i32,
    pub description: String,
    pub target_url: String,
    pub state: String,
}

#[derive(Serialize)]
#[derive(Queryable)]
#[diesel(table_name="gh_reviews")]
pub struct Review {
    pub conf_id: i32,
    pub pull_id: i32,
    pub review_id: i32,
    pub state: String,
}

#[derive(Serialize)]
#[derive(Queryable)]
#[diesel(table_name="jenkins_builds")]
pub struct Build {
    pub conf_id: i32,
    pub pull_id: i32, 
    pub build_id: i32,
    pub name: String,
    pub status: String,
    pub start_time_millis: i32,
    pub duration_millis: i32,
}

#[derive(Serialize)]
#[derive(Queryable)]
#[diesel(table_name="jenkins_stages")]
pub struct Stage {
    pub conf_id: i32,
    pub pull_id: i32, 
    pub build_id: i32,
    pub stage_id: i32,
    pub name: String,
    pub status: String,
    pub start_time_millis: i32,
    pub duration_millis: i32,
}

#[derive(Serialize)]
#[derive(Queryable)]
#[diesel(table_name="configurations")]
pub struct Configuration {
    pub conf_id: i32,
    pub active: bool,
    pub server: String,
    pub org: String,
    pub repo: String,
    pub token: Option<String>,
    pub jenkins: Option<String>,
}