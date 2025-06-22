// src/models/coupon.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::utils::{COUPON_REGEX, validate_coupon_value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CouponType {
    Fixed,
    Percent,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Coupon {
    pub id: Uuid,
    #[validate(regex = "COUPON_REGEX")]
    pub code: String,
    #[serde(rename = "type")]
    pub coupon_type: CouponType,
    #[validate(custom = "validate_coupon_value")]
    pub value: i64,
    pub one_shot: bool,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub uses_count: i32,
    pub max_uses: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCoupon {
    #[validate(regex = "COUPON_REGEX")]
    pub code: String,
    pub coupon_type: CouponType,
    #[validate(custom = "validate_coupon_value")]
    pub value: i64,
    pub one_shot: bool,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub max_uses: Option<i32>,
}
