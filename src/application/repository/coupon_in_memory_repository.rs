use crate::domain::entity::discount_entity::PaginatedResponse;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::{
    entity::{
        coupon_entity::{Coupon, CreateCoupon, UpdateCoupon},
        discount_entity::PaginationMeta,
    },
    repository::coupon_repository::CouponRepository,
};

#[derive(Debug, Default)]
pub struct InMemoryCouponRepository {
    coupons: Arc<RwLock<Vec<Coupon>>>,
}

impl InMemoryCouponRepository {
    pub fn new() -> Self {
        Self {
            coupons: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl CouponRepository for InMemoryCouponRepository {
    async fn create(&self, coupon: CreateCoupon) -> Result<Coupon, String> {
        let mut coupons = self.coupons.write().await;

        if coupons
            .iter()
            .any(|p| p.deleted_at.is_none() && p.code.to_lowercase() == coupon.code.to_lowercase())
        {
            return Err("Coupon already exists".to_string());
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        let new_coupon = Coupon {
            id,
            code: coupon.code.clone(),
            coupon_type: coupon.coupon_type,
            value: coupon.value,
            one_shot: coupon.one_shot,
            valid_from: coupon.valid_from,
            valid_until: coupon.valid_until,
            uses_count: 0,
            max_uses: coupon.max_uses,
            created_at: now,
            updated_at: Some(now),
            deleted_at: None,
        };

        coupons.push(new_coupon.clone());

        Ok(new_coupon)
    }

    async fn find(&self, code: &str) -> Result<Coupon, String> {
        let coupons = self.coupons.read().await;
        coupons
            .iter()
            .find(|c| c.code == code && c.deleted_at.is_none())
            .cloned()
            .ok_or_else(|| "Coupon not found".to_string())
    }

    async fn find_all(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        search: Option<String>,
        valid_from: Option<DateTime<Utc>>,
        valid_until: Option<DateTime<Utc>>,
        is_active: Option<bool>,
    ) -> Result<PaginatedResponse<Coupon>, String> {
        let coupons = self.coupons.read().await;
        let now = Utc::now();

        let search_lower = search.as_ref().map(|s| s.to_lowercase());

        let filtered_coupons: Vec<Coupon> = coupons
            .iter()
            .filter(|c| c.deleted_at.is_none())
            .filter(|c| {
                if let Some(search_str) = &search_lower {
                    if !c.code.to_lowercase().contains(search_str) {
                        return false;
                    }
                }
                true
            })
            .filter(|c| {
                if let Some(vf) = valid_from {
                    if c.valid_from < vf {
                        return false;
                    }
                }
                true
            })
            .filter(|c| {
                if let Some(vu) = valid_until {
                    if c.valid_until > vu {
                        return false;
                    }
                }
                true
            })
            .filter(|c| {
                if let Some(active) = is_active {
                    let currently_active = c.valid_from <= now && now <= c.valid_until;
                    if active != currently_active {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        let mut sorted_coupons = filtered_coupons;
        sorted_coupons.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);

        let total_items = sorted_coupons.len() as u64;
        let total_pages = (total_items as f64 / limit as f64).ceil() as u32;

        let start_index = ((page - 1) * limit) as usize;
        let end_index = std::cmp::min(start_index + limit as usize, sorted_coupons.len());

        let paginated_data = if start_index < sorted_coupons.len() {
            sorted_coupons[start_index..end_index].to_vec()
        } else {
            Vec::new()
        };

        Ok(PaginatedResponse {
            data: paginated_data,
            meta: PaginationMeta {
                page,
                limit,
                total_items,
                total_pages,
            },
        })
    }

    async fn update(&self, code: String, data: UpdateCoupon) -> Result<Coupon, String> {
        let mut coupons = self.coupons.write().await;
        if let Some(coupon) = coupons
            .iter_mut()
            .find(|c| c.code == code && c.deleted_at.is_none())
        {
            if let Some(coupon_type) = data.coupon_type {
                coupon.coupon_type = coupon_type;
            }
            if let Some(value) = data.value {
                coupon.value = value;
            }
            if let Some(one_shot) = data.one_shot {
                coupon.one_shot = one_shot;
            }
            if let Some(valid_from) = data.valid_from {
                coupon.valid_from = valid_from;
            }
            if let Some(valid_until) = data.valid_until {
                coupon.valid_until = valid_until;
            }
            if let Some(max_uses) = data.max_uses {
                coupon.max_uses = Some(max_uses);
            }
            coupon.updated_at = Some(Utc::now());
            Ok(coupon.clone())
        } else {
            Err("Coupon not found".to_string())
        }
    }

    async fn delete(&self, code: String) -> Result<(), String> {
        let mut coupons = self.coupons.write().await;
        if let Some(coupon) = coupons
            .iter_mut()
            .find(|c| c.code == code && c.deleted_at.is_none())
        {
            coupon.deleted_at = Some(Utc::now());
            Ok(())
        } else {
            Err("Coupon not found".to_string())
        }
    }

    async fn find_valid_coupon_by_code(&self, code: &str) -> Result<Coupon, String> {
        let coupons = self.coupons.read().await;
        let now = Utc::now();

        coupons
            .iter()
            .find(|c| {
                c.code == code
                    && c.valid_from <= now
                    && c.valid_until >= now
                    && c.deleted_at.is_none()
                    && c.max_uses.map_or(true, |max| c.uses_count < max)
            })
            .cloned()
            .ok_or_else(|| "Valid coupon not found".to_string())
    }

    async fn increment_uses(&self, coupon_id: String) {
        if let Ok(uuid) = Uuid::parse_str(&coupon_id) {
            let mut coupons = self.coupons.write().await;
            if let Some(coupon) = coupons
                .iter_mut()
                .find(|c| c.id == uuid && c.deleted_at.is_none())
            {
                coupon.uses_count += 1;
            }
        }
    }
}
