use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;
use tokio;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_06_update_product_success() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");
    let client = Client::new();

    let create_url = format!("http://localhost:{}/api/v1/products", port);

    let product_data = json!({
        "name": format!("Café Premium {}", uuid::Uuid::new_v4()),
        "description": "Para o dia a dia",
        "stock": 500,
        "price": 1500
    });

    let create_response = client
        .post(&create_url)
        .json(&product_data)
        .send()
        .await
        .expect("Failed to create product");

    let created_product: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse created product");

    let product_id = created_product["id"].as_str().expect("Product ID missing");
    let update_url = format!("http://localhost:{}/api/v1/products/{}", port, product_id);
    let update_data = json!({
        "name": format!("Café Gourmet {}", uuid::Uuid::new_v4()),
        "stock": 50
    });

    let response = client
        .patch(&update_url)
        .json(&update_data)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_07_update_product_not_found() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");
    let client = Client::new();

    let invalid_id = "123e4567-e89b-12d3-a456-426614174000";
    let url = format!("http://localhost:{}/api/v1/products/{}", port, invalid_id);
    let update_data = json!({
        "name": "Produto Inexistente"
    });

    let response = client
        .patch(&url)
        .json(&update_data)
        .send()
        .await
        .expect("Failed to send request");

    println!("Response: {:#?}", response);

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Product not found");
}
