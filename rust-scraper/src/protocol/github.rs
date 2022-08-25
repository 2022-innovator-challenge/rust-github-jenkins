#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GithubUser {
    login: String
}

#[derive(Deserialize, Debug)]
pub struct GithubPull {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub url: String,
    pub state: String, // enum "open"
    updated_at: String,
    pub statuses_url: String, // call for status
    // head: GitCommit,
    // target: GitCommit,
    user: GithubUser
}

#[derive(Deserialize, Debug)]
pub struct GithubStatus {
    pub id: u64,
    pub description: String,
    pub target_url: String,
    pub state: String, // enum "pending", "error", "success"
    updated_at: String
}

#[derive(Deserialize, Debug)]
pub struct GithubReview {
    pub id: u64,
    pub state: String, // enum "APPROVED", "DISMISSED", "COMMENTED"
    submitted_at: String,
    user: GithubUser
}
