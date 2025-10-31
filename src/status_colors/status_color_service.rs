use crate::common::errors::ApiError;
use super::status_color_repository::StatusColorRepository;
use super::dto::status_color_dto::{StatusColorResponse, StatusColorsListResponse};

pub struct StatusColorService {
    repository: StatusColorRepository,
}

impl StatusColorService {
    pub fn new(repository: StatusColorRepository) -> Self {
        Self { repository }
    }

    pub fn get_all_colors(&self) -> Result<StatusColorsListResponse, ApiError> {
        match self.repository.get_all_colors() {
            Ok(colors) => {
                let color_responses: Vec<StatusColorResponse> = colors
                    .iter()
                    .map(|color| StatusColorResponse::from_entity(color))
                    .collect();

                let total = color_responses.len();

                Ok(StatusColorsListResponse {
                    colors: color_responses,
                    total,
                })
            }
            Err(e) => {
                eprintln!("Error getting status colors: {:?}", e);
                Err(ApiError::InternalError("Error retrieving status colors".to_string()))
            }
        }
    }
}