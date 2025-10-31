use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponseDto {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct AuthResponseMessage {
    pub message: String,
    pub tokens_in_cookies: bool,
}