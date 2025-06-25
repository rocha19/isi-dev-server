use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_05_get_product_by_id_success() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();
    let name = format!("Café Premium {}", uuid::Uuid::new_v4());

    let create_url = format!("http://localhost:{}/products", port);
    let product_data = json!({
        "name": name.clone(),
        "description": "Notas de chocolate",
        "stock": 100,
        "price": 3200
    });

    let create_response = client
        .post(&create_url)
        .json(&product_data)
        .send()
        .await
        .expect("Failed to create product");

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let created_product: serde_json::Value = create_response
        .json()
        .await
        .expect("Failed to parse created product");

    let product_id = created_product["id"].as_str().expect("Product ID missing");

    let get_url = format!("http://localhost:{}/products/{}", port, product_id);
    let response = client
        .get(&get_url)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");

    assert_eq!(response_body["name"], name);
    assert_eq!(response_body["description"], "Notas de chocolate");
    assert_eq!(response_body["stock"], 100);
    assert_eq!(response_body["price"], 3200);
}
