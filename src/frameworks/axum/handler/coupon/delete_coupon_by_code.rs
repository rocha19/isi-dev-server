use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use std::sync::Arc;

use crate::{
    frameworks::adapter::axum::{AxumHandler, handle},
    interfaces::handler::generic_handler::AdapterRequest,
};

pub async fn delete_coupon_by_id_handler(
    handler: Arc<AxumHandler>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let request = AdapterRequest {
        query: None,
        params: Some(json!({"code": code})),
        body: None,
    };

    let Json(adapter_response) = handle(handler, Json(request)).await;

    let status = StatusCode::from_u16(adapter_response.status as u16)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    match status == StatusCode::NO_CONTENT {
        true => status.into_response(),
        false => (status, Json(adapter_response.data)).into_response(),
    }
}
