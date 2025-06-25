use crate::domain::{
    entity::product_entity::{CreateProduct, Product},
    repository::product_repository::ProductRepository,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CreateProductUseCase {
    pub repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

impl CreateProductUseCase {
    pub fn new(repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        name: String,
        description: Option<String>,
        stock: u32,
        price: u64,
    ) -> Result<Product, std::io::Error> {
        log::info!("Start request");

        let write_repository = self.repository.write().await;
        let product = CreateProduct::new(name, description, stock, price);

        let response = write_repository
            .create(product.clone())
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        log::info!("End request");
        Ok(response)
    }
}
