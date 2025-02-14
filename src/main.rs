use deadpool_postgres::{Manager, Pool};
use dotenv::dotenv;
use env_logger;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Config, NoTls};
use utils::rate_limit::with_ip_rate_limit;
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

    // Parse the database URL
    let config: Config = database_url.parse().expect("Invalid DATABASE_URL");

    // Create Manager from the configured settings
    let mgr = Manager::new(config, NoTls);
    let pool = Pool::builder(mgr)
        .max_size(16) // Maximum 16 connections in the pool
        .build()
        .unwrap();

    // Check database connection
    match pool.get().await {
        Ok(_) => println!("Successfully connected to the database."),
        Err(e) => {
            eprintln!("Failed to connect to the database: {:?}", e);
            eprintln!("Please check the DATABASE_URL and ensure the database server is reachable.");
            std::process::exit(1);
        }
    }

    let rate_limiter = Arc::new(Mutex::new(HashMap::new()));

    let routes = routes::routes::create_routes(pool.clone())
        .and(with_ip_rate_limit(rate_limiter.clone()))
        .recover(utils::validate::error_handler) 
        .with(warp::log("warp::server"));

    let port = if env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "production" {
        env::var("PORT").unwrap_or_else(|_| "80".to_string()).parse().expect("Invalid port number")
    } else {
        3030
    };

    println!("Server is running on port {}", port);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
