use deadpool_redis::{Config, Pool};
use std::env;
use log::info;

pub async fn configure_redis() -> Pool {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    info!("Connecting to Redis at {}", redis_url);
    
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(None).expect("Failed to create Redis pool");

    info!("Successfully created Redis pool");
    pool
}
