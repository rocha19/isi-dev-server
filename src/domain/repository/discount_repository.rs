use async_trait::async_trait;

use crate::domain::entity::{coupon_entity::Coupon, discount_entity::ProductDiscount};

#[async_trait]
pub trait DiscountRepository: Send + Sync {
    async fn apply_coupon(
        &self,
        product_id: String,
        coupon_id: String,
    ) -> Result<ProductDiscount, String>;

    async fn remove_coupon(
        &self,
        product_id: String,
        coupon_id: String,
    ) -> Result<ProductDiscount, String>;

    async fn find_active_discount(
        &self,
        product_id: String,
    ) -> Result<Option<(ProductDiscount, Coupon)>, String>;
}
