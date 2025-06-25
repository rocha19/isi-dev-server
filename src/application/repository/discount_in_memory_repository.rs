use crate::domain::entity::{
    discount_entity::{DiscountDiscount, PaginatedResponse, PaginationMeta},
    product_entity::{CreateDiscount, Discount, UpdateDiscount},
};
use crate::domain::repository::product_repository::DiscountRepository;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InMemoryDiscountRepository {
    products: Arc<RwLock<HashMap<String, Discount>>>,
    discounts: Arc<RwLock<HashMap<String, DiscountDiscount>>>,
}

impl InMemoryDiscountRepository {
    pub fn new() -> Self {
        Self {
            products: Arc::new(RwLock::new(HashMap::new())),
            discounts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

unsafe impl Send for InMemoryDiscountRepository {}
unsafe impl Sync for InMemoryDiscountRepository {}

#[async_trait]
impl DiscountRepository for InMemoryDiscountRepository {
    async fn apply_coupon(&self, product_id: Uuid, coupon_id: Uuid) -> Result<(), String> {
        let mut applications = self.coupon_applications.write().await;

        applications.retain(|app| {
            app.product_id != product_id
                || (app.product_id == product_id && app.removed_at.is_some())
        });

        let new_application = DiscountCouponApplication {
            id: Uuid::new_v4(),
            product_id,
            coupon_id,
            applied_at: Utc::now(),
            removed_at: None,
        };

        applications.push(new_application);
        Ok(())
    }

    async fn remove_coupon(&self, product_id: Uuid) -> Result<(), String> {
        let mut applications = self.coupon_applications.write().await;
        let now = Utc::now();

        if let Some(app) = applications
            .iter_mut()
            .find(|app| app.product_id == product_id && app.removed_at.is_none())
        {
            app.removed_at = Some(now);
            Ok(())
        } else {
            Err("No active coupon found for this product".to_string())
        }
    }

    async fn get_active_coupon(&self, product_id: Uuid) -> Option<Uuid> {
        let applications = self.coupon_applications.read().await;
        applications
            .iter()
            .find(|app| app.product_id == product_id && app.removed_at.is_none())
            .map(|app| app.coupon_id)
    }

    async fn has_discount(&self, product_id: String) -> bool {
        if let Ok(uuid) = Uuid::parse_str(&product_id) {
            self.get_active_coupon(uuid).await.is_some()
        } else {
            false
        }
    }

    async fn has_discount(&self, product_id: String) -> bool {
        let discounts = self.discounts.read().await;
        if let Some(discount) = discounts.get(&product_id) {
            discount.applied_at > Utc::now() - chrono::Duration::days(30)
        } else {
            false
        }
    }
}
