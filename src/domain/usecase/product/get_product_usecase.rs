use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    entity::discount_entity::{ProductDiscountInfo, ProductResponse},
    repository::{discount_repository::DiscountRepository, product_repository::ProductRepository},
};

pub struct GetProductUseCase {
    pub product_repo: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
    pub discount_repo: Arc<RwLock<dyn DiscountRepository + Send + Sync>>,
}

impl GetProductUseCase {
    pub fn new(
        product_repo: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
        discount_repo: Arc<RwLock<dyn DiscountRepository + Send + Sync>>,
    ) -> Self {
        Self {
            product_repo,
            discount_repo,
        }
    }

    pub async fn execute(&self, id: String) -> Result<ProductResponse, String> {
        let product_repo = self.product_repo.read().await;
        let discount_repo = self.discount_repo.read().await;

        let product = product_repo.find(id).await?;

        let discount_info = discount_repo
            .find_active_discount(product.id.to_string())
            .await?
            .map(|(discount, coupon)| ProductDiscountInfo {
                discount_type: coupon.coupon_type.to_string(),
                value: coupon.value,
                applied_at: discount.applied_at,
            });

        log::warn!("{:?}", product);

        let has_coupon_applied = discount_info.is_some();

        let final_price = if let Some(d) = &discount_info {
            match d.discount_type.as_str() {
                "percent" => {
                    let discount_amount = (product.price * d.value) / 100;
                    product.price.saturating_sub(discount_amount).max(1)
                }
                "fixed" => product.price.saturating_sub(d.value).max(1),
                _ => product.price,
            }
        } else {
            product.price
        };

        Ok(ProductResponse {
            id: product.id,
            name: product.name,
            description: product.description,
            stock: product.stock,
            is_out_of_stock: product.stock == 0,
            price: product.price,
            final_price,
            discount: discount_info,
            has_coupon_applied,
            created_at: product.created_at,
            updated_at: product.updated_at,
        })
    }
}
