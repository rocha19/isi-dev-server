mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod utils;

use axum::http::Method;
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");
    db::init_db(&pool)
        .await
        .expect("Failed to initialize database");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = routes::create_router(pool).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
