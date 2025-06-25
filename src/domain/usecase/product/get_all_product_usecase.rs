use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    entity::{discount_entity::PaginatedResponse, product_entity::Product},
    repository::product_repository::ProductRepository,
};

pub struct GetAllProductsUseCase {
    pub repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

impl GetAllProductsUseCase {
    pub fn new(repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        page: u32,
        limit: u32,
        search: String,
        min_price: u32,
        max_price: u32,
        has_discount: bool,
    ) -> Result<PaginatedResponse<Product>, String> {
        log::info!("Start request");
        let repository = self.repository.read().await;
        let works = repository
            .find_all(
                Some(page),
                Some(limit),
                search,
                min_price,
                max_price,
                has_discount,
            )
            .await?;
        log::info!("End request");
        Ok(works)
    }
}
