use serde::{Deserialize, Serialize};

/// Response structure for shortened URL.
#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

/// Request structure for shortening URL.
#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}
