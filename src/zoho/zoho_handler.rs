use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use std::env;
use std::sync::Arc;

use crate::zoho::zoho_trait::ZohoServiceTrait;
use crate::common::errors::ApiError;

#[derive(Deserialize, Debug)]
struct ProductRequest {
    product_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct MapAccessRequest {
    pub code: String,
}

#[actix_web::get("/token")]
async fn get_access_token_handler(
    service: web::Data<Arc<dyn ZohoServiceTrait>>,
) -> Result<impl Responder, ApiError> {
    service
        .get_access_token()
        .await
        .map(|token| HttpResponse::Ok().json(token))
        .map_err(|err| {
            eprintln!("Error fetching Zoho token: {:?}", err);
            ApiError::InternalError(err.to_string())
        })
}

#[actix_web::post("/search")]
async fn get_products_by_ids_handler(
    service: web::Data<Arc<dyn ZohoServiceTrait>>,
    product_request: web::Json<ProductRequest>,
) -> Result<impl Responder, ApiError> {
service
        .get_products_by_ids(product_request.product_ids.iter().map(String::as_str).collect())
        .await
        .map(|products| HttpResponse::Ok().json(products))
        .map_err(|err| {
            eprintln!("Error fetching products: {:?}", err);
            if err.to_string().contains("No products found") {
                ApiError::NotFound("No products found for the given IDs".to_string())
            } else {
                ApiError::InternalError(err.to_string())
            }
        })
}

#[get("/url")]
async fn get_url_base_zoho() -> Result<impl Responder, ApiError> {
    let zoho_url_base = env::var("ZOHO_URL_BASE")
        .map_err(|_| ApiError::InternalError("ZOHO_URL_BASE is not set in .env".to_string()))?;
    
    Ok(HttpResponse::Ok().json(zoho_url_base))
}



