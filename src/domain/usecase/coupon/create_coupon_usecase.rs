use crate::domain::{
    entity::coupon_entity::{Coupon, CouponType, CreateCoupon},
    repository::coupon_repository::CouponRepository,
};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CreateCouponUseCase {
    pub repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

impl CreateCouponUseCase {
    pub fn new(repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        code: String,
        coupon_type: String,
        value: u64,
        one_shot: bool,
        valid_from: String,
        valid_until: String,
        max_uses: Option<u32>,
    ) -> Result<Coupon, std::io::Error> {
        log::info!("Start request");

        let coupon_type: CouponType = CouponType::from_str(&coupon_type).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Tipo de cupom inválido")
        })?;

        let parse_date = |s: &str| -> Result<DateTime<Utc>, std::io::Error> {
            DateTime::parse_from_rfc3339(s)
                .or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
                .or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S"))
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Formato de data inválido: {}. Use YYYY-MM-DDTHH:MM:SSZ", e),
                    )
                })
        };

        let valid_from = parse_date(&valid_from)?;
        let valid_until = parse_date(&valid_until)?;

        let coupon = CreateCoupon {
            code,
            coupon_type,
            value,
            one_shot,
            valid_from,
            valid_until,
            max_uses,
        };

        let write_repository = self.repository.write().await;
        let response = write_repository.create(coupon).await.map_err(|e| {
            log::error!("Erro ao criar cupom: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

        log::info!("End request");
        Ok(response)
    }
}
