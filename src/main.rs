use deadpool_postgres::{Manager, Pool};
use dotenv::dotenv;
use env_logger;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Config, NoTls};
use utils::rate_limit::with_ip_rate_limit;
use utils::validate::error_handler;
use warp::Filter;

mod db;
mod handlers;
mod models;
mod routes;
mod utils;
mod views;

/// Main function to start the server.
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

    // Setup connection pool for PostgreSQL using Config
    let mut config = Config::new();
    config.host(&database_url);

    // Create Manager from the configured settings
    let mgr = Manager::new(config, NoTls);
    let pool = Pool::builder(mgr)
        .max_size(16) // Maximum 16 connections in the pool
        .build()
        .unwrap();

    let rate_limiter = Arc::new(Mutex::new(HashMap::new()));

    let routes = routes::routes::create_routes(pool.clone())
        .and(with_ip_rate_limit(rate_limiter.clone()))
        .recover(error_handler)
        .with(warp::log("warp::server"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    if env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "production" {
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3030".to_string());
        println!("Server is running on {}", base_url);
    }
}
