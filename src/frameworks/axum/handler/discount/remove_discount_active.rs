use axum::{
    body::Bytes,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{Value, from_slice, json};
use std::sync::Arc;

use crate::{
    frameworks::adapter::axum::{AxumHandler, handle},
    interfaces::handler::generic_handler::AdapterRequest,
};

pub async fn remove_discount_handler(
    handler: Arc<AxumHandler>,
    Path(id): Path<String>,
    body: Bytes,
) -> impl IntoResponse {
    let body: Value = from_slice(&body).unwrap_or_else(|_| json!({}));
    let request = AdapterRequest {
        query: None,
        params: Some(json!({ "product_id": id })),
        body: Some(body),
    };

    let Json(adapter_response) = handle(handler, Json(request)).await;
    let status = StatusCode::from_u16(adapter_response.status as u16)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    match status == StatusCode::NO_CONTENT {
        true => status.into_response(),
        false => (status, Json(adapter_response.data)).into_response(),
    }
}
