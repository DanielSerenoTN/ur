use actix_web::dev::ServiceRequest;
use async_trait::async_trait;
use anyhow::Result;

use super::{dto::auth_dto::TokenResponseDto, entities::auth_entities::Claims};

#[async_trait]
pub trait AuthServiceTrait: Send + Sync  {
    async fn login_with_code(&self, code: &str) -> Result<TokenResponseDto>;
    fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenResponseDto>;
    fn verify_token(&self, token: &str) -> Result<Claims>;
    fn extract_token(&self, req: &ServiceRequest) -> Option<String>;
}