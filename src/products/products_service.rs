use crate::common::errors::ApiError;
use super::products_repository::ProductRepository;
use super::entities::products_entity::{NewProduct, Product};

pub struct ProductService {
    repository: ProductRepository,
}

impl ProductService {
    pub fn new(repository: ProductRepository) -> Self {
        Self { repository }
    }

    pub fn upsert_product(&mut self, product: &NewProduct) -> Result<String, ApiError> {
        self.repository
            .upsert_product(product)
            .map_err(|err| {
                eprintln!("Failed to insert or update product: {:?}", err);
                ApiError::InternalError("Failed to insert or update product".to_string())
            })
    }

    pub fn get_many_by_ids(&mut self, product_ids: Vec<&str>) -> Result<Vec<Product>, ApiError> {
        self.repository
            .get_many_by_ids(product_ids)
            .map_err(|err| {
                eprintln!("Error getting products: {:?}", err);
                if err.to_string().contains("No products found") {
                    ApiError::NotFound("No products found for the given IDs".to_string())
                } else {
                    ApiError::InternalError("Failed to fetch products".to_string())
                }
            })
    }

}
