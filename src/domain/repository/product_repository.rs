use async_trait::async_trait;

use crate::domain::entity::{
    discount_entity::PaginatedResponse,
    product_entity::{CreateProduct, Product, UpdateProduct},
};

#[allow(dead_code)]
#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find(&self, id: String) -> Result<Product, String>;
    async fn find_all(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        search: String,
        min_price: u32,
        max_price: u32,
        has_discount: bool,
    ) -> Result<PaginatedResponse<Product>, String>;
    async fn create(&self, product: CreateProduct) -> Result<Product, String>;
    async fn update(&self, id: String, new_product: UpdateProduct) -> Result<Product, String>;
    async fn delete(&self, id: String) -> Result<(), String>;
    async fn has_discount(&self, product_id: String) -> bool;
}
