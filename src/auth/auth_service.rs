use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use actix_web::dev::ServiceRequest;
use std::sync::Arc;

use crate::{common::errors::ApiError, http::zoho::{search_map_access_by_name, ZohoMapAccess}};
use crate::zoho_code::zoho_code_service::ZohoCodeService;

use super::{auth_service_trait::AuthServiceTrait, dto::auth_dto::TokenResponseDto, entities::auth_entities::Claims};

pub struct AuthService {
    jwt_secret: String,
    jwt_refresh_secret: String,
    token_expiration: usize,
    token_refresh_expiration: usize,
    zoho_code_service: Arc<ZohoCodeService>,
}

impl AuthService {
    pub fn new(
        jwt_secret: String, 
        jwt_refresh_secret: String, 
        token_expiration: usize, 
        token_refresh_expiration: usize,
        zoho_code_service: Arc<ZohoCodeService>
    ) -> Self {
        Self {
            jwt_secret,
            jwt_refresh_secret,
            token_expiration,
            token_refresh_expiration,
            zoho_code_service,
        }
    }

    pub fn generate_tokens(&self, user_id: &str, name: &str) -> Result<TokenResponseDto> {
        let access_exp = Utc::now().timestamp() as usize + self.token_expiration;
        let refresh_exp = Utc::now().timestamp() as usize + self.token_refresh_expiration;

        let access_claims = Claims {
            id: user_id.to_string(),
            name: name.to_string(),
            exp: access_exp,
        };

        let refresh_claims = Claims {
            id: user_id.to_string(),
            name: name.to_string(),
            exp: refresh_exp,
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_refresh_secret.as_ref()),
        )?;

        Ok(TokenResponseDto {
            access_token,
            refresh_token,
        })
    }

    pub async fn login_with_code(&self, code: &str) -> Result<TokenResponseDto> {
        let is_valid = self.zoho_code_service
            .validate_code(code)
            .map_err(|err| anyhow::anyhow!("Error validating code: {}", err))?;

        if !is_valid {
            anyhow::bail!("Código de Zoho inválido o expirado");
        }

        println!("Zoho code validated successfully from DB: {}", code);

        let map_access = self
            .get_map_access_by_name(code)
            .await
            .map_err(|err| {
                if err.to_string().contains("Unauthorized") {
                    anyhow::anyhow!("No autorizado: Verifica tus credenciales o permisos para acceder a Zoho")
                } else {
                    anyhow::anyhow!("Error al obtener acceso al mapa: {}", err)
                }
            })?;
        
        self.generate_tokens(&map_access.id, &map_access.name)
    }

    pub async fn login_with_zoho(&self, name: &str) -> Result<TokenResponseDto> {
        let map_access = self
            .get_map_access_by_name(name)
            .await
            .map_err(|err| {
                if err.to_string().contains("Unauthorized") {
                    anyhow::anyhow!("No autorizado: Verifica tus credenciales o permisos para acceder a Zoho")
                } else {
                    anyhow::anyhow!("Error al obtener acceso al mapa: {}", err)
                }
            })?;
        
        if map_access.name != name {
            anyhow::bail!(
                "No autorizado: Verifica tus credenciales o permisos para acceder a Zoho"
            );
        };

        self.generate_tokens(&map_access.id, &map_access.name)
    }

    pub fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenResponseDto> {
        let token_data = decode::<Claims>(
            refresh_token,
            &DecodingKey::from_secret(self.jwt_refresh_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;

        let claims = token_data.claims;
        let now = Utc::now().timestamp() as usize;
        if now > claims.exp {
            return Err(anyhow::anyhow!("Refresh token has expired"));
        }

        self.generate_tokens(&claims.id, &claims.name)
    }

    async fn get_map_access_by_name(&self, name: &str) -> Result<ZohoMapAccess, ApiError> {
        search_map_access_by_name(name).await.map_err(|err| {
            eprintln!("Error fetching map access from Zoho: {:?}", err);
            if err.to_string().contains("No map access found") {
                ApiError::NotFound(format!("No map access found for the name: {}", name))
            } else {
                ApiError::InternalError("Failed to fetch map access from Zoho".to_string())
            }
        })
    }

    pub fn extract_token_impl(req: &ServiceRequest) -> Option<String> {
        req.cookie("access_token")
            .map(|cookie| cookie.value().to_string())
            .or_else(|| {
                req.headers().get("Authorization").and_then(|header| {
                    header.to_str().ok().map(|value| value.replace("Bearer ", ""))
                })
            })
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn login_with_code(&self, code: &str) -> Result<TokenResponseDto> {
        self.login_with_code(code).await
    }

    fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenResponseDto> {
        self.refresh_tokens(refresh_token)
    }

    fn verify_token(&self, token: &str) -> Result<Claims> {
        self.verify_token(token)
    }
    
    fn extract_token(&self, req: &ServiceRequest) -> Option<String> {
        AuthService::extract_token_impl(req)
    }
}