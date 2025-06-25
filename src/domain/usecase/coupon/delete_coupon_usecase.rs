use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::repository::coupon_repository::CouponRepository;

pub struct DeleteCouponUseCase {
    pub repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

impl DeleteCouponUseCase {
    pub fn new(repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, code: String) -> Result<(), String> {
        let repository = self.repository.write().await;
        let response = repository.delete(code).await;
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
