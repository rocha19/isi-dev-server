use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;
use uuid::Uuid;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_05_get_coupon_by_code_success() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();

    let code = format!("PROMO-{}", Uuid::new_v4());

    let create_url = format!("http://localhost:{}/api/v1/coupons", port);
    let coupon_data = json!({
        "code": code,
        "type": "percent",
        "value": 2000,
        "one_shot": false,
        "valid_from": "2025-01-01T00:00:00Z",
        "valid_until": "2025-12-31T23:59:59Z",
        "max_uses": 100
    });

    let create_response = client
        .post(&create_url)
        .json(&coupon_data)
        .send()
        .await
        .expect("Failed to create coupon");

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let get_url = format!("http://localhost:{}/api/v1/coupons/{}", port, code);
    let response = client
        .get(&get_url)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");

    println!("{:?}", response_body);
    // let coupon_data = &response_body["data"];
    println!("{:?}", response_body["code"]);

    assert_eq!(
        response_body["code"].as_str().unwrap().to_uppercase(),
        code.to_uppercase()
    );
    assert_eq!(response_body["type"], "percent");
    assert_eq!(response_body["value"], 2000);
    assert_eq!(response_body["max_uses"], 100);
}

#[tokio::test]
#[serial]
async fn test_06_get_coupon_by_code_not_found() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();

    let code = format!("INVALID-{}", Uuid::new_v4());

    let get_url = format!("http://localhost:{}/api/v1/coupons/{}", port, code);
    let response = client
        .get(&get_url)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Coupon not found");
}
