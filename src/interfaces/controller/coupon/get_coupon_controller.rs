use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::coupon_repository::CouponRepository,
        usecase::coupon::get_coupon_usecase::GetCouponUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct GetCouponController {
    pub coupon_repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for GetCouponController {
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

        log::warn!("Code: {}", code);

        let repo = self.coupon_repository.clone();
        let usecase = GetCouponUseCase::new(repo);

        let response = usecase.execute(code).await;
        log::info!("End request");
        log::info!("Response: {:#?}", response);

        match response {
            Ok(coupon) => {
                let coupon_json = serde_json::to_value(coupon);
                log::info!("Coupon: {:#?}", coupon_json);
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
