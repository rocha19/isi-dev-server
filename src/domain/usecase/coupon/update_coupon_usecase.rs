use crate::{
    application::usecase::patch_operation::PatchOperation,
    domain::{
        entity::coupon_entity::{CouponType, UpdateCoupon},
        repository::coupon_repository::CouponRepository,
    },
};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct UpdateCouponUseCase {
    pub repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

impl UpdateCouponUseCase {
    pub fn new(repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        code: String,
        coupon_type: Option<String>,
        value: Option<u64>,
        one_shot: Option<bool>,
        valid_from: Option<String>,
        valid_until: Option<String>,
        max_uses: Option<u32>,
    ) -> Result<Vec<PatchOperation>, String> {
        log::info!("Start request: UpdateCouponUseCase");

        let coupon_type = match coupon_type {
            Some(ct) => match CouponType::from_str(&ct) {
                Ok(val) => Some(val),
                Err(_) => return Err("Tipo de cupom inválido".to_string()),
            },
            None => None,
        };

        let valid_from = match valid_from {
            Some(vf) => match vf.parse::<DateTime<Utc>>() {
                Ok(dt) => Some(dt),
                Err(e) => return Err(format!("valid_from inválida: {}", e)),
            },
            None => None,
        };

        let valid_until = match valid_until {
            Some(vu) => match vu.parse::<DateTime<Utc>>() {
                Ok(dt) => Some(dt),
                Err(e) => return Err(format!("valid_until inválida: {}", e)),
            },
            None => None,
        };

        let update_data = UpdateCoupon {
            coupon_type,
            value,
            one_shot,
            valid_from,
            valid_until,
            max_uses,
        };

        let mut patches = Vec::new();

        if update_data.coupon_type.is_some() {
            patches.push(PatchOperation::replace(
                "/coupon_type",
                update_data.coupon_type.as_ref().unwrap(),
            ));
        }
        if update_data.value.is_some() {
            patches.push(PatchOperation::replace(
                "/value",
                update_data.value.as_ref().unwrap(),
            ));
        }
        if update_data.one_shot.is_some() {
            patches.push(PatchOperation::replace(
                "/one_shot",
                update_data.one_shot.as_ref().unwrap(),
            ));
        }
        if update_data.valid_from.is_some() {
            patches.push(PatchOperation::replace(
                "/valid_from",
                update_data.valid_from.as_ref().unwrap(),
            ));
        }
        if update_data.valid_until.is_some() {
            patches.push(PatchOperation::replace(
                "/valid_until",
                update_data.valid_until.as_ref().unwrap(),
            ));
        }
        if update_data.max_uses.is_some() {
            patches.push(PatchOperation::replace(
                "/max_uses",
                update_data.max_uses.as_ref().unwrap(),
            ));
        }

        let write_repository = self.repository.write().await;

        let update_result = write_repository.update(code, update_data).await;

        log::info!("End request: UpdateCouponUseCase");
        match update_result {
            Ok(_) => {
                log::info!("End request");
                Ok(patches)
            }
            Err(e) => {
                log::error!("Failed to update product: {}", e);
                Err(e)
            }
        }
    }
}
