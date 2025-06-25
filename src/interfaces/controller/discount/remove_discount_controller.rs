use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, from_value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::discount_repository::DiscountRepository,
        usecase::discount::remove_discount_usecase::RemoveDiscountUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Deserialize, Default, Debug)]
struct DiscountDTO {
    code: String,
}
#[derive(Clone)]
pub struct RemoveDiscountController {
    pub discount_repository: Arc<RwLock<dyn DiscountRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for RemoveDiscountController {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse {
        log::info!("Start request");
        let product_id = match request.params.and_then(|p| p.get("product_id").cloned()) {
            Some(Value::String(product_id)) => product_id,
            _ => {
                log::error!("Missing or invalid 'product_id' parameter in request");
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Missing or invalid product product_id"}),
                    binary: None,
                };
            }
        };
        let discount: DiscountDTO = from_value(request.body.unwrap()).expect("Invalid User data");

        let repository = self.discount_repository.clone();
        let usecase = RemoveDiscountUseCase::new(repository);

        let response = usecase.execute(product_id, discount.code).await;
        log::info!("End request");

        match response {
            Ok(()) => AdapterResponse {
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
