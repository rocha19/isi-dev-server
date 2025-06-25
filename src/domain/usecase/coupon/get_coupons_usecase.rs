use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    entity::{coupon_entity::Coupon, discount_entity::PaginatedResponse},
    repository::coupon_repository::CouponRepository,
};

pub struct GetAllCouponsUseCase {
    pub repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

impl GetAllCouponsUseCase {
    pub fn new(repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        page: u32,
        limit: u32,
        search: String,
        valid_from: String,
        valid_until: String,
        is_active: bool,
    ) -> Result<PaginatedResponse<Coupon>, String> {
        log::info!("Start request");

        let page = Some(page);
        let limit = Some(limit);
        let search = if search.is_empty() {
            None
        } else {
            Some(search)
        };
        let is_active = Some(is_active);

        let valid_from = if valid_from.is_empty() {
            None
        } else {
            Some(
                valid_from
                    .parse::<DateTime<Utc>>()
                    .map_err(|e| format!("valid_from inválida: {}", e))?,
            )
        };

        let valid_until = if valid_until.is_empty() {
            None
        } else {
            Some(
                valid_until
                    .parse::<DateTime<Utc>>()
                    .map_err(|e| format!("valid_until inválida: {}", e))?,
            )
        };

        let repository = self.repository.read().await;
        let coupons = repository
            .find_all(page, limit, search, valid_from, valid_until, is_active)
            .await
            .map_err(|e| e.to_string())?;

        log::info!("End request");
        Ok(coupons)
    }
}
