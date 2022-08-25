#![allow(non_snake_case)]
extern crate diesel;
extern crate scrape;

use scrape::scraper;

static DEFAULT_LOG_LEVEL: &str = "info";

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    // set default log level to info
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, DEFAULT_LOG_LEVEL));
    
    scraper::start().await
}
