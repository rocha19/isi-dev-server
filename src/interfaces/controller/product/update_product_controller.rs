use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, from_value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::product_repository::ProductRepository,
        usecase::product::update_product_usecase::UpdateProductUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Deserialize, Default, Debug)]
struct ProductDTO {
    name: Option<String>,
    description: Option<String>,
    stock: Option<u32>,
    price: Option<u64>,
}

#[derive(Clone)]
pub struct UpdateProductController {
    pub product_repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for UpdateProductController {
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

        let product: ProductDTO = match request.body {
            Some(b) => match from_value(b) {
                Ok(prod) => prod,
                Err(_) => {
                    return AdapterResponse {
                        status: StatusCode::BadRequest,
                        data: json!({"error": "Invalid body"}),
                        binary: None,
                    };
                }
            },
            None => {
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Missing body"}),
                    binary: None,
                };
            }
        };

        let usecase = UpdateProductUseCase::new(self.product_repository.clone());
        let response = usecase
            .execute(
                id,
                product.name,
                product.description,
                product.stock,
                product.price,
            )
            .await;

        match response {
            Ok(product) => {
                let product_json = serde_json::to_value(product);
                log::info!("End request");
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
                data: json!({ "error": e }),
                binary: None,
            },
        }
    }
}
