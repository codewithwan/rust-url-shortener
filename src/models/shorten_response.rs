use serde::{Deserialize, Serialize};

/// Response structure for shortened URL.
#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
    pub qr_code: String, // Add QR code field
}

/// Request structure for shortening URL.
#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}
