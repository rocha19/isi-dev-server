use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, from_value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::coupon_repository::CouponRepository,
        usecase::coupon::update_coupon_usecase::UpdateCouponUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Deserialize, Default, Debug)]
struct CouponDTO {
    #[serde(default)]
    coupon_type: Option<String>,
    #[serde(default)]
    value: Option<u64>,
    #[serde(default)]
    one_shot: Option<bool>,
    #[serde(default)]
    valid_from: Option<String>,
    #[serde(default)]
    valid_until: Option<String>,
    #[serde(default)]
    max_uses: Option<u32>,
}

#[derive(Clone)]
pub struct UpdateCouponController {
    pub coupon_repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for UpdateCouponController {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse {
        log::info!("Start request");
        let code = match request.params.and_then(|p| p.get("code").cloned()) {
            Some(Value::String(code)) => code,
            _ => {
                log::error!("Missing or invalid 'code' parameter in request");
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Missing or invalid coupon code"}),
                    binary: None,
                };
            }
        };
        let body = match request.body {
            Some(b) => b,
            None => {
                log::error!("Missing request body");
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Missing request body"}),
                    binary: None,
                };
            }
        };

        let coupon: CouponDTO = match from_value(body) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Invalid JSON body: {}", e);
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": format!("Invalid body: {}", e)}),
                    binary: None,
                };
            }
        };

        let usecase = UpdateCouponUseCase::new(self.coupon_repository.clone());
        let response = usecase
            .execute(
                code,
                coupon.coupon_type,
                coupon.value,
                coupon.one_shot,
                coupon.valid_from,
                coupon.valid_until,
                coupon.max_uses,
            )
            .await;
        match response {
            Ok(coupon) => {
                let coupon_json = serde_json::to_value(coupon);
                log::info!("End request");
                match coupon_json {
                    Ok(json_value) => AdapterResponse {
                        status: StatusCode::Ok,
                        data: json_value,
                        binary: None,
                    },
                    Err(_) => AdapterResponse {
                        status: StatusCode::InternalServerError,
                        data: Value::String("Failed to serialize coupon".to_string()),
                        binary: None,
                    },
                }
            }
            Err(e) => AdapterResponse {
                status: StatusCode::NotFound,
                data: json!({"error": e}),
                binary: None,
            },
        }
    }
}
