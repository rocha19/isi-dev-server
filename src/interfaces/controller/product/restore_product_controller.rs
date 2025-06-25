use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::product_repository::ProductRepository,
        usecase::product::restore_product_usecase::RestoreProductUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct RestoreProductController {
    pub product_repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for RestoreProductController {
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

        let repository = self.product_repository.clone();
        let usecase = RestoreProductUseCase::new(repository);

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
