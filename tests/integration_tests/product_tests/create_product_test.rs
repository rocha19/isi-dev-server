use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode as ReqwestStatusCode};
use serde_json::json;
use serial_test::serial;
use tokio;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_01_create_product_success() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();
    let url = format!("http://localhost:{}/api/v1/products", port);
    let name = format!("Café Premium {}", uuid::Uuid::new_v4());

    let product_data = json!({
        "name": name,
        "description": "100% arábica",
        "stock": 250,
        "price": 2590
    });
    let response = client
        .post(&url)
        .json(&product_data)
        .send()
        .await
        .expect("Failed to send request");
    println!("{:?}", response);

    assert_eq!(response.status(), ReqwestStatusCode::CREATED);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");

    assert_eq!(response_body["name"], name);
    assert_eq!(response_body["description"], "100% arábica");
    assert_eq!(response_body["stock"], 250);
    assert_eq!(response_body["price"], 2590);
}

#[tokio::test]
#[serial]
async fn test_02_create_product_missing_fields() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();
    let url = format!("http://localhost:{}/api/v1/products", port);

    let invalid_data = json!({});
    let response = client
        .post(&url)
        .json(&invalid_data)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), ReqwestStatusCode::BAD_REQUEST);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Missing required fields");
    let fields = response_body["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 3);
    assert!(fields.contains(&json!("name")));
    assert!(fields.contains(&json!("stock")));
    assert!(fields.contains(&json!("price")));

    let invalid_data = json!({
        "stock": 100,
        "price": 2000
    });
    let response = client
        .post(&url)
        .json(&invalid_data)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), ReqwestStatusCode::BAD_REQUEST);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Missing required fields");
    let fields = response_body["fields"].as_array().unwrap();
    assert_eq!(fields, &[json!("name")]);
}

#[tokio::test]
#[serial]
async fn test_03_create_product_invalid_types() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();
    let url = format!("http://localhost:{}/api/v1/products", port);

    let invalid_data = json!({
        "name": "Café Especial",
        "stock": "cem",
        "price": 3000
    });
    let response = client
        .post(&url)
        .json(&invalid_data)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), ReqwestStatusCode::BAD_REQUEST);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Invalid field types");
    assert!(
        response_body["details"]
            .as_str()
            .unwrap()
            .contains("invalid type: string \"cem\", expected u32")
    );

    let invalid_data = json!({
        "name": "Café Especial",
        "stock": 100,
        "price": "trinta"
    });
    let response = client
        .post(&url)
        .json(&invalid_data)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), ReqwestStatusCode::BAD_REQUEST);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Invalid field types");
    assert!(
        response_body["details"]
            .as_str()
            .unwrap()
            .contains("invalid type: string \"trinta\", expected u64")
    );
}

#[tokio::test]
#[serial]
async fn test_04_create_product_conflict() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();
    let url = format!("http://localhost:{}/api/v1/products", port);
    let name = format!("Café Premium {}", uuid::Uuid::new_v4());

    let product_data = json!({
        "name": name,
        "description": "Café especial de alta qualidade",
        "stock": 100,
        "price": 3000
    });

    let response1 = client
        .post(&url)
        .json(&product_data)
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(response1.status(), ReqwestStatusCode::CREATED);

    let response2 = client
        .post(&url)
        .json(&product_data)
        .send()
        .await
        .expect("Failed to send request");

    println!("{:?}", response2);

    assert_eq!(response2.status(), ReqwestStatusCode::CONFLICT);

    let response_body: serde_json::Value =
        response2.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Product already exists");
}
