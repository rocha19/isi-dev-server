use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::product_repository::ProductRepository,
        usecase::product::get_all_product_usecase::GetAllProductsUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct GetAllProductsController {
    pub product_repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for GetAllProductsController {
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
        let min_price: u32 = get_param(&query, "min_price", 0);
        let max_price: u32 = get_param(&query, "max_price", u32::MAX);
        let has_discount: bool = get_param(&query, "has_discount", false);

        let repo = self.product_repository.clone();
        let usecase = GetAllProductsUseCase::new(repo);

        let response = usecase
            .execute(page, limit, search, min_price, max_price, has_discount)
            .await;

        log::info!("End request");

        match response {
            Ok(products) => {
                let products_json = serde_json::to_value(products);
                match products_json {
                    Ok(json_value) => AdapterResponse {
                        status: StatusCode::Ok,
                        data: json_value,
                        binary: None,
                    },
                    Err(_) => AdapterResponse {
                        status: StatusCode::InternalServerError,
                        data: Value::String("Failed to serialize products".to_string()),
                        binary: None,
                    },
                }
            }
            Err(e) => AdapterResponse {
                status: StatusCode::InternalServerError,
                data: json!({"error": format!("Failed to fetch products: {}", e)}),
                binary: None,
            },
        }
    }
}
