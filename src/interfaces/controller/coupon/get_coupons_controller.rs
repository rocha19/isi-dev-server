use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::coupon_repository::CouponRepository,
        usecase::coupon::get_coupons_usecase::GetAllCouponsUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct GetAllCouponsController {
    pub coupon_repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for GetAllCouponsController {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse {
        log::info!("Start request");
        fn get_param<T: serde::de::DeserializeOwned>(query: &Value, key: &str, default: T) -> T {
            query
                .get(key)
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or(default)
        }

        let query = request.query.unwrap_or_else(|| json!({}));

        let page: u32 = get_param(&query, "page", 1);
        let limit: u32 = get_param(&query, "limit", 10);
        let search: String = get_param(&query, "search", "".to_string());
        let valid_from: String = get_param(&query, "valid_from", "".to_string());
        let valid_until: String = get_param(&query, "valid_until", "".to_string());
        let is_active: bool = get_param(&query, "is_active", false);

        let repo = self.coupon_repository.clone();
        let usecase = GetAllCouponsUseCase::new(repo);

        let response = usecase
            .execute(page, limit, search, valid_from, valid_until, is_active)
            .await;

        log::info!("End request");

        match response {
            Ok(coupons) => {
                let coupons_json = serde_json::to_value(coupons);
                match coupons_json {
                    Ok(json_value) => AdapterResponse {
                        status: StatusCode::Ok,
                        data: json_value,
                        binary: None,
                    },
                    Err(_) => AdapterResponse {
                        status: StatusCode::InternalServerError,
                        data: Value::String("Failed to serialize coupons".to_string()),
                        binary: None,
                    },
                }
            }
            Err(e) => AdapterResponse {
                status: StatusCode::NotFound,
                data: json!({"error": format!("Failed to fetch coupons: {}", e)}),
                binary: None,
            },
        }
    }
}
