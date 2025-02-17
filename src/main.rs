use dotenv::dotenv;
use env_logger;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use utils::rate_limit::with_ip_rate_limit;
use warp::Filter;
use deadpool_redis::redis::AsyncCommands; 
use config::db::configure_db;

mod db;
mod handlers;
mod models;
mod routes;
mod utils;
mod views;

use crate::routes::create_routes;

/// Main function to start the server.
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // Configure the PostgreSQL database connection pool
    let pool = configure_db().await;

    // Check database connection
    match pool.get().await {
        Ok(_) => println!("Successfully connected to the database."),
        Err(e) => {
            eprintln!("Failed to connect to the database: {:?}", e);
            eprintln!("Please check the DATABASE_URL and ensure the database server is reachable.");
            std::process::exit(1);
        }
    }

    // Configure Redis
    let redis_pool = config::redis::configure_redis().await;

    // Check Redis connection
    let mut conn = redis_pool.get().await.expect("Failed to get Redis connection");
    match conn.ping::<String>().await {
        Ok(pong) => println!("Successfully connected to Redis: {}", pong),
        Err(e) => {
            eprintln!("Failed to connect to Redis: {:?}", e);
            eprintln!("Please check the REDIS_URL and ensure the Redis server is reachable.");
            std::process::exit(1);
        }
    }

    let rate_limiter = Arc::new(Mutex::new(HashMap::new()));

    let routes = create_routes(pool.clone(), redis_pool.clone())
        .and(with_ip_rate_limit(rate_limiter.clone()))
        .recover(utils::validate::error_handler)
        .with(warp::log("warp::server"));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Server is running on port {}", port);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
