use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::{
            discount_repository::DiscountRepository, product_repository::ProductRepository,
        },
        usecase::product::get_product_usecase::GetProductUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct GetProductController {
    pub product_repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
    pub discount_repository: Arc<RwLock<dyn DiscountRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for GetProductController {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse {
        log::info!("Start request");
        let id = match request.params.and_then(|p| p.get("id").cloned()) {
            Some(Value::String(id)) => id,
            _ => {
                log::error!("Missing or invalid 'id' parameter in request");
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Missing or invalid product ID"}),
                    binary: None,
                };
            }
        };

        let product_repository = self.product_repository.clone();
        let discount_repository = self.discount_repository.clone();

        let usecase = GetProductUseCase::new(product_repository, discount_repository);

        let response = usecase.execute(id).await;
        log::info!("End request");

        match response {
            Ok(product) => {
                let product_json = serde_json::to_value(product);
                match product_json {
                    Ok(json_value) => AdapterResponse {
                        status: StatusCode::Ok,
                        data: json_value,
                        binary: None,
                    },
                    Err(_) => AdapterResponse {
                        status: StatusCode::InternalServerError,
                        data: Value::String("Failed to serialize product".to_string()),
                        binary: None,
                    },
                }
            }
            Err(e) => AdapterResponse {
                status: StatusCode::NotFound,
                data: json!({"error": format!("Product not found: {}", e)}),
                binary: None,
            },
        }
    }
}
