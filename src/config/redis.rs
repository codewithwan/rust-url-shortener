use redis::Client;
use std::env;

pub async fn configure_redis() -> redis::aio::Connection {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = Client::open(redis_url).expect("Failed to create Redis client");
    let redis_pool = redis_client.get_connection().expect("Failed to connect to Redis");

    redis_pool
}

