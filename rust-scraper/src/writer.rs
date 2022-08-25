#![allow(non_snake_case)]
extern crate diesel;

use crate::protocol::{GithubPull, GithubReview, GithubStatus, JenkinsBuild, JenkinsStage};
use crate::models::{Pull, Review, Status, Build, Stage};
use diesel::prelude::*;

pub async fn create_pull<'a>(conn: &PgConnection, pull: &'a GithubPull, confId: i32) -> usize {
    use crate::schema::gh_pulls;
    let new_pull = Pull {
        conf_id: &(confId as i32),
        pull_id: &(pull.id as i32),
        number: &(pull.number as i32),
        title: &pull.title,
        url: &pull.url,
        state: &pull.state,
    };

    let result: QueryResult<usize> = diesel::insert_into(gh_pulls::table)
        .values(&new_pull)
        .execute(conn);

    result.unwrap_or(0)
}


pub async fn create_build<'a>(conn: &PgConnection, build: &'a JenkinsBuild, pullId: u64, confId: i32) -> usize {
    use crate::schema::jenkins_builds;
    let new_build = Build {
        conf_id: &(confId as i32),
        pull_id: &(pullId as i32),
        build_id: &build.id.parse::<i32>().unwrap(),
        name: &build.name,
        status: &build.status,
        start_time_millis: &(build.startTimeMillis as i32),
        duration_millis: &(build.durationMillis as i32),
    };

    let result: QueryResult<usize> = diesel::insert_into(jenkins_builds::table)
        .values(&new_build)
        .execute(conn);
    
    if result.is_ok() {
        for stage in &build.stages {
            create_stage(conn, stage, build, pullId, confId).await;
        }
    }

    result.unwrap_or(0)
}

pub async fn create_review<'a>(conn: &PgConnection, review: &'a GithubReview, pullId: u64, confId: i32) -> usize {
    use crate::schema::gh_reviews;
    let new_review = Review {
        conf_id: &(confId as i32),
        pull_id: &(pullId as i32),
        review_id: &(review.id as i32),
        state: &review.state,
    };

    let result: QueryResult<usize> = diesel::insert_into(gh_reviews::table)
        .values(&new_review)
        .execute(conn);

    result.unwrap_or(0)
}

pub async fn create_status<'a>(conn: &PgConnection, status: &'a GithubStatus, pullId: u64, confId: i32) -> usize {
    use crate::schema::gh_statuses;
    let new_status = Status {
        conf_id: &(confId as i32),
        pull_id: &(pullId as i32),
        status_id: &(status.id as i32),
        state: &status.state,
        target_url: &status.target_url,
        description: &status.description,
    };

    let result: QueryResult<usize> = diesel::insert_into(gh_statuses::table)
        .values(&new_status)
        .execute(conn);

    result.unwrap_or(0)
}

pub async fn create_stage<'a>(conn: &PgConnection, stage: &'a JenkinsStage, build: &'a JenkinsBuild, pullId: u64, confId: i32) -> usize {
    use crate::schema::jenkins_stages;
    let new_stage = Stage {
        conf_id: &(confId as i32),
        pull_id: &(pullId as i32),
        build_id: &build.id.parse::<i32>().unwrap(),
        stage_id: &stage.id.parse::<i32>().unwrap(),
        name: &stage.name,
        status: &stage.status,
        start_time_millis: &(stage.startTimeMillis as i32),
        duration_millis: &(stage.durationMillis as i32),
    };

    let result: QueryResult<usize> = diesel::insert_into(jenkins_stages::table)
        .values(&new_stage)
        .execute(conn);

    result.unwrap_or(0)
}
