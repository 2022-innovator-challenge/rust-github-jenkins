#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JenkinsBuild {
    pub id: String,
    pub name: String,
    pub status: String, // enum "FAILED", "SUCCESS", "SKIPPED"?
    pub startTimeMillis: u64,
    pub durationMillis: u64,
    pub stages: Vec<JenkinsStage>
}


#[derive(Deserialize, Debug)]
pub struct JenkinsStage {
    pub id: String,
    pub name: String,
    pub status: String, // enum "FAILED", "SUCCESS", "SKIPPED"?
    pub startTimeMillis: u64,
    pub durationMillis: u64
}
