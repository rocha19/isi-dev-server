use crate::{
    application::usecase::patch_operation::PatchOperation,
    domain::{
        entity::product_entity::UpdateProduct, repository::product_repository::ProductRepository,
    },
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct UpdateProductUseCase {
    pub repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

impl UpdateProductUseCase {
    pub fn new(repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: String,
        name: Option<String>,
        description: Option<String>,
        stock: Option<u32>,
        price: Option<u64>,
    ) -> Result<Vec<PatchOperation>, String> {
        log::info!("Start request");
        let write_repository = self.repository.write().await;
        let product = UpdateProduct::new(
            name.clone(),
            description.clone(),
            stock.clone(),
            price.clone(),
            None,
        );

        let mut patches = Vec::new();

        if let Some(ref v) = name {
            patches.push(PatchOperation::replace("/name", v));
        }
        if let Some(ref v) = description {
            patches.push(PatchOperation::replace("/description", v));
        }
        if let Some(v) = stock {
            patches.push(PatchOperation::replace("/stock", v));
        }
        if let Some(v) = price {
            patches.push(PatchOperation::replace("/price", v));
        }

        let update_result = write_repository.update(id, product.clone()).await;

        match update_result {
            Ok(_) => {
                log::info!("End request");
                Ok(patches)
            }
            Err(e) => {
                log::error!("Failed to update product: {}", e);
                Err(e)
            }
        }
    }
}
