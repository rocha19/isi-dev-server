use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;
use tokio;
use uuid::Uuid;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_10_delete_coupon_success() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");
    let client = Client::new();

    let code = format!("del-{}", Uuid::new_v4());
    let create_url = format!("http://localhost:{}/coupons", port);
    let coupon_data = json!({
        "code": code,
        "type": "fixed",
        "value": 1000,
        "one_shot": true,
        "valid_from": "2025-01-01T00:00:00Z",
        "valid_until": "2025-12-31T23:59:59Z",
        "max_uses": null
    });

    client
        .post(&create_url)
        .json(&coupon_data)
        .send()
        .await
        .expect("Failed to create coupon");

    let delete_url = format!("http://localhost:{}/coupons/{}", port, code);
    let delete_response = client
        .delete(&delete_url)
        .send()
        .await
        .expect("Failed to send DELETE request");

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let get_url = format!("http://localhost:{}/coupons/{}", port, code);
    let get_response = client
        .get(&get_url)
        .send()
        .await
        .expect("Failed to send GET request");

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
#[serial]
async fn test_11_delete_coupon_not_found() {
    init_tracing();
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");
    let client = Client::new();

    let invalid_code = format!("INVALID-{}", Uuid::new_v4());
    let delete_url = format!("http://localhost:{}/coupons/{}", port, invalid_code);

    let response = client
        .delete(&delete_url)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Coupon not found");
}
