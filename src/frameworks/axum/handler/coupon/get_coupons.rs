use axum::{Json, extract::Query};
use serde_json::Value;
use std::sync::Arc;

use crate::{
    frameworks::adapter::axum::{AxumHandler, handle},
    interfaces::handler::generic_handler::{AdapterRequest, AdapterResponse},
};

pub async fn get_coupons_handler(
    handler: Arc<AxumHandler>,
    query: Query<Value>,
) -> Json<AdapterResponse> {
    let request = AdapterRequest {
        query: None,
        params: None,
        body: None,
    };

    return handle(handler, axum::Json(request)).await;
}
