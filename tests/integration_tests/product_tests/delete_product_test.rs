use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;
use tokio;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_08_delete_product_success() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");
    let client = Client::new();

    let create_url = format!("http://localhost:{}/products", port);
    let name = format!("Café Premium {}", uuid::Uuid::new_v4());

    let product_data = json!({
        "name": name,
        "description": "Descrição temporária",
        "stock": 10,
        "price": 1000
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

    let delete_url = format!("http://localhost:{}/products/{}", port, product_id);
    let response = client
        .delete(&delete_url)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let get_url = format!("http://localhost:{}/products/{}", port, product_id);
    let get_response = client
        .get(&get_url)
        .send()
        .await
        .expect("Failed to send request");

    println!("get_response: {:#?}", get_response);

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
#[serial]
async fn test_09_delete_product_not_found() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();

    let invalid_id = uuid::Uuid::new_v4();
    let url = format!("http://localhost:{}/products/{}", port, invalid_id);

    let response = client
        .delete(&url)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(response_body["error"], "Product not found");
}
