use axum::{
    body::Bytes,
    extract::{Path, Query},
    http::Method,
    routing::{Router, delete, get, patch, post},
};
use serde_json::Value;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use super::handler::{
    coupon::{
        create_coupon::create_coupon_handler, delete_coupon_by_code::delete_coupon_by_id_handler,
        get_coupon_by_code::get_coupon_by_id_handler, get_coupons::get_coupons_handler,
        update_coupon_by_code::update_coupon_by_id_handler,
    },
    discount::{
        apply_coupon_discount::apply_coupon_discount_handler,
        apply_percent_discount::apply_percent_discount_handler,
        remove_discount_active::remove_discount_handler,
    },
    product::{
        create_product::create_product_handler, delete_product_by_id::delete_product_by_id_handler,
        get_product_by_id::get_product_by_id_handler, get_products::get_all_products_handler,
        restore_product_by_id::restore_product_by_id_handler,
        update_product_by_id::update_product_by_id_handler,
    },
};
use crate::{
    application::repository::{
        coupon_postgres_repository::PostgresCouponRepository,
        discount_postgres_repository::PostgresDiscountRepository,
        product_postgres_repository::PostgresProductRepository,
    },
    domain::repository::{
        coupon_repository::CouponRepository, discount_repository::DiscountRepository,
        product_repository::ProductRepository,
    },
    frameworks::{
        adapter::axum::AxumHandler,
        sqlx::{pool::add_pool, run_schema::run_schema},
    },
    interfaces::controller::{
        coupon::{
            create_coupon_controller::CreateCouponController,
            delete_coupon_controller::DeleteCouponController,
            get_coupon_controller::GetCouponController,
            get_coupons_controller::GetAllCouponsController,
            update_coupon_controller::UpdateCouponController,
        },
        discount::{
            apply_coupon_discount_controller::ApplyCouponDiscountController,
            apply_percent_discount_controller::ApplyPercentDiscountController,
            remove_discount_controller::RemoveDiscountController,
        },
        product::{
            create_product_controller::CreateProductController,
            delete_product_controller::DeleteProductController,
            get_product_controller::GetProductController,
            get_products_controller::GetAllProductsController,
            restore_product_controller::RestoreProductController,
            update_product_controller::UpdateProductController,
        },
    },
};

