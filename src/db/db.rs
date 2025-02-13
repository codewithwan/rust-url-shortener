use thiserror::Error;
use tokio_postgres::Client;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error")]
    DatabaseError,
}

impl warp::reject::Reject for DbError {}

/// Insert a shortlink into the database.
pub async fn insert_shortlink(client: &Client, short_code: &str, original_url: &str) -> Result<(), DbError> {
    let query = "INSERT INTO shortlink (short_code, original_url) VALUES ($1, $2)";
    client
        .execute(query, &[&short_code, &original_url])
        .await
        .map_err(|_| DbError::DatabaseError)?;
    Ok(())
}

/// Retrieve the original URL from the database using the short code.
pub async fn get_original_url(client: &Client, short_code: &str) -> Result<Option<String>, DbError> {
    let query = "SELECT original_url FROM shortlink WHERE short_code = $1";
    let row = client.query_opt(query, &[&short_code]).await.map_err(|_| DbError::DatabaseError)?;
    Ok(row.map(|r| r.get(0)))
}
