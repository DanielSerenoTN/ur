use std::sync::Arc;
use chrono::{Duration, Utc};
use crate::common::errors::ApiError;
use crate::http::zoho::{get_access_token, get_paginated_products, ZohoMapAccess, ZohoProduct};
use super::zoho_trait::ZohoServiceTrait;
use crate::products::products_service::ProductService;
use crate::products::entities::products_entity::{NewProduct, Product};
use tokio::sync::Mutex;

pub struct ZohoService {
    product_service: Arc<Mutex<ProductService>>,
}

impl ZohoService {
    pub fn new(product_service: Arc<Mutex<ProductService>>) -> Self {
        Self { product_service }
    }
}

#[async_trait::async_trait]
impl ZohoServiceTrait for ZohoService {
    async fn get_access_token(&self) -> Result<String, ApiError> {
        get_access_token()
            .await
            .map_err(|err| {
                eprintln!("Error fetching Zoho access token: {:?}", err);
                ApiError::InternalError("Failed to fetch Zoho access token".to_string())
            })
    }

    async fn get_products_by_ids(&self, product_ids: Vec<&str>) -> Result<Vec<ZohoProduct>, ApiError> {
        println!("Searching for {} products in the database...", product_ids.len());

        let mut service = self.product_service.lock().await;

        let cached_products = service.get_many_by_ids(product_ids.clone()).map_err(|err| {
            eprintln!("Error retrieving products from the database: {:?}", err);
            ApiError::InternalError("Failed to get products from the database".to_string())
        })?;

        println!("Found {} products in the database", cached_products.len());
        let valid_zoho_products: Vec<ZohoProduct> = cached_products
            .iter()
            .map(|p| ZohoProduct {
                id: p.id.clone(),
                Product_Name: p.product_name.clone(),
                Estatus_venta: p.estatus_venta.clone(),
            })
            .collect();
        return Ok(valid_zoho_products);
    }

    async fn fetch_all_products_from_zoho(&self) -> Result<Vec<ZohoProduct>, ApiError> {
        println!("Fetching all products from Zoho API in batches of 200...");
    
        let mut all_products = Vec::new();
        let mut page = 1;
        let per_page = 200;
    
        loop {
            println!("Fetching page {}...", page);
    
            let fetched_products = match get_paginated_products(page, per_page).await {
                Ok(products) => products,
                Err(err) => {
                    eprintln!("Error fetching products from Zoho API on page {}: {:?}", page, err);
                    break;
                }
            };
    
            if fetched_products.is_empty() {
                println!("No more products found. Stopping pagination.");
                break;
            }
    
            println!("Retrieved {} products from page {}", fetched_products.len(), page);
            all_products.extend(fetched_products);
            page += 1;
        }
    
        println!("Successfully retrieved {} total products from Zoho API.", all_products.len());
    
        let mut service = self.product_service.lock().await;
    
        println!("Saving {} products to the database...", all_products.len());
        for product in &all_products {
            println!("Saving product {:?} to the database...", product);
            let new_product = NewProduct {
                id: product.id.clone(),
                product_name: product.Product_Name.clone(),
                estatus_venta: product.Estatus_venta.clone(),
            };
    
            if let Err(err) = service.upsert_product(&new_product) {
                eprintln!("Error saving product {} to database: {:?}", new_product.id, err);
            }
        }
    
        println!("All products have been saved to the database.");
        Ok(all_products)
    }
    
}
