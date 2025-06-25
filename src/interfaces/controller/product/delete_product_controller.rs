use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        repository::product_repository::ProductRepository,
        usecase::product::delete_product_usecase::DeleteProductUseCase,
    },
    interfaces::handler::generic_handler::{
        AdapterRequest, AdapterResponse, GenericHandler, StatusCode,
    },
};

#[derive(Clone)]
pub struct DeleteProductController {
    pub product_repository: Arc<RwLock<dyn ProductRepository + Send + Sync>>,
}

#[async_trait]
impl GenericHandler for DeleteProductController {
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
        let usecase = DeleteProductUseCase::new(repository);

        let response = usecase.execute(id).await;
        log::warn!("Response: {:#?}", response);

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
