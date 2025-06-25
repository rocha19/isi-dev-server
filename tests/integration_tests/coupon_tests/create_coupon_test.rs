use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode as ReqwestStatusCode};
use serde_json::json;
use serial_test::serial;
use tokio;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_01_create_coupon_success() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT").unwrap().parse().unwrap();
    let client = Client::new();
    let url = format!("http://localhost:{}/coupons", port);

    let code = format!("PROMO-{}", uuid::Uuid::new_v4());

    let coupon_data = json!({
        "code": code,
        "type": "percent",
        "value": 2000,
        "one_shot": false,
        "valid_from": "2025-01-01T00:00:00Z",
        "valid_until": "2025-12-31T23:59:59Z",
        "max_uses": 100
    });

    let response = client.post(&url).json(&coupon_data).send().await.unwrap();
    assert_eq!(response.status(), ReqwestStatusCode::CREATED);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(
        body["code"].as_str().unwrap().to_lowercase(),
        code.to_lowercase()
    );
    assert_eq!(body["type"], "percent");
    assert_eq!(body["value"], 2000);
    assert_eq!(body["one_shot"], false);
    assert_eq!(body["max_uses"], 100);
}

#[tokio::test]
#[serial]
async fn test_02_create_coupon_missing_fields() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT").unwrap().parse().unwrap();
    let client = Client::new();
    let url = format!("http://localhost:{}/coupons", port);

    let invalid_data = json!({});
    let response = client.post(&url).json(&invalid_data).send().await.unwrap();
    assert_eq!(response.status(), ReqwestStatusCode::BAD_REQUEST);

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{}", body);
    assert_eq!(body["error"], "Missing required fields");

    let fields = body["fields"].as_array().unwrap();
    assert!(fields.contains(&json!("code")));
    assert!(fields.contains(&json!("type")));
    assert!(fields.contains(&json!("value")));
}

#[tokio::test]
#[serial]
async fn test_03_create_coupon_invalid_types() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT").unwrap().parse().unwrap();
    let client = Client::new();
    let url = format!("http://localhost:{}/coupons", port);

    let invalid_data = json!({
        "code": "PROMO10",
        "type": "percent",
        "value": "vinte",
        "one_shot": "yes",
        "valid_from": "invalid-date",
        "valid_until": "also-invalid",
        "max_uses": "cem"
    });

    let response = client.post(&url).json(&invalid_data).send().await.unwrap();
    assert_eq!(response.status(), ReqwestStatusCode::BAD_REQUEST);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["error"], "Invalid field types");
    assert!(body["details"].as_str().unwrap().contains("invalid type"));
}

#[tokio::test]
#[serial]
async fn test_04_create_coupon_conflict() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT").unwrap().parse().unwrap();
    let client = Client::new();
    let url = format!("http://localhost:{}/coupons", port);

    let code = format!("PROMO20-{}", uuid::Uuid::new_v4());

    let coupon_data = json!({
        "code": code,
        "type": "percent",
        "value": 1500,
        "one_shot": false,
        "valid_from": "2025-01-01T00:00:00Z",
        "valid_until": "2025-12-31T23:59:59Z",
        "max_uses": 50
    });

    let response1 = client.post(&url).json(&coupon_data).send().await.unwrap();
    assert_eq!(response1.status(), ReqwestStatusCode::CREATED);

    let response2 = client.post(&url).json(&coupon_data).send().await.unwrap();
    assert_eq!(response2.status(), ReqwestStatusCode::CONFLICT);
}
