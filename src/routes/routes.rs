use crate::handlers::{handle_rejection, redirect_url, shorten_url};
use crate::views::{index::index, not_found::not_found};
use deadpool_postgres::Pool;
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

/// Create the routes for the application.
pub fn create_routes(
    db_pool: Pool,
    redis_pool: Arc<Mutex<MultiplexedConnection>>,
) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let shorten = warp::post()
        .and(warp::path("shorten"))
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(shorten_url)
        .recover(handle_rejection)
        .boxed();

    let redirect = warp::get()
        .and(warp::path::param())
        .and(with_db(db_pool.clone()))
        .and(with_redis(redis_pool.clone()))
        .and_then(redirect_url)
        .boxed();

    let index_route = warp::get().and(warp::path::end()).and_then(index).boxed();

    index_route
        .or(redirect)
        .or(shorten)
        .or(warp::any().and_then(not_found).boxed())
        .recover(handle_rejection)
        .boxed()
}

/// Attach the database pool to the filter.
fn with_db(
    db_pool: Pool,
) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

/// Attach Redis connection to the filter.
fn with_redis(
    redis_pool: Arc<Mutex<MultiplexedConnection>>,
) -> impl Filter<Extract = (Arc<Mutex<MultiplexedConnection>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || redis_pool.clone())
}
