#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use web::schema::{gh_configurations::dsl::*, gh_pulls};
use diesel::prelude::*;
use serde_json::json;
use rocket::http::RawStr;


static DEFAULT_LOG_LEVEL: &str = "info";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/configurations")]
fn configurations() -> String {
    
    // establish sql connection
    let sql_conn = web::establish_connection();
    
    let configurations = gh_configurations.filter(active.eq(true))
        .limit(1000)
        .load::<web::models::Configuration>(&sql_conn)
        .expect("Error loading configurations");

    let configurations_json = serde_json::to_string(&configurations).expect("Unable to serialize to JSON.");
    configurations_json.to_string()
}


#[get("/gh_pulls?<confid>")]
fn pulls(confid: &RawStr) -> String {
    
    // establish sql connection
    let sql_conn = web::establish_connection();
    
    let configurations = gh_pulls.filter(gh_puls::conf_id.eq(confid))
        .limit(1000)
        .load::<web::models::Pull>(&sql_conn)
        .expect("Error loading configurations");

    let configurations_json = serde_json::to_string(&configurations).expect("Unable to serialize to JSON.");
    configurations_json.to_string()
}



fn main() {
    // set default log level to info
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, DEFAULT_LOG_LEVEL));
    
    rocket::ignite()
        .mount("/", routes![index, configurations, pulls])
        .launch();
}
