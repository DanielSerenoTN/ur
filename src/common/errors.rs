use actix_web::{ResponseError, http::StatusCode, HttpResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid Token: {0}")]
    InvalidToken(String),

    #[error("Expired Token: {0}")]
    TokenExpired(String),

    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(String),

    #[error("Internal Error: {0}")]
    InternalError(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            ApiError::TokenExpired(_) => StatusCode::UNAUTHORIZED,
            ApiError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let msg = format!("{{\"error\":\"{}\"}}", self.to_string());
        HttpResponse::build(self.status_code()).body(msg)
    }
}
