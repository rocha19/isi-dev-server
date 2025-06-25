use async_trait::async_trait;
use serde_json::{from_value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::coupon_repository::CouponRepository,
        usecase::coupon::create_coupon_usecase::CreateCouponUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct CreateCouponController {
    pub coupon_repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

#[derive(serde::Deserialize)]
struct CouponDTO {
    code: String,
    #[serde(rename = "type")]
    coupon_type: String,
    value: u64,
    one_shot: bool,
    valid_from: String,
    valid_until: String,
    max_uses: Option<u32>,
}

#[async_trait]
impl GenericHandler for CreateCouponController {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse {
        log::info!("Start request");

        let body = match request.body {
            Some(body) => body,
            None => {
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Missing coupon data"}),
                    binary: None,
                };
            }
        };

        if !body.is_object() {
            return AdapterResponse {
                status: StatusCode::BadRequest,
                data: json!({"error": "Invalid coupon data format"}),
                binary: None,
            };
        }

        let obj = body.as_object().unwrap();

        let mut missing_fields = Vec::new();
        if !obj.contains_key("code") {
            missing_fields.push("code");
        }
        if !obj.contains_key("type") {
            missing_fields.push("type");
        }
        if !obj.contains_key("value") {
            missing_fields.push("value");
        }
        if !obj.contains_key("one_shot") {
            missing_fields.push("one_shot");
        }
        if !obj.contains_key("valid_from") {
            missing_fields.push("valid_from");
        }
        if !obj.contains_key("valid_until") {
            missing_fields.push("valid_until");
        }

        if !missing_fields.is_empty() {
            return AdapterResponse {
                status: StatusCode::BadRequest,
                data: json!({"error": "Missing required fields", "fields": missing_fields}),
                binary: None,
            };
        }

        let coupon: CouponDTO = match from_value(body) {
            Ok(coupon) => coupon,
            Err(e) => {
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Invalid field types", "details": e.to_string()}),
                    binary: None,
                };
            }
        };

        let usecase = CreateCouponUseCase::new(self.coupon_repository.clone());

        let response = usecase
            .execute(
                coupon.code,
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
                log::warn!("Coupon created: {:#?}", coupon);
                let coupon_json = serde_json::to_value(coupon);
                log::info!("Product created successfully");
                match coupon_json {
                    Ok(json_value) => AdapterResponse {
                        status: StatusCode::Created,
                        data: json_value,
                        binary: None,
                    },
                    Err(_) => AdapterResponse {
                        status: StatusCode::InternalServerError,
                        data: json!({"error": "Failed to serialize coupon"}),
                        binary: None,
                    },
                }
            }
            Err(e) => {
                let error_str = e.to_string();
                if error_str.contains("already exists") {
                    AdapterResponse {
                        status: StatusCode::Conflict,
                        data: json!({"error": "Coupon already exists"}),
                        binary: None,
                    }
                } else if error_str.contains("inválida") {
                    AdapterResponse {
                        status: StatusCode::BadRequest,
                        data: json!({"error": error_str}),
                        binary: None,
                    }
                } else {
                    AdapterResponse {
                        status: StatusCode::InternalServerError,
                        data: json!({"error": "Internal server error"}),
                        binary: None,
                    }
                }
            }
        }
    }
}
