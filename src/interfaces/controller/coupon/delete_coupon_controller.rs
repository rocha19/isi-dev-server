use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::coupon_repository::CouponRepository,
        usecase::coupon::delete_coupon_usecase::DeleteCouponUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct DeleteCouponController {
    pub coupon_repository: Arc<RwLock<dyn CouponRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for DeleteCouponController {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse {
        log::info!("Start request");
        let code = match request.params.and_then(|p| p.get("code").cloned()) {
            Some(Value::String(code)) => code,
            _ => {
                log::error!("Missing or invalid 'code'  parameter in request");
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Missing or invalid coupon code"}),
                    binary: None,
                };
            }
        };

        let repository = self.coupon_repository.clone();
        let usecase = DeleteCouponUseCase::new(repository);

        let response = usecase.execute(code).await;
        log::info!("End request");

        match response {
            Ok(_) => AdapterResponse {
                status: StatusCode::NoContent,
                data: Value::Null,
                binary: None,
            },
            Err(e) => AdapterResponse {
                status: StatusCode::NotFound,
                data: json!({"error": e}),
                binary: None,
            },
        }
    }
}
