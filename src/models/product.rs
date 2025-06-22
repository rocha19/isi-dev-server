// src/models/product.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::utils::normalize_name;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Product {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    #[serde(deserialize_with = "normalize_name")]
    pub name: String,
    #[validate(length(max = 300))]
    pub description: Option<String>,
    #[validate(range(min = 0, max = 999999))]
    pub stock: i32,
    pub price: i64,
    pub original_price: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 1, max = 100))]
    #[serde(deserialize_with = "normalize_name")]
    pub name: String,
    #[validate(length(max = 300))]
    pub description: Option<String>,
    #[validate(range(min = 0, max = 999999))]
    pub stock: i32,
    #[validate(range(min = 1))]
    pub price: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProduct {
    #[validate(length(min = 1, max = 100))]
    #[serde(default, deserialize_with = "normalize_opt_name")]
    pub name: Option<String>,
    #[validate(length(max = 300))]
    pub description: Option<String>,
    #[validate(range(min = 0, max = 999999))]
    pub stock: Option<i32>,
    #[validate(range(min = 1))]
    pub price: Option<i64>,
}

pub fn normalize_opt_name<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let s = s.trim().to_lowercase();
            let s = s.replace(|c: char| c.is_whitespace(), " ");
            Ok(Some(s))
        }
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ApplyPercentDiscount {
    #[validate(range(min = 1, max = 80))]
    pub percentage: i32,
}

#[derive(Debug, Deserialize)]
pub struct ApplyCoupon {
    pub code: String,
}
