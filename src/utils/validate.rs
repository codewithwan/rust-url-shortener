use regex::Regex;
use warp::http::StatusCode;
use warp::reject::{custom, Rejection};
use warp::reply::Reply;

#[derive(Debug)]
pub struct InvalidLink;

impl warp::reject::Reject for InvalidLink {}

/// Validate the link to prevent exploitation
pub fn validate_link(link: String) -> Result<String, Rejection> {
    let url_regex = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();
    let banned_chars = vec![" ", "?", "\"", "&", "%", "javascript:", "data:"];

    if banned_chars.iter().any(|c| link.contains(c)) || !url_regex.is_match(&link) {
        return Err(custom(InvalidLink));
    }

    Ok(link)
}

/// Error handler
pub async fn error_handler(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.find::<InvalidLink>().is_some() {
        Ok(warp::reply::with_status(
            "Invalid link",
            StatusCode::BAD_REQUEST,
        ))
    } else if err.find::<crate::utils::rate_limit::TooManyRequests>().is_some() {
        Ok(warp::reply::with_status(
            "Too many requests, slow down!",
            StatusCode::TOO_MANY_REQUESTS,
        ))
    } else if err.find::<crate::config::db::DbError>().is_some() {
        Ok(warp::reply::with_status(
            "Database error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Unhandled rejection",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
