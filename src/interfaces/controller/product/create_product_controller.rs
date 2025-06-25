use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{from_value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::product_repository::ProductRepository,
        usecase::product::create_product_usecase::CreateProductUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct CreateProductController {
    pub product_repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

#[derive(Deserialize, Debug)]
struct ProductDTO {
    name: String,
    description: Option<String>,
    stock: u32,
    price: u64,
}

#[async_trait]
impl GenericHandler for CreateProductController {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse {
        log::info!("Start create product request");

        let body = match request.body {
            Some(body) => body,
            None => {
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Missing product data"}),
                    binary: None,
                };
            }
        };

        if !body.is_object() {
            return AdapterResponse {
                status: StatusCode::BadRequest,
                data: json!({"error": "Invalid product data format"}),
                binary: None,
            };
        }

        let obj = body.as_object().unwrap();

        let mut missing_fields = Vec::new();
        if !obj.contains_key("name") {
            missing_fields.push("name");
        }
        if !obj.contains_key("stock") {
            missing_fields.push("stock");
        }
        if !obj.contains_key("price") {
            missing_fields.push("price");
        }

        if !missing_fields.is_empty() {
            return AdapterResponse {
                status: StatusCode::BadRequest,
                data: json!({"error": "Missing required fields", "fields": missing_fields}),
                binary: None,
            };
        }

        let product: ProductDTO = match from_value(body) {
            Ok(p) => p,
            Err(e) => {
                return AdapterResponse {
                    status: StatusCode::BadRequest,
                    data: json!({"error": "Invalid field types", "details": e.to_string()}),
                    binary: None,
                };
            }
        };

        let usecase = CreateProductUseCase::new(self.product_repository.clone());
        let response = usecase
            .execute(
                product.name,
                product.description,
                product.stock,
                product.price,
            )
            .await;

        match response {
            Ok(product) => {
                log::info!("Product created successfully");
                AdapterResponse {
                    status: StatusCode::Created,
                    data: serde_json::to_value(product)
                        .unwrap_or_else(|_| json!({"error": "Failed to serialize product"})),
                    binary: None,
                }
            }

            Err(e) => {
                let error_str = e.to_string();
                log::error!("Error creating product: {}", error_str);

                if error_str.contains("products_name_unique_idx")
                    || error_str.contains("duplicate key value")
                {
                    AdapterResponse {
                        status: StatusCode::Conflict,
                        data: json!({"error": "Product already exists"}),
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