pub async fn run(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let database: String = dotenv::var("DATABASE_URL")
        .unwrap()
        .parse()
        .expect("DATABASE_URL must be a string");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
        ])
        .max_age(Duration::from_secs(3601));

    tracing_subscriber::fmt::init();

    /*
     * Repositories Postgres
     */
    let arc_pool = add_pool(database).await;

    run_schema(&arc_pool).await.map_err(|e| {
        tracing::error!("Schema execution error: {}", e);
        e
    })?;

    let postgres_product_repository = PostgresProductRepository::new(arc_pool.clone());
    let postgres_coupon_repository = PostgresCouponRepository::new(arc_pool.clone());
    let postgres_discount_repository = PostgresDiscountRepository::new(arc_pool.clone());

    /*
     * Repositories In Memory
     */
    // let in_memory_product_repository = InMemoryProductRepository::new()
    // let in_memory_coupon_repository =    InMemoryCouponRepository::new();

    /*
     * Repositories
     */
    let product_repository: Arc<RwLock<dyn ProductRepository + Send + Sync>> =
        Arc::new(RwLock::new(postgres_product_repository));
    let coupon_repository: Arc<RwLock<dyn CouponRepository + Send + Sync>> =
        Arc::new(RwLock::new(postgres_coupon_repository));
    let discount_repository: Arc<RwLock<dyn DiscountRepository + Send + Sync>> =
        Arc::new(RwLock::new(postgres_discount_repository));

    /*
     * Product Controllers
     */
    let (
        create_product_controller,
        get_product_controller,
        get_all_product_controller,
        update_product_controller,
        delete_product_controller,
        restore_product_controller,
    ) = (
        Arc::new(CreateProductController {
            product_repository: product_repository.clone(),
        }),
        Arc::new(GetProductController {
            product_repository: product_repository.clone(),
            discount_repository: discount_repository.clone(),
        }),
        Arc::new(GetAllProductsController {
            product_repository: product_repository.clone(),
        }),
        Arc::new(UpdateProductController {
            product_repository: product_repository.clone(),
        }),
        Arc::new(DeleteProductController {
            product_repository: product_repository.clone(),
        }),
        Arc::new(RestoreProductController {
            product_repository: product_repository.clone(),
        }),
    );

    /*
     * Coupons Controllers
     */
    let (
        create_coupon_controller,
        get_coupon_controller,
        get_coupons_controller,
        update_coupon_controller,
        delete_coupon_controller,
    ) = (
        Arc::new(CreateCouponController {
            coupon_repository: coupon_repository.clone(),
        }),
        Arc::new(GetCouponController {
            coupon_repository: coupon_repository.clone(),
        }),
        Arc::new(GetAllCouponsController {
            coupon_repository: coupon_repository.clone(),
        }),
        Arc::new(UpdateCouponController {
            coupon_repository: coupon_repository.clone(),
        }),
        Arc::new(DeleteCouponController {
            coupon_repository: coupon_repository.clone(),
        }),
    );

    /*
     * Discount Controllers
     */
    let (
        apply_percent_discount_controller,
        apply_coupon_discount_controller,
        remove_discount_controller,
    ) = (
        Arc::new(ApplyPercentDiscountController {
            discount_repository: discount_repository.clone(),
        }),
        Arc::new(ApplyCouponDiscountController {
            discount_repository: discount_repository.clone(),
        }),
        Arc::new(RemoveDiscountController {
            discount_repository: discount_repository.clone(),
        }),
    );

    /*
     * Product Handlers Adapters
     */
    let (
        make_create_product_handler,
        make_get_product_by_id_handler,
        make_get_all_products_handler,
        make_update_product_by_id_handler,
        make_delete_product_by_id_handler,
        make_restore_product_by_id_handler,
    ) = (
        Arc::new(AxumHandler {
            inner: create_product_controller,
        }),
        Arc::new(AxumHandler {
            inner: get_product_controller,
        }),
        Arc::new(AxumHandler {
            inner: get_all_product_controller,
        }),
        Arc::new(AxumHandler {
            inner: update_product_controller,
        }),
        Arc::new(AxumHandler {
            inner: delete_product_controller,
        }),
        Arc::new(AxumHandler {
            inner: restore_product_controller,
        }),
    );

    /*
     * Coupons Generic Handlers
     */
    let (
        make_create_coupon_handler,
        make_get_coupon_by_id_handler,
        make_get_coupons_handler,
        make_update_coupon_by_id_handler,
        make_delete_coupon_by_id_handler,
    ) = (
        Arc::new(AxumHandler {
            inner: create_coupon_controller,
        }),
        Arc::new(AxumHandler {
            inner: get_coupon_controller,
        }),
        Arc::new(AxumHandler {
            inner: get_coupons_controller,
        }),
        Arc::new(AxumHandler {
            inner: update_coupon_controller,
        }),
        Arc::new(AxumHandler {
            inner: delete_coupon_controller,
        }),
    );

    /*
     * Discount Generic Handlers
     */
    let (
        make_apply_percent_discount_handler,
        make_apply_coupon_discount_handler,
        make_remove_discount_handler,
    ) = (
        Arc::new(AxumHandler {
            inner: apply_percent_discount_controller,
        }),
        Arc::new(AxumHandler {
            inner: apply_coupon_discount_controller,
        }),
        Arc::new(AxumHandler {
            inner: remove_discount_controller,
        }),
    );

    /*
     * Product Routes (Axum Adapters)
     */
    let create_product_route =
        move |body: Bytes| create_product_handler(make_create_product_handler.clone(), body);
    let get_product_route = move |param: Path<String>| {
        get_product_by_id_handler(make_get_product_by_id_handler.clone(), param)
    };
    let get_all_products_route = move |query: Query<Value>| {
        get_all_products_handler(make_get_all_products_handler.clone(), query)
    };
    let update_product_route = move |param: Path<String>, body: Bytes| {
        update_product_by_id_handler(make_update_product_by_id_handler.clone(), param, body)
    };
    let delete_product_route = move |param: Path<String>| {
        delete_product_by_id_handler(make_delete_product_by_id_handler.clone(), param)
    };
    let restore_product_route = move |param: Path<String>| {
        restore_product_by_id_handler(make_restore_product_by_id_handler.clone(), param)
    };

    /*
     * Discount Routes (Axum Adapters)
     */
    let apply_percent_discount_route = move |param: Path<String>, body: Bytes| {
        apply_percent_discount_handler(make_apply_percent_discount_handler.clone(), param, body)
    };
    let apply_coupon_discount_route = move |param: Path<String>, body: Bytes| {
        apply_coupon_discount_handler(make_apply_coupon_discount_handler.clone(), param, body)
    };
    let remove_discount_route = move |param: Path<String>, body: Bytes| {
        remove_discount_handler(make_remove_discount_handler.clone(), param, body)
    };

    /*
     * Coupons Routes (Axum Adapters)
     */
    let create_coupon_route =
        move |body: Bytes| create_coupon_handler(make_create_coupon_handler.clone(), body);
    let get_coupon_route = move |param: Path<String>| {
        get_coupon_by_id_handler(make_get_coupon_by_id_handler.clone(), param)
    };
    let list_coupons_route =
        move |query: Query<Value>| get_coupons_handler(make_get_coupons_handler.clone(), query);
    let update_coupon_route = move |param: Path<String>, body: Bytes| {
        update_coupon_by_id_handler(make_update_coupon_by_id_handler.clone(), param, body)
    };
    let delete_coupon_route = move |param: Path<String>| {
        delete_coupon_by_id_handler(make_delete_coupon_by_id_handler.clone(), param)
    };

    let api_routes = Router::new()
        .route("/products", post(create_product_route))
        .route("/products", get(get_all_products_route))
        .route("/products/:id", get(get_product_route))
        .route("/products/:id", patch(update_product_route))
        .route("/products/:id", delete(delete_product_route))
        .route("/products/:id/restore", post(restore_product_route))
        .route(
            "/products/:id/discount/percent",
            post(apply_percent_discount_route),
        )
        .route(
            "/products/:id/discount/coupon",
            post(apply_coupon_discount_route),
        )
        .route("/products/:id/discount", delete(remove_discount_route))
        .route("/coupons", post(create_coupon_route))
        .route("/coupons", get(list_coupons_route))
        .route("/coupons/:code", get(get_coupon_route))
        .route("/coupons/:code", patch(update_coupon_route))
        .route("/coupons/:code", delete(delete_coupon_route))
        .route("/health", get(health_check));

    let app = Router::new().merge(api_routes).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
