use axum::extract::{Json, Path};
use serde_json::json;
use std::sync::Arc;

use crate::{
    frameworks::adapter::axum::{AxumHandler, handle},
    interfaces::handler::generic_handler::{AdapterRequest, AdapterResponse},
};

pub async fn restore_product_by_id_handler(
    handler: Arc<AxumHandler>,
    Path(id): Path<String>,
) -> Json<AdapterResponse> {
    log::info!("Start request");
    log::info!("Product ID: {}", id);
    let request = AdapterRequest {
        query: None,
        params: Some(json!({"id": id})),
        body: None,
    };

    log::info!("End request");
    handle(handler, Json(request)).await
}
