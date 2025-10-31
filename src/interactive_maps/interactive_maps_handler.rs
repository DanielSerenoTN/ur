use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use crate::{
    db::DbPool,
    interactive_maps::{
        dto::svg_dto::SvgRequest,
        interactive_maps_service::SvgService,
    },
};
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct SvgRequestSchema {
    pub name: String,
    pub content: String,
}

#[utoipa::path(
    post,
    path = "/maps",
    request_body = SvgRequestSchema,
    responses(
        (status = 200, description = "SVG saved successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "SVG"
)]
#[actix_web::post("")]
async fn save_svg(
    data: web::Json<SvgRequest>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Connection Error: {}", e)),
    };

    let mut service = SvgService::new(conn);
    match service.save_svg(data.into_inner()) {
        Ok(svg_info) => HttpResponse::Ok().json(svg_info),
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}

#[utoipa::path(
    post,
    path = "/maps/stream",
    responses(
        (status = 200, description = "SVG stream saved successfully"),
        (status = 400, description = "Missing required parameter"),
        (status = 500, description = "Internal server error")
    ),
    tag = "SVG"
)]
#[actix_web::post("/stream")]
async fn save_svg_stream(
    payload: web::Payload,
    query: web::Query<HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let name = match query.get("name") {
        Some(name) => name.clone(),
        None => return HttpResponse::BadRequest().body("name parameter is required"),
    };

    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error de conexión: {}", e)),
    };

    let mut service = SvgService::new(conn);
    match service.save_svg_streaming(payload, name).await {
        Ok(svg_id) => HttpResponse::Ok().json(svg_id),
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}

#[utoipa::path(
    get,
    path = "/maps/{id}",
    params(
        ("id" = String, Path, description = "Map id", example = "550e8400-e29b-41d4-a716-446655440000")
    ),
    responses(
        (status = 200, description = "SVG retrieved successfully"),
        (status = 404, description = "SVG not found")
    ),
    tag = "SVG"
)]

#[actix_web::get("/{id}")]
async fn get_svg_by_id(
    id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error de conexión: {}", e)),
    };

    let mut service = SvgService::new(conn);
    match service.get_svg_by_id(id.to_string()) {
        Ok((svg_info, svg_content)) => HttpResponse::Ok().json((svg_info, svg_content)),
        Err(error) => HttpResponse::NotFound().body(error),
    }
}

#[utoipa::path(
    delete,
    path = "/maps/delete/{id}",
    params(
        ("id" = String, Path, description = "Map id", example = "550e8400-e29b-41d4-a716-446655440000")
    ),
    responses(
        (status = 200, description = "SVG deleted successfully"),
        (status = 404, description = "SVG not found")
    ),
    tag = "SVG"
)]
#[actix_web::delete("/delete/{id}")]
async fn delete_svg_by_id(
    id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error de conexión: {}", e)),
    };

    let mut service = SvgService::new(conn);
    match service.delete_svg_by_id(id.to_string()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::NotFound().body(error),
    }
}

#[utoipa::path(
    get,
    path = "/maps",
    params(
        ("page" = u32, Query, description = "Page number for pagination", example = 1),
        ("per_page" = u32, Query, description = "Number of items per page", example = 10)
    ),
    responses(
        (status = 200, description = "SVGs retrieved successfully"),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "SVG"
)]
#[actix_web::get("")]
async fn get_paginated_svgs(
    pool: web::Data<DbPool>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let page = query.get("page").and_then(|p| p.parse().ok()).unwrap_or(1);
    let per_page = query
        .get("per_page")
        .and_then(|pp| pp.parse().ok())
        .unwrap_or(10);

    if page < 1 || per_page < 1 {
        return HttpResponse::BadRequest().body("page and per_page must be positive integers");
    }

    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error de conexión: {}", e)),
    };

    let mut service = SvgService::new(conn);
    match service.get_paginated_svgs(page, per_page) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}
