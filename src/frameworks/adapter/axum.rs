use axum::Json;
use std::sync::Arc;

use crate::interfaces::handler::generic_handler::{
    AdapterRequest, AdapterResponse, GenericHandler,
};

#[allow(dead_code)]
pub struct AxumHandler {
    pub inner: Arc<dyn GenericHandler + Send + Sync>,
}

#[allow(dead_code)]
pub async fn handle(
    axum_handler: Arc<AxumHandler>,
    Json(request): Json<AdapterRequest>,
) -> Json<AdapterResponse> {
    let response = axum_handler.inner.handle(request).await;
    Json(response)
}
