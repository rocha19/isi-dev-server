use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::utils::normalize_name::normalize_name;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Product {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    #[serde(deserialize_with = "normalize_name")]
    pub name: String,
    #[validate(length(max = 300))]
    pub description: Option<String>,
    #[validate(range(min = 0, max = 999999))]
    pub stock: u32,
    pub price: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 1, max = 100))]
    #[serde(deserialize_with = "normalize_name")]
    pub name: String,
    #[validate(length(max = 300))]
    pub description: Option<String>,
    #[validate(range(min = 0, max = 999999))]
    pub stock: u32,
    #[validate(range(min = 1))]
    pub price: u64,
}

impl CreateProduct {
    pub fn new(name: String, description: Option<String>, stock: u32, price: u64) -> Self {
        CreateProduct {
            name,
            description,
            stock,
            price,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateProduct {
    #[validate(length(min = 1, max = 100))]
    #[serde(default, deserialize_with = "normalize_opt_name")]
    pub name: Option<String>,
    #[validate(length(max = 300))]
    pub description: Option<String>,
    #[validate(range(min = 0, max = 999999))]
    pub stock: Option<u32>,
    #[validate(range(min = 1))]
    pub price: Option<u64>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UpdateProduct {
    pub fn new(
        name: Option<String>,
        description: Option<String>,
        stock: Option<u32>,
        price: Option<u64>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        UpdateProduct {
            name,
            description,
            stock,
            price,
            deleted_at,
        }
    }
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

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ApplyPercentDiscount {
    #[validate(range(min = 1, max = 80))]
    pub percentage: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyCoupon {
    pub code: String,
}
