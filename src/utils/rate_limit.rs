use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use warp::filters::BoxedFilter;
use warp::reject::Rejection;
use warp::Filter;
use std::env;

#[derive(Debug)]
pub struct TooManyRequests;

impl warp::reject::Reject for TooManyRequests {}

/// Maximum requests per IP
const MAX_REQUESTS_PER_IP: u32 = 10;
/// Rate limit duration in seconds
const RATE_LIMIT_DURATION: Duration = Duration::from_secs(60);

/// Rate limit implementation based on IP
pub async fn rate_limit(
    ip: IpAddr,
    rate_limiter: Arc<Mutex<HashMap<IpAddr, (u32, Instant)>>>,
) -> Result<(), Rejection> {
    // Check if the environment is development
    if env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
        return Ok(());
    }

    let mut requests = rate_limiter.lock().await;
    let current_time = Instant::now();
    let entry = requests.entry(ip).or_insert((0, current_time));

    if current_time.duration_since(entry.1) > RATE_LIMIT_DURATION {
        entry.0 = 0;
        entry.1 = current_time;
    }

    if entry.0 >= MAX_REQUESTS_PER_IP {
        return Err(warp::reject::custom(TooManyRequests));
    }

    entry.0 += 1;

    Ok(())
}

/// Middleware to limit requests per IP
pub fn with_ip_rate_limit(
    rate_limiter: Arc<Mutex<HashMap<IpAddr, (u32, Instant)>>>,
) -> BoxedFilter<()> {
    warp::addr::remote()
        .and_then(move |addr: Option<SocketAddr>| {
            let rate_limiter = rate_limiter.clone();
            async move {
                if let Some(socket) = addr {
                    rate_limit(socket.ip(), rate_limiter).await?;
                }
                Ok(()) as Result<(), Rejection>
            }
        })
        .untuple_one()
        .boxed()
}
