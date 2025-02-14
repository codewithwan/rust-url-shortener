use crate::handlers::{handle_rejection, health_check::health_check, redirect_url, shorten_url};
use crate::views::index::index;
use deadpool_postgres::Pool;
use warp::Filter;

/// Create the routes for the application.
pub fn create_routes(db_pool: Pool) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
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
        .and_then(redirect_url)
        .recover(handle_rejection)
        .boxed();

    let index_route = warp::get().and(warp::path::end()).and_then(index).boxed();

    let health_route = warp::get()
        .and(warp::path("health"))
        .and_then(move || health_check(db_pool.clone()))
        .boxed();

    // Ensure the redirect route is prioritized correctly
    redirect
        .or(index_route)
        .or(health_route)
        .or(shorten)
        .recover(handle_rejection)
        .boxed()
}

/// Attach the database pool to the filter.
fn with_db(
    db_pool: Pool,
) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
