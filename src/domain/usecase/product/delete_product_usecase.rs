use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::repository::product_repository::ProductRepository;

pub struct DeleteProductUseCase {
    pub repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

impl DeleteProductUseCase {
    pub fn new(repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: String) -> Result<(), String> {
        let repository = self.repository.write().await;
        let response = repository.delete(id).await;
        match response {
            Ok(_) => {
                log::info!("End request");
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to update product: {}", e);
                Err(e)
            }
        }
    }
}
