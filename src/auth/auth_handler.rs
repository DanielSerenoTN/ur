use actix_web::{cookie::{time::Duration, Cookie}, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use utoipa::ToSchema;
use std::sync::Arc;
use std::env;

use crate::{auth::{auth_service_trait::AuthServiceTrait, dto::auth_dto::AuthResponseMessage}, common::errors::ApiError};

#[derive(Deserialize, Debug, ToSchema)]
pub struct LoginRequest {
    pub code: String,
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authentication successful. Tokens stored in cookies."),
        (status = 500, description = "Internal server error"),
    ),
    tag = "auth"
)]
#[actix_web::post("/login")]
async fn login_with_zoho_handler(
    service: web::Data<Arc<dyn AuthServiceTrait>>,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    match service
        .login_with_code(&login_request.code)
        .await
    {
        Ok(token_response) => {
            let http_only: bool = env::var("COOKIE_HTTP_ONLY").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true);
            let secure: bool = env::var("COOKIE_SECURE").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true);
            let same_site = match env::var("COOKIE_SAME_SITE").unwrap_or_else(|_| "Lax".to_string()).as_str() {
                "Strict" => actix_web::cookie::SameSite::Strict,
                "None" => actix_web::cookie::SameSite::None,
                _ => actix_web::cookie::SameSite::Lax,
            };

            let access_cookie = Cookie::build("access_token", token_response.access_token)
                .http_only(http_only)
                .secure(secure)
                .same_site(same_site.clone())
                .path("/")
                .finish();

            let refresh_cookie = Cookie::build("refresh_token", token_response.refresh_token)
                .http_only(http_only)
                .secure(secure)
                .same_site(same_site)
                .path("/")
                .finish();

            let response_message = AuthResponseMessage {
                message: "Authentication successful. Tokens are stored in cookies.".to_string(),
                tokens_in_cookies: true,
            };

            HttpResponse::Ok()
                .cookie(access_cookie)
                .cookie(refresh_cookie)
                .json(response_message)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error obtaining token: {:?}", err))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/auth/refresh",
    responses(
        (status = 200, description = "New tokens generated and stored in cookies."),
        (status = 400, description = "Invalid token or missing refresh token cookie"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "auth"
)]

#[actix_web::get("/refresh")]

pub async fn auth_refresh_token(
    req: HttpRequest,
    service: web::Data<Arc<dyn AuthServiceTrait>>,
) -> Result<impl Responder, ApiError> {
    let refresh_token = match req.cookie("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return Err(ApiError::InvalidToken("No refresh token cookie".to_string()));
        }
    };


    let token_response = service.refresh_tokens(&refresh_token).map_err(|err| {
        eprintln!("Error refreshing tokens: {:?}", err);
        ApiError::InvalidToken("Failed to refresh tokens".to_string())
    })?;

    let http_only: bool = env::var("COOKIE_HTTP_ONLY").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true);
    let secure: bool = env::var("COOKIE_SECURE").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true);
    let same_site = match env::var("COOKIE_SAME_SITE").unwrap_or_else(|_| "Lax".to_string()).as_str() {
        "Strict" => actix_web::cookie::SameSite::Strict,
        "None" => actix_web::cookie::SameSite::None,
        _ => actix_web::cookie::SameSite::Lax,
    };
    let access_cookie = Cookie::build("access_token", token_response.access_token)
        .http_only(http_only)
        .secure(secure)
        .same_site(same_site)
        .path("/")
        .finish();
    
    let refresh_cookie = Cookie::build("refresh_token", token_response.refresh_token)
        .http_only(http_only)
        .secure(secure)
        .same_site(same_site)
        .path("/")
        .finish();

    let response_message = AuthResponseMessage {
        message: "Authentication successful. Tokens are stored in cookies.".to_string(),
        tokens_in_cookies: true,
    };

    Ok(HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(response_message))
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 400, description = "Missing or invalid refresh token cookie"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "auth"
)]
#[actix_web::post("/logout")]

pub async fn logout_handler() -> impl Responder {
    let access_cookie = Cookie::build("access_token", "")
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::seconds(-1))
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", "")
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::seconds(-1)) 
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json("Logout exitoso")
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "User information retrieved successfully"),
        (status = 400, description = "Missing or invalid refresh token cookie"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "auth"
)]
#[actix_web::get("/me")]
pub async fn get_me_handler(
    req: HttpRequest,
    service: web::Data<Arc<dyn AuthServiceTrait>>,
) -> impl Responder {

    let access_token = match req.cookie("access_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return HttpResponse::Unauthorized().json("Access token not found."),
    };

    match service.verify_token(&access_token) {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(_) => HttpResponse::Unauthorized().json("Invalid or expired token."),
    }
}