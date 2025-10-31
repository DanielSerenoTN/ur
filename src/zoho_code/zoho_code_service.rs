use std::sync::Arc;
use tokio::sync::Mutex;
use crate::common::errors::ApiError;
use super::zoho_code_repository::ZohoCodeRepository;
use super::entities::zoho_code_entity::ZohoCode;

pub struct ZohoCodeService {
    repository: ZohoCodeRepository,
}

impl ZohoCodeService {
    pub fn new(repository: ZohoCodeRepository) -> Self {
        Self { repository }
    }

    pub fn validate_code(&self, code: &str) -> Result<bool, ApiError> {
        self.repository
            .is_code_active(code)
            .map_err(|e| {
                eprintln!("Error validating Zoho code: {:?}", e);
                ApiError::InternalError("Error validating Zoho code".to_string())
            })
    }

    pub fn get_current_active_code(&self) -> Result<Option<ZohoCode>, ApiError> {
        self.repository
            .get_active_code()
            .map_err(|e| {
                eprintln!("Error getting active Zoho code: {:?}", e);
                ApiError::InternalError("Error getting active Zoho code".to_string())
            })
    }

    pub fn update_code(&self, new_code: &str) -> Result<ZohoCode, ApiError> {
        if let Err(e) = self.repository.expire_all_active_codes() {
            eprintln!("Warning: Failed to expire old codes: {:?}", e);
        }

        self.repository
            .create_zoho_code(new_code)
            .map_err(|e| {
                eprintln!("Error creating new Zoho code: {:?}", e);
                ApiError::InternalError("Error creating new Zoho code".to_string())
            })
    }

    
    pub fn needs_update(&self, current_zoho_code: &str) -> Result<bool, ApiError> {
        match self.get_current_active_code()? {
            Some(db_code) => {
                let needs_update = db_code.code != current_zoho_code;
                if needs_update {
                    println!("Zoho code mismatch - DB: {}, Current: {}", 
                        db_code.code, current_zoho_code);
                }
                Ok(needs_update)
            }
            None => {
                println!("No active Zoho code in DB, need to create one");
                Ok(true) 
            }
        }
    }

    pub fn initialize_code(&self, code: &str) -> Result<ZohoCode, ApiError> {
        if let Some(existing) = self.get_current_active_code()? {
            if existing.code == code {
                println!("Zoho code already exists and is current");
                return Ok(existing);
            }
        }
        self.update_code(code)
    }

}

#[derive(Debug, serde::Serialize)]
pub struct ZohoCodeStats {
    pub has_active_code: bool,
    pub active_code: Option<String>,
    pub total_codes_in_history: usize,
}