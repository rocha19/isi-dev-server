use axum::{body::Bytes, extract::Json, response::IntoResponse};
use serde_json::{Value, from_slice, json};
use std::sync::Arc;

use crate::{
    frameworks::adapter::axum::{AxumHandler, handle},
    interfaces::handler::generic_handler::AdapterRequest,
};

use crate::interfaces::handler::generic_handler::StatusCode as AdapterStatusCode;
use axum::http::StatusCode as AxumStatusCode;

pub async fn create_coupon_handler(handler: Arc<AxumHandler>, body: Bytes) -> impl IntoResponse {
    let body: Value = match from_slice(&body) {
        Ok(v) => v,
        Err(_) => {
            return (
                AxumStatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid JSON format" })),
            )
                .into_response();
        }
    };

    let request = AdapterRequest {
        query: None,
        params: None,
        body: Some(body),
    };

    let response = handle(handler, Json(request)).await.0;

    let status = match response.status {
        AdapterStatusCode::Ok | AdapterStatusCode::Created => AxumStatusCode::CREATED,
        AdapterStatusCode::Conflict => AxumStatusCode::CONFLICT,
        _ => {
            if let Some(error_str) = response.data.get("error").and_then(|v| v.as_str()) {
                match error_str {
                    "Coupon already exists" => AxumStatusCode::CONFLICT,
                    _ => AxumStatusCode::BAD_REQUEST,
                }
            } else {
                AxumStatusCode::BAD_REQUEST
            }
        }
    };

    log::warn!("Response: {:#?}", status);

    (status, Json(response.data)).into_response()
}
