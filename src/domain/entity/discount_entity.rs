use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct ProductDiscount {
    pub id: Uuid,
    pub product_id: Uuid,
    pub coupon_id: Uuid,
    pub applied_at: DateTime<Utc>,
    pub removed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub stock: u32,
    pub is_out_of_stock: bool,
    pub price: u64,
    pub final_price: u64,
    pub discount: Option<ProductDiscountInfo>,
    pub has_coupon_applied: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ProductDiscountInfo {
    #[serde(rename = "type")]
    pub discount_type: String,
    pub value: u64,
    pub applied_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total_items: u64,
    pub total_pages: u32,
}
