use crate::db::db::DbError;
use deadpool_postgres::Pool;
use warp::{Rejection, Reply};

/// Health check handler to verify database connectivity.
pub async fn health_check(db_pool: Pool) -> Result<impl Reply, Rejection> {
    let client = db_pool.get().await.map_err(|_| warp::reject::custom(DbError::DatabaseError))?;
    client.simple_query("SELECT 1").await.map_err(|_| warp::reject::custom(DbError::DatabaseError))?;
    Ok(warp::reply::json(&serde_json::json!({ "status": "OK" })))
}
