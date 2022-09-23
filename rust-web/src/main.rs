#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

use web::{schema::{gh_configurations::dsl::*, gh_pulls}, models::Pull, models::Configuration};
use diesel::prelude::*;
use rocket::http::RawStr;
use rocket_contrib::json::Json;


static DEFAULT_LOG_LEVEL: &str = "info";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/configurations")]
fn configurations() -> Json<Vec<Configuration>> {
    
    // establish sql connection
    let sql_conn = web::establish_connection();
    
    let configurations = gh_configurations.filter(active.eq(true))
        .limit(1000)
        .load::<web::models::Configuration>(&sql_conn)
        .expect("Error loading configurations");

    Json(configurations)
}


#[get("/gh_pulls?<confid>")]
fn pulls(confid: &RawStr) -> Json<Vec<Pull>> {
    
    // establish sql connection
    let sql_conn = web::establish_connection();
    let cid: i32 = confid.as_str().parse().unwrap();

    let configurations = gh_pulls::table
        .filter(gh_pulls::conf_id.eq(cid))
        .limit(1000)
        .load::<web::models::Pull>(&sql_conn)
        .expect("Error loading configurations");

    Json(configurations)
}



fn main() {
    // set default log level to info
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, DEFAULT_LOG_LEVEL));
    
    rocket::ignite()
        .mount("/", routes![index, configurations, pulls])
        .launch();
}
