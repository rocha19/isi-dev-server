// src/handlers/coupon.rs
use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DbPool,
    error::AppError,
    models::coupon::{Coupon, CouponType, CreateCoupon},
};

pub async fn create_coupon(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateCoupon>,
) -> Result<Json<Coupon>, AppError> {
    payload.validate()?;

    let client = pool.get().await?;
    let coupon = client.query_one(
        "INSERT INTO coupons (id, code, type, value, one_shot, valid_from, valid_until, max_uses) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
         RETURNING *",
        &[
            &Uuid::new_v4(),
            &payload.code,
            &match payload.coupon_type {
                CouponType::Fixed => "fixed",
                CouponType::Percent => "percent",
            },
            &payload.value,
            &payload.one_shot,
            &payload.valid_from,
            &payload.valid_until,
            &payload.max_uses,
        ],
    ).await?;

    Ok(Json(Coupon {
        id: coupon.get(0),
        code: coupon.get(1),
        coupon_type: match coupon.get::<_, &str>(2) {
            "fixed" => CouponType::Fixed,
            _ => CouponType::Percent,
        },
        value: coupon.get(3),
        one_shot: coupon.get(4),
        valid_from: coupon.get(5),
        valid_until: coupon.get(6),
        uses_count: coupon.get(7),
        max_uses: coupon.get(8),
    }))
}

pub async fn list_coupons(State(pool): State<DbPool>) -> Result<Json<Vec<Coupon>>, AppError> {
    let client = pool.get().await?;
    let coupons = client.query("SELECT * FROM coupons", &[]).await?;

    let result = coupons
        .iter()
        .map(|row| Coupon {
            id: row.get(0),
            code: row.get(1),
            coupon_type: match row.get::<_, &str>(2) {
                "fixed" => CouponType::Fixed,
                _ => CouponType::Percent,
            },
            value: row.get(3),
            one_shot: row.get(4),
            valid_from: row.get(5),
            valid_until: row.get(6),
            uses_count: row.get(7),
            max_uses: row.get(8),
        })
        .collect();

    Ok(Json(result))
}

pub async fn get_coupon(
    State(pool): State<DbPool>,
    Path(code): Path<String>,
) -> Result<Json<Coupon>, AppError> {
    let client = pool.get().await?;
    // Diagnostics:
    // 1. `?` couldn't convert the error to `error::AppError`
    //    the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
    //    the following other types implement trait `std::convert::From<T>`:
    //      `error::AppError` implements `std::convert::From<deadpool::managed::PoolError<tokio_postgres::Error>>`
    //      `error::AppError` implements `std::convert::From<tokio_postgres::Error>`
    //      `error::AppError` implements `std::convert::From<validator::ValidationErrors>`
    //    required for `std::result::Result<axum::Json<models::coupon::Coupon>, error::AppError>` to implement `std::ops::FromResidual<std::result::Result<std::convert::Infallible, deadpool::managed::errors::PoolError<tokio_postgres::Error>>>` [E0277]
    // 2. this can't be annotated with `?` because it has type `Result<_, deadpool::managed::errors::PoolError<tokio_postgres::Error>>` [E0277]
    let coupon = client
        .query_one("SELECT * FROM coupons WHERE code = $1", &[&code])
        .await?;

    Ok(Json(Coupon {
        id: coupon.get(0),
        code: coupon.get(1),
        coupon_type: match coupon.get::<_, &str>(2) {
            "fixed" => CouponType::Fixed,
            _ => CouponType::Percent,
        },
        value: coupon.get(3),
        one_shot: coupon.get(4),
        valid_from: coupon.get(5),
        valid_until: coupon.get(6),
        uses_count: coupon.get(7),
        max_uses: coupon.get(8),
    }))
}
