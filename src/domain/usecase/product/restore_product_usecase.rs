use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    entity::product_entity::UpdateProduct, repository::product_repository::ProductRepository,
};

pub struct RestoreProductUseCase {
    pub repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

impl RestoreProductUseCase {
    pub fn new(repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: String) -> Result<(), String> {
        let repository = self.repository.write().await;
        let product = UpdateProduct::new(None, None, None, None, Some(Utc::now()));

        let _ = repository.update(id, product).await;
        Ok(())
    }
}
