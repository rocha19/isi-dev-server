use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::entity::{
    coupon_entity::{Coupon, CreateCoupon, UpdateCoupon},
    discount_entity::PaginatedResponse,
};

#[async_trait]
pub trait CouponRepository: Send + Sync {
    async fn create(&self, coupon: CreateCoupon) -> Result<Coupon, String>;
    async fn find(&self, code: &str) -> Result<Coupon, String>;
    async fn find_all(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        search: Option<String>,
        valid_from: Option<DateTime<Utc>>,
        valid_until: Option<DateTime<Utc>>,
        is_active: Option<bool>,
    ) -> Result<PaginatedResponse<Coupon>, String>;
    async fn update(&self, id: String, data: UpdateCoupon) -> Result<Coupon, String>;
    async fn delete(&self, id: String) -> Result<(), String>;
    async fn find_valid_coupon_by_code(&self, code: &str) -> Result<Coupon, String>;
    async fn increment_uses(&self, coupon_id: String);
}
