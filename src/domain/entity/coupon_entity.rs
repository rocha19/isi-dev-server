use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

use crate::domain::utils::{coupon_value_validate::validate_coupon_value, statics::COUPON_REGEX};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Type)]
#[sqlx(type_name = "coupon_discount_type")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CouponType {
    Fixed,
    Percent,
}

impl CouponType {}

impl fmt::Display for CouponType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CouponType::Percent => write!(f, "percent"),
            CouponType::Fixed => write!(f, "fixed"),
        }
    }
}

impl FromStr for CouponType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "percent" => Ok(CouponType::Percent),
            "fixed" => Ok(CouponType::Fixed),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct Coupon {
    pub id: Uuid,
    #[validate(regex = "COUPON_REGEX")]
    pub code: String,
    #[serde(rename = "type")]
    pub coupon_type: CouponType,
    #[validate(custom = "validate_coupon_value")]
    pub value: u64,
    pub one_shot: bool,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub uses_count: u32,
    pub max_uses: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateCoupon {
    #[validate(regex = "COUPON_REGEX")]
    pub code: String,
    #[serde(rename = "type")]
    pub coupon_type: CouponType,
    #[validate(custom = "validate_coupon_value")]
    pub value: u64,
    pub one_shot: bool,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub max_uses: Option<u32>,
}

impl Coupon {
    pub fn new(coupon: CreateCoupon) -> Self {
        Self {
            id: Uuid::new_v4(),
            code: coupon.code,
            coupon_type: coupon.coupon_type,
            value: coupon.value,
            one_shot: coupon.one_shot,
            valid_from: coupon.valid_from,
            valid_until: coupon.valid_until,
            uses_count: 0,
            max_uses: coupon.max_uses,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct UpdateCoupon {
    #[serde(rename = "type")]
    pub coupon_type: Option<CouponType>,
    pub value: Option<u64>,
    pub one_shot: Option<bool>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub max_uses: Option<u32>,
}

impl UpdateCoupon {
    pub fn new(coupon: UpdateCoupon) -> Self {
        Self {
            coupon_type: coupon.coupon_type,
            value: coupon.value,
            one_shot: coupon.one_shot,
            valid_from: coupon.valid_from,
            valid_until: coupon.valid_until,
            max_uses: coupon.max_uses,
        }
    }
}
