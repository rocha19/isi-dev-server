use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;
use tokio;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_05_update_coupon_success() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT").unwrap().parse().unwrap();
    let client = Client::new();
    let create_url = format!("http://localhost:{}/coupons", port);

    let code = format!("UPDATE-{}", uuid::Uuid::new_v4());
    let create_data = json!({
        "code": code,
        "type": "percent",
        "value": 2000,
        "one_shot": true,
        "valid_from": "2025-01-01T00:00:00Z",
        "valid_until": "2025-12-31T23:59:59Z",
        "max_uses": null
    });

    let create_response = client
        .post(&create_url)
        .json(&create_data)
        .send()
        .await
        .unwrap();
    assert_eq!(create_response.status(), StatusCode::CREATED);

    let update_url = format!("http://localhost:{}/coupons/{}", port, code);
    let update_data = json!({
        "value": 2000,
        "one_shot": false,
        "max_uses": 100,
        "valid_until": "2026-12-31T23:59:59Z"
    });

    let response = client
        .patch(&update_url)
        .json(&update_data)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_06_update_coupon_not_found() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT").unwrap().parse().unwrap();
    let client = Client::new();

    let invalid_code = "INVALID_CODE_123";
    let url = format!("http://localhost:{}/coupons/{}", port, invalid_code);
    let update_data = json!({"value": 5000});

    let response = client.patch(&url).json(&update_data).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["error"], "Coupon not found");
}

#[tokio::test]
#[serial]
async fn test_07_update_coupon_invalid_data() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT").unwrap().parse().unwrap();
    let client = Client::new();
    let create_url = format!("http://localhost:{}/coupons", port);

    let code = format!("INVALID-{}", uuid::Uuid::new_v4());
    let create_data = json!({
        "code": &code,
        "type": "percent",
        "value": 1000
    });

    client
        .post(&create_url)
        .json(&create_data)
        .send()
        .await
        .unwrap();

    let update_url = format!("http://localhost:{}/coupons/{}", port, code);
    let invalid_data = json!({
        "value": "dez",
        "max_uses": -5,
        "valid_until": "formato-invalido"
    });

    let response = client
        .patch(&update_url)
        .json(&invalid_data)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body: serde_json::Value = response.json().await.unwrap();
    let error_msg = body["error"].as_str().unwrap_or_default();
    assert!(error_msg.starts_with("Invalid body:"));
}
