use crate::config::db::{get_original_url, insert_shortlink, DbError};
use crate::models::{ShortenRequest, ShortenResponse};
use crate::utils::validate::validate_link;
use crate::views::not_found::not_found;
use deadpool_postgres::Pool;
use log::{error, info};
use std::env;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use warp::{Rejection, Reply};
use qrcode::QrCode;
use image::Luma;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use base64::engine::general_purpose::STANDARD as base64_std;
use base64::Engine as _;
use std::io::Cursor;
use deadpool_redis::Pool as RedisPool;
use deadpool_redis::redis::AsyncCommands; 
use deadpool_redis::redis::RedisResult;

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
    let short_url = format!("{}/{}", base_url, short_code);

    // Generate QR code from short_url
    let code = QrCode::new(&short_url).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let mut buffer = Cursor::new(Vec::new());
    PngEncoder::new(&mut buffer).write_image(&image, image.width(), image.height(), image::ExtendedColorType::L8).unwrap();
    let qr_base64 = base64_std.encode(buffer.into_inner());

    let response = ShortenResponse {
        short_url,
        qr_code: format!("data:image/png;base64,{}", qr_base64),
    };
    info!("Shortened URL: {} -> {}", body.url, response.short_url);
    Ok(warp::reply::json(&response))
}

/// Handler to redirect a shortened URL to the original URL.
pub async fn redirect_url(
    code: String,
    db_pool: Pool,
    redis_pool: RedisPool,
) -> Result<Box<dyn Reply>, Rejection> {
    let mut redis_conn = redis_pool.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {:?}", e);
        warp::reject::custom(DbError::DatabaseError)
    })?;

    // Add debug logging to track Redis operations
    info!("Attempting to fetch from Redis for code: {}", code);
    
    let redis_key = format!("short:{}", code);
    
    // Try Redis get with explicit error logging
    let redis_result: RedisResult<Option<String>> = redis_conn.get(&redis_key).await;
    match redis_result {
        Ok(Some(original_url)) => {
            info!("Successfully retrieved URL from Redis: {}", original_url);
            match original_url.parse::<warp::http::Uri>() {
                Ok(uri) => return Ok(Box::new(warp::redirect::temporary(uri))),
                Err(e) => {
                    error!("Failed to parse URI from Redis: {:?}", e);
                    return Err(warp::reject::custom(DbError::DatabaseError));
                }
            }
        }
        Ok(None) => {
            info!("No URL found in Redis for key: {}", redis_key);
        }
        Err(e) => {
            error!("Redis error: {:?}", e);
        }
    }

    // If Redis fails, proceed with database lookup
    let client = db_pool.get().await.map_err(|e| {
        error!("DB connection error: {:?}", e);
        warp::reject::custom(DbError::DatabaseError)
    })?;

    match get_original_url(&client, &code).await {
        Ok(Some(original_url)) => {
            let redis_set_result: RedisResult<()> = redis_conn
                .set_ex(&redis_key, &original_url, 3600)
                .await;
            
            match redis_set_result {
                Ok(()) => info!("Successfully cached in Redis: {}", redis_key),
                Err(e) => error!("Failed to cache in Redis: {:?}", e),
            }

            match original_url.parse::<warp::http::Uri>() {
                Ok(uri) => {
                    info!("Redirecting short code {} to {}", code, original_url);
                    return Ok(Box::new(warp::redirect::temporary(uri)));
                }
                Err(e) => {
                    error!("Failed to parse URI from database: {:?}", e);
                    return Err(warp::reject::custom(DbError::DatabaseError));
                }
            }
        }
        Ok(None) => {
            info!("Short code {} not found, displaying 404 page", code);
            let response = not_found().await?;
            return Ok(Box::new(warp::reply::with_status(response.into_response(), StatusCode::NOT_FOUND)));
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            return Err(warp::reject::custom(DbError::DatabaseError));
        }
    }
}

/// Handler to manage rejections and errors.
pub async fn handle_rejection(err: Rejection) -> Result<Box<dyn Reply>, Rejection> {
    if err.is_not_found() {
        info!("Route not found, displaying 404 page");
        let response = not_found().await?;
        Ok(Box::new(warp::reply::with_status(response.into_response(), StatusCode::NOT_FOUND)))
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
    } else if let Some(_) = err.find::<crate::config::db::DbError>() {
        error!("Database error occurred");
        Ok(Box::new(with_status(
            json(&serde_json::json!({ "error": "Database error occurred" })),
            StatusCode::INTERNAL_SERVER_ERROR,
        )))
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        info!("Method not allowed, displaying 404 page");
        let response = not_found().await?;
        Ok(Box::new(warp::reply::with_status(response.into_response(), StatusCode::NOT_FOUND)))
    } else {
        error!("Unhandled rejection: {:?}", err);
        Ok(Box::new(with_status(
            json(&serde_json::json!({ "error": "Unhandled rejection" })),
            StatusCode::INTERNAL_SERVER_ERROR,
        )))
    }
}
