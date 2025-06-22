// src/routes.rs
use axum::{
    Router,
    routing::{delete, get, post},
};
use tower_http::trace::TraceLayer;

use crate::handlers::{coupon, product};

pub fn create_router(pool: crate::db::DbPool) -> Router {
    Router::new()
        .route(
            "/products",
            get(product::list_products).post(product::create_product),
        )
        .route(
            "/products/:id",
            get(product::get_product)
                .patch(product::update_product)
                .delete(product::delete_product),
        )
        .route(
            "/products/:id/discount/percent",
            post(product::apply_percent_discount),
        )
        .route(
            "/products/:id/discount/coupon",
            post(product::apply_coupon_discount),
        )
        .route("/products/:id/discount", delete(product::remove_discount))
        .route(
            "/coupons",
            get(coupon::list_coupons).post(coupon::create_coupon),
        )
        .route("/coupons/:code", get(coupon::get_coupon))
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}
