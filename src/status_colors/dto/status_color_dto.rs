use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StatusColorResponse {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub hexadecimal: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StatusColorsListResponse {
    pub colors: Vec<StatusColorResponse>,
    pub total: usize,
}

impl StatusColorResponse {
    pub fn from_entity(entity: &crate::status_colors::entities::status_color_entity::StatusColor) -> Self {
        Self {
            id: entity.id,
            name: entity.name.clone(),
            status: entity.status.clone(),
            hexadecimal: entity.hexadecimal.clone(),
        }
    }
}