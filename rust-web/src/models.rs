use super::schema::gh_pulls;
use super::schema::gh_statuses;
use super::schema::gh_reviews;
use super::schema::jenkins_builds;
use super::schema::jenkins_stages;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name="gh_pulls"]
pub struct Pull<'a> {
    pub conf_id: &'a i32,
    pub pull_id: &'a i32,
    pub number: &'a i32,
    pub title: &'a str,
    pub url: &'a str,
    pub state: &'a str,
}

#[derive(Insertable)]
#[table_name="gh_statuses"]
pub struct Status<'a> {
    pub conf_id: &'a i32,
    pub pull_id: &'a i32,
    pub status_id: &'a i32,
    pub description: &'a str,
    pub target_url: &'a str,
    pub state: &'a str,
}

#[derive(Insertable)]
#[table_name="gh_reviews"]
pub struct Review<'a> {
    pub conf_id: &'a i32,
    pub pull_id: &'a i32,
    pub review_id: &'a i32,
    pub state: &'a str,
}

#[derive(Insertable)]
#[table_name="jenkins_builds"]
pub struct Build<'a> {
    pub conf_id: &'a i32,
    pub pull_id: &'a i32, 
    pub build_id: &'a i32,
    pub name: &'a str,
    pub status: &'a str,
    pub start_time_millis: &'a i32,
    pub duration_millis: &'a i32,
}

#[derive(Insertable)]
#[table_name="jenkins_stages"]
pub struct Stage<'a> {
    pub conf_id: &'a i32,
    pub pull_id: &'a i32, 
    pub build_id: &'a i32,
    pub stage_id: &'a i32,
    pub name: &'a str,
    pub status: &'a str,
    pub start_time_millis: &'a i32,
    pub duration_millis: &'a i32,
}

#[derive(Serialize)]
#[derive(Queryable)]
pub struct Configuration {
    pub conf_id: i32,
    pub active: bool,
    pub server: String,
    pub org: String,
    pub repo: String,
    pub token: Option<String>,
    pub jenkins: Option<String>,
}