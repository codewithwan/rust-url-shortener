use crate::db::db::{get_original_url, insert_shortlink, DbError};
use crate::models::{ShortenRequest, ShortenResponse};
use crate::utils::validate::validate_link;
use deadpool_postgres::Pool;
use log::{error, info};
use std::env;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use warp::{Rejection, Reply};

/// Handler to shorten a URL.
pub async fn shorten_url(body: ShortenRequest, db_pool: Pool) -> Result<impl Reply, Rejection> {
    let client = db_pool
        .get()
        .await
        .map_err(|e| {
            error!("Failed to get DB client: {:?}", e);
            warp::reject::custom(DbError::DatabaseError)
        })?;
    let validated_url = validate_link(body.url.clone())?;
    let short_code = Uuid::new_v4().to_string()[..8].to_string();
    insert_shortlink(&client, &short_code, &validated_url)
        .await
        .map_err(|e| {
            error!("Failed to insert shortlink: {:?}", e);
            warp::reject::custom(DbError::DatabaseError)
        })?;
    let base_url = env::var("BASE_URL").expect("BASE_URL is not set in .env");
    let response = ShortenResponse {
        short_url: format!("{}/{}", base_url, short_code),
    };
    info!("Shortened URL: {} -> {}", body.url, response.short_url);
    Ok(warp::reply::json(&response))
}

/// Handler to redirect a shortened URL to the original URL.
pub async fn redirect_url(code: String, db_pool: Pool) -> Result<Box<dyn Reply>, Rejection> {
    let client = db_pool
        .get()
        .await
        .map_err(|_| warp::reject::custom(DbError::DatabaseError))?;
    if let Ok(Some(original_url)) = get_original_url(&client, &code).await {
        let uri: warp::http::Uri = original_url.parse().unwrap();
        info!("Redirecting short code {} to {}", code, original_url);
        Ok(Box::new(warp::redirect::temporary(uri)))
    } else {
        info!("Short code {} not found, redirecting to /", code);
        Ok(Box::new(warp::redirect::temporary(
            "/".parse::<warp::http::Uri>().unwrap(),
        )))
    }
}

/// Handler to manage rejections and errors.
pub async fn handle_rejection(err: Rejection) -> Result<Box<dyn Reply>, Rejection> {
    if err.is_not_found() {
        info!("Route not found, redirecting to /");
        Ok(Box::new(warp::redirect::temporary(
            "/".parse::<warp::http::Uri>().unwrap(),
        )))
    } else if let Some(_) = err.find::<warp::body::BodyDeserializeError>() {
        error!("Invalid request body, redirecting to /");
        Ok(Box::new(warp::redirect::temporary(
            "/".parse::<warp::http::Uri>().unwrap(),
        )))
    } else if let Some(_) = err.find::<crate::utils::validate::InvalidLink>() {
        error!("Invalid link provided");
        Ok(Box::new(with_status(
            json(&serde_json::json!({ "error": "Invalid link provided" })),
            StatusCode::BAD_REQUEST,
        )))
    } else if let Some(_) = err.find::<crate::utils::rate_limit::TooManyRequests>() {
        error!("Too many requests");
        Ok(Box::new(with_status(
            json(&serde_json::json!({ "error": "Too many requests, slow down!" })),
            StatusCode::TOO_MANY_REQUESTS,
        )))
    } else if let Some(_) = err.find::<crate::db::db::DbError>() {
        error!("Database error occurred");
        Ok(Box::new(with_status(
            json(&serde_json::json!({ "error": "Database error occurred" })),
            StatusCode::INTERNAL_SERVER_ERROR,
        )))
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        error!("Method not allowed");
        Ok(Box::new(with_status(
            json(&serde_json::json!({ "error": "Method not allowed" })),
            StatusCode::METHOD_NOT_ALLOWED,
        )))
    } else {
        error!("Unhandled rejection: {:?}", err);
        Ok(Box::new(with_status(
            json(&serde_json::json!({ "error": "Unhandled rejection" })),
            StatusCode::INTERNAL_SERVER_ERROR,
        )))
    }
}
