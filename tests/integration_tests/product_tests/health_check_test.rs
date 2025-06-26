use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use serial_test::serial;
use tokio;

use crate::utils::start_server::init_tracing;

#[tokio::test]
#[serial]
async fn test_00_health_check_success() {
    init_tracing();

    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client = Client::new();
    let url = format!("http://localhost:{}/api/v1/health", port);
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    let response_body = response.text().await.expect("Failed to read response text");
    assert_eq!(response_body, "OK");
}
