use std::env;

#[derive(Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub serv_addrs: String,
    pub jwt_secret: String,
    pub jwt_refresh_secret: String,
    pub token_expiration: usize,
    pub token_refresh_expiration: usize,
    pub cors_allowed_origins: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let client_id = env::var("ZOHO_CLIENT_ID").expect("Client ID is missing.");
        let client_secret = env::var("ZOHO_CLIENT_SECRET").expect("Client secret is missing.");
        let serv_addrs = env::var("SERV_ADDRS").unwrap_or_else(|_| "0.0.0.0:20090".to_string());
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "test_jwt_secret".to_string());
        let jwt_refresh_secret = env::var("JWT_REFRESH_SECRET").unwrap_or_else(|_| "test_jwt_refresh_secret".to_string());
        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "*".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let token_expiration: usize = env::var("ACCESS_TOKEN_EXPIRATION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(14400);
        let token_refresh_expiration = 30000;

        Config {
            client_id,
            client_secret,
            serv_addrs,
            jwt_secret,
            jwt_refresh_secret,
            token_expiration,
            token_refresh_expiration,
            cors_allowed_origins,
        }
    }
}
