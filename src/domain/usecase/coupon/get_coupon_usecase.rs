use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    entity::coupon_entity::Coupon, repository::coupon_repository::CouponRepository,
};

pub struct GetCouponUseCase {
    pub repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

impl GetCouponUseCase {
    pub fn new(repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, code: String) -> Result<Coupon, String> {
        let repository = self.repository.read().await;
        let coupon = repository.find(code.as_str()).await?;
        Ok(coupon)
    }
}
