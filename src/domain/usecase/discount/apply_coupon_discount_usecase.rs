use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::repository::discount_repository::DiscountRepository;

pub struct ApplyCouponDiscountUseCase {
    pub repository: Arc<RwLock<dyn DiscountRepository + Send + Sync>>,
}

impl ApplyCouponDiscountUseCase {
    pub fn new(repository: Arc<RwLock<dyn DiscountRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, product_id: String, coupon_code: String) -> Result<(), String> {
        let repository = self.repository.write().await;
        let response = repository.apply_coupon(product_id, coupon_code).await;
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
