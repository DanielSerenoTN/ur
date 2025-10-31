use anyhow::Result;
use crate::{common::errors::ApiError, http::zoho::{ZohoMapAccess, ZohoProduct}};



#[async_trait::async_trait]
pub trait ZohoServiceTrait: Send + Sync {
    async fn get_access_token(&self) -> Result<String, ApiError>;
    async fn get_products_by_ids(&self, product_ids: Vec<&str>) -> Result<Vec<ZohoProduct>, ApiError>;
    async fn fetch_all_products_from_zoho(&self) -> Result<Vec<ZohoProduct>, ApiError>;
}
