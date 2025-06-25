use axum::{Json, extract::Query};
use serde_json::{Value, json};
use std::sync::Arc;

use crate::{
    frameworks::adapter::axum::{AxumHandler, handle},
    interfaces::handler::generic_handler::{AdapterRequest, AdapterResponse},
};

pub async fn get_all_products_handler(
    handler: Arc<AxumHandler>,
    query: Query<Value>,
) -> Json<AdapterResponse> {
    let search = query.get("search").cloned().unwrap_or_else(|| json!(null));
    let page = match query.get("page") {
        Some(value) => match value.as_str().unwrap_or("").parse::<u32>() {
            Ok(parsed_value) => json!(parsed_value),
            Err(_) => json!(null),
        },
        None => json!(null),
    };

    let limit = match query.get("limit") {
        Some(value) => match value.as_str().unwrap_or("").parse::<u32>() {
            Ok(parsed_value) => json!(parsed_value),
            Err(_) => json!(null),
        },
        None => json!(null),
    };

    let min_price = match query.get("min_price") {
        Some(value) => match value.as_str().unwrap_or("").parse::<u32>() {
            Ok(parsed_value) => json!(parsed_value),
            Err(_) => json!(null),
        },
        None => json!(null),
    };

    let max_price = match query.get("max_price") {
        Some(value) => match value.as_str().unwrap_or("").parse::<u32>() {
            Ok(parsed_value) => json!(parsed_value),
            Err(_) => json!(null),
        },
        None => json!(null),
    };

    let has_discount = match query.get("has_discount") {
        Some(value) => match value.as_str().unwrap_or("").parse::<bool>() {
            Ok(parsed_value) => json!(parsed_value),
            Err(_) => json!(null),
        },
        None => json!(null),
    };

    let mut products = json!({});
    products["search"] = search;
    products["page"] = page;
    products["limit"] = limit;
    products["min_price"] = min_price;
    products["max_price"] = max_price;
    products["has_discount"] = has_discount;

    let request = AdapterRequest {
        query: Some(products),
        params: None,
        body: None,
    };

    return handle(handler, axum::Json(request)).await;
}
