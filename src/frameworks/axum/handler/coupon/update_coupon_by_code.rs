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

pub async fn update_coupon_by_id_handler(
    handler: Arc<AxumHandler>,
    Path(code): Path<String>,
    body: Bytes,
) -> impl IntoResponse {
    let body: Value = from_slice(&body).unwrap_or_else(|_| json!({}));

    let request = AdapterRequest {
        query: None,
        params: Some(json!({"code": code})),
        body: Some(body),
    };

    let Json(adapter_response) = handle(handler, Json(request)).await;

    let status = StatusCode::from_u16(adapter_response.status as u16)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    (status, Json(adapter_response.data)).into_response()
}
