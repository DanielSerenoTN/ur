use std::{collections::HashMap, sync::Mutex};

use serde::Serialize;
use uuid::Uuid;

use crate::interactive_maps::dto::svg_dto::SvgInfo;

pub struct AppState {
    pub svg_db: Mutex<HashMap<Uuid, SvgInfo>>,
    pub storage_path: String,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub per_page: i64,
    pub current_page: i64,
    pub total_pages: i64,
}