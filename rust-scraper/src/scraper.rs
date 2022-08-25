#![allow(non_snake_case)]

use regex::Regex;
use reqwest::Error;
use reqwest::header::AUTHORIZATION;
use log::{info, debug, trace};

use crate::establish_connection;
use crate::models::{Configuration};
use crate::schema::gh_configurations::dsl::*;
use diesel::prelude::*;
use std::{thread, time};

static DEFAULT_LOOP_WAIT: u64 = 15_000;

pub async fn start() -> Result<(), &'static str> {
    // init http client, accepting non-validateable certificates
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    // establish sql connection
    let sqlConn = establish_connection();

    // run repetitive task
    schedule(DEFAULT_LOOP_WAIT, 5, &client, &sqlConn).await
}

async fn schedule(sleepDuration: u64, configLimit: i64, client: &reqwest::Client, sqlConn: &PgConnection) -> Result<(), &'static str> {
    loop {
        let results = getConfigurations(sqlConn, configLimit).await;
        info!("Working {} configurations", results.len());

        for conf in results {
            scheduleConfiguration(&conf, client, sqlConn).await;
        }
        thread::sleep(time::Duration::from_millis(sleepDuration));
    }
    Ok(())
}

async fn scheduleConfiguration(conf: &Configuration, client: &reqwest::Client, sqlConn: &PgConnection) -> Vec<crate::protocol::GithubPull> {
    debug!("Starting process for configuration with {} {} {}", conf.server, conf.org, conf.repo);
        
    // if fails with "invalid type: map, expected a sequence", then it's likely an outdated/invalid access token
    let pulls = queryGithubPulls(client, &conf.token, &conf.server, &conf.org, &conf.repo).await;
    let pullsUnwrap = pulls.unwrap();
    for pull in &pullsUnwrap {
        debug!("GitHub Pull-Request: {:?}", pull);

        // store GH pull into database
        let local_pull_id = crate::writer::create_pull(sqlConn, pull, conf.conf_id).await;
        if local_pull_id==0 && pull.state.eq_ignore_ascii_case("closed") {
            debug!("Pull request {} already scanned.", pull.id);
            continue;
        }

        let statuses = queryGithubStatuses(client, &conf.token, &(pull.statuses_url)).await;

        for status in statuses.unwrap() {
            match status.state.as_str() {
                    "error"|"success" => {
                        crate::writer::create_status(sqlConn, &status, pull.id, conf.conf_id).await;
                        let buildDetails = queryGithubStatus(client, &conf.jenkins, &status.target_url).await;
                        debug!("GitHub Pull-Request Status Details: {:?}", buildDetails);
                        if buildDetails.is_ok() {
                            crate::writer::create_build(sqlConn, &buildDetails.unwrap(), pull.id, conf.conf_id).await;
                        }
                    },
                    "pending" => trace!("GitHub Pull-Request Status Pending. Not querying."),
                    _ => trace!("GitHub Pull-Request Status Unknown {:?}", status.state),
            }
        }

        let reviews  = queryGithubReviews(client, &conf.token, &conf.server, &conf.org, &conf.repo, &pull.number).await;
        info!("Reviews: {:?}", reviews);
        for review in reviews.unwrap() {
            crate::writer::create_review(sqlConn, &review, pull.id, conf.conf_id).await;
        }
    }
    pullsUnwrap
}

async fn getConfigurations(conn: &PgConnection, limit: i64) -> Vec<Configuration> {
    gh_configurations.filter(active.eq(true))
        .limit(limit)
        .load::<Configuration>(conn)
        .expect("Error loading configurations")
}

async fn queryGithubPulls(client: &reqwest::Client, ghAuhtorization: &Option<String>, ghServer: &str, ghOrg: &str, ghRepo: &str) -> Result<Vec<crate::protocol::GithubPull>, Error> {
    let url = format!("https://{server}/api/v3/repos/{org}/{repo}/pulls?per_page=50&state=all",
                              server = ghServer,
                              org = ghOrg,
                              repo = ghRepo);
    info!("Querying GitHub URL: {}", url);

    client
        .get(&url)
        .header(AUTHORIZATION, ghAuhtorization.as_ref().unwrap())
        .send()
        .await?
        .json()
        .await
}

async fn queryGithubStatuses(client: &reqwest::Client, ghAuhtorization: &Option<String>, ghUrl: &str) -> Result<Vec<crate::protocol::GithubStatus>, Error> {
    info!("Querying GitHub Status for PR: {}",  ghUrl);

    client
        .get(ghUrl)
        .header(AUTHORIZATION, ghAuhtorization.as_ref().unwrap())
        .send()
        .await?
        .json()
        .await
}

async fn queryGithubReviews(client: &reqwest::Client, ghAuhtorization: &Option<String>, ghServer: &str, ghOrg: &str, ghRepo: &str, ghPull: &u64) -> Result<Vec<crate::protocol::GithubReview>, Error> {
    let url = format!("https://{server}/api/v3/repos/{org}/{repo}/pulls/{id}/reviews",
                            server = ghServer,
                            org = ghOrg,
                            repo = ghRepo,
                            id = ghPull);
    info!("Querying GitHub Reviews for PR: {}", url);

    client
        .get(&url)
        .header(AUTHORIZATION, ghAuhtorization.as_ref().unwrap())
        .send()
        .await?
        .json()
        .await
}

async fn queryGithubStatus(client: &reqwest::Client, jenkinsServer: &Option<String>, jenkinsUrl: &str) -> Result<crate::protocol::JenkinsBuild, Error> {
    if !jenkinsUrl.contains(jenkinsServer.as_ref().unwrap()) {
        panic!("Unexpected status url");
    }

    let re = Regex::new(r"/job/(?P<job>\w+)/job/(?P<build>[\w-]+)/(?P<run>\d+)/display/redirect$").unwrap();
    let url:String = re.replace(jenkinsUrl, "/job/${job}/job/${build}/${run}/wfapi/describe").into();
    info!("Querying Jenkins build status for PR: {}", url);

    client
        .get(&url)
        .send()
        .await?
        .json()
        .await
}

