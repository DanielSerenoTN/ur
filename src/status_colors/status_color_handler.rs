use actix_web::{web, HttpResponse, Result};
use utoipa::OpenApi;
use std::sync::Arc;
use crate::common::errors::ApiError;
use super::status_color_service::StatusColorService;
use super::dto::status_color_dto::{StatusColorResponse, StatusColorsListResponse};

#[derive(OpenApi)]
#[openapi(
    paths(get_all_status_colors),
    components(schemas(StatusColorResponse, StatusColorsListResponse))
)]
pub struct StatusColorsOpenApi;

#[utoipa::path(
    get,
    path = "/api/status-colors",
    responses(
        (status = 200, description = "Lista de colores de estatus", body = StatusColorsListResponse),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = "Status Colors"
)]
pub async fn get_all_status_colors(
    status_color_service: web::Data<Arc<StatusColorService>>,
) -> Result<HttpResponse, ApiError> {
    match status_color_service.get_all_colors() {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Err(e),
    }
}


