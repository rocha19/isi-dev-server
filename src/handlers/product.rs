// src/handlers/product.rs
use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DbPool,
    error::AppError,
    models::{
        discount::{PaginatedResponse, PaginationMeta},
        product::{ApplyCoupon, ApplyPercentDiscount, CreateProduct, Product, UpdateProduct},
    },
};

#[derive(Debug, Deserialize)]
pub struct ProductQueryParams {
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>,
    min_price: Option<i64>,
    max_price: Option<i64>,
    has_discount: Option<bool>,
}

pub async fn create_product(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateProduct>,
) -> Result<Json<Product>, AppError> {
    payload.validate()?;

    let client = pool.get().await?;
    let product = client.query_one(
        "INSERT INTO products (id, name, description, stock, price, original_price, created_at) 
         VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP) 
         RETURNING *",
        &[
            &Uuid::new_v4(),
            &payload.name,
            &payload.description,
            &payload.stock,
            &payload.price,
            &payload.price,
        ],
    ).await?;

    Ok(Json(Product {
        id: product.get(0),
        name: product.get(1),
        description: product.get(2),
        stock: product.get(3),
        price: product.get(4),
        original_price: product.get(5),
        created_at: product.get(6),
        updated_at: product.get(7),
        deleted_at: product.get(8),
    }))
}

pub async fn list_products(
    State(pool): State<DbPool>,
    Query(params): Query<ProductQueryParams>,
) -> Result<Json<PaginatedResponse<Product>>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let client = pool.get().await?;
    let mut query = "
        SELECT *, COUNT(*) OVER() AS total_items 
        FROM products 
        WHERE deleted_at IS NULL 
    "
    .to_string();

    let search_term = params.search.map(|s| format!("%{}%", s));
    let min_price = params.min_price;
    let max_price = params.max_price;
    let limit_val = limit as i64;
    let offset_val = offset as i64;

    let mut conditions = Vec::new();
    let mut query_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    let mut param_count = 1;

    if let Some(ref search) = search_term {
        conditions.push(format!("name ILIKE ${}", param_count));
        query_params.push(search);
        param_count += 1;
    }

    if let Some(ref min_price) = min_price {
        conditions.push(format!("price >= ${}", param_count));
        query_params.push(min_price);
        param_count += 1;
    }

    if let Some(ref max_price) = max_price {
        conditions.push(format!("price <= ${}", param_count));
        query_params.push(max_price);
        param_count += 1;
    }

    if let Some(true) = params.has_discount {
        conditions.push(
            "EXISTS (SELECT 1 FROM product_discounts WHERE product_id = products.id)".to_string(),
        );
    }

    if !conditions.is_empty() {
        query.push_str(" AND ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY created_at DESC");
    query.push_str(&format!(
        " LIMIT ${} OFFSET ${}",
        param_count,
        param_count + 1
    ));
    query_params.push(&limit_val);
    query_params.push(&offset_val);

    let rows = client.query(&query, &query_params).await?;

    let total_items: i64 = if rows.is_empty() {
        0
    } else {
        rows[0].get("total_items")
    };

    let products = rows
        .iter()
        .map(|row| Product {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            stock: row.get("stock"),
            price: row.get("price"),
            original_price: row.get("original_price"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        })
        .collect();

    Ok(Json(PaginatedResponse {
        data: products,
        meta: PaginationMeta {
            page,
            limit,
            total_items: total_items as u64,
            total_pages: (total_items as f64 / limit as f64).ceil() as u32,
        },
    }))
}

pub async fn get_product(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, AppError> {
    let client = pool.get().await?;

    let row = client
        .query_one(
            "SELECT * FROM products WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await?;

    Ok(Json(Product {
        id: row.get(0),
        name: row.get(1),
        description: row.get(2),
        stock: row.get(3),
        price: row.get(4),
        original_price: row.get(5),
        created_at: row.get(6),
        updated_at: row.get(7),
        deleted_at: row.get(8),
    }))
}

pub async fn update_product(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, AppError> {
    payload.validate()?;

    let client = pool.get().await?;
    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>> = Vec::new();
    params.push(Box::new(id));

    let mut param_index = 2;

    let name_val = payload.name;
    let description_val = payload.description;
    let stock_val = payload.stock;
    let price_val = payload.price;
    let now = Utc::now();

    if let Some(name) = name_val {
        updates.push(format!("name = ${}", param_index));
        params.push(Box::new(name));
        param_index += 1;
    }

    if let Some(description) = description_val {
        updates.push(format!("description = ${}", param_index));
        params.push(Box::new(description));
        param_index += 1;
    }

    if let Some(stock) = stock_val {
        updates.push(format!("stock = ${}", param_index));
        params.push(Box::new(stock));
        param_index += 1;
    }

    if let Some(price) = price_val {
        updates.push(format!("price = ${}", param_index));
        params.push(Box::new(price));
        param_index += 1;
    }

    if updates.is_empty() {
        return Err(AppError::UnprocessableEntity("No fields to update".into()));
    }

    updates.push(format!("updated_at = ${}", param_index));
    params.push(Box::new(now));

    let query = format!(
        "UPDATE products SET {} WHERE id = $1 RETURNING *",
        updates.join(", ")
    );

    let ref_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params
        .iter()
        .map(|x| x.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync))
        .collect();
    let row = client.query_one(&query, &ref_params[..]).await?;

    Ok(Json(Product {
        id: row.get(0),
        name: row.get(1),
        description: row.get(2),
        stock: row.get(3),
        price: row.get(4),
        original_price: row.get(5),
        created_at: row.get(6),
        updated_at: row.get(7),
        deleted_at: row.get(8),
    }))
}

pub async fn delete_product(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let client = pool.get().await?;

    client
        .execute(
            "UPDATE products SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1",
            &[&id],
        )
        .await?;

    Ok(Json(()))
}

pub async fn apply_percent_discount(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ApplyPercentDiscount>,
) -> Result<Json<Product>, AppError> {
    let mut client = pool.get().await?;
    let tx = client.transaction().await?;

    let discount_exists = tx
        .query_opt(
            "SELECT 1 FROM product_discounts WHERE product_id = $1",
            &[&id],
        )
        .await?
        .is_some();

    if discount_exists {
        return Err(AppError::Conflict("Discount already applied".into()));
    }

    let product = tx
        .query_one(
            "UPDATE products 
             SET price = original_price * (100 - $1) / 100 
             WHERE id = $2 AND deleted_at IS NULL
             RETURNING *",
            &[&(payload.percentage as i64), &id],
        )
        .await?;

    let final_price: i64 = product.get(4);
    if final_price < 1 {
        return Err(AppError::UnprocessableEntity(
            "Final price must be at least 1 cent".into(),
        ));
    }

    tx.execute(
        "INSERT INTO product_discounts (product_id, discount_type, discount_value) 
         VALUES ($1, 'percentage', $2)",
        &[&id, &(payload.percentage as i64)],
    )
    .await?;

    tx.commit().await?;

    Ok(Json(Product {
        id: product.get(0),
        name: product.get(1),
        description: product.get(2),
        stock: product.get(3),
        price: final_price,
        original_price: product.get(5),
        created_at: product.get(6),
        updated_at: product.get(7),
        deleted_at: product.get(8),
    }))
}

pub async fn apply_coupon_discount(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ApplyCoupon>,
) -> Result<Json<Product>, AppError> {
    let mut client = pool.get().await?;
    let tx = client.transaction().await?;

    let discount_exists = tx
        .query_opt(
            "SELECT 1 FROM product_discounts WHERE product_id = $1",
            &[&id],
        )
        .await?
        .is_some();

    if discount_exists {
        return Err(AppError::Conflict("Discount already applied".into()));
    }

    let coupon = tx
        .query_opt(
            "SELECT * FROM coupons 
             WHERE code = $1 
               AND valid_from <= CURRENT_TIMESTAMP 
               AND valid_until >= CURRENT_TIMESTAMP 
               AND (max_uses IS NULL OR uses_count < max_uses)",
            &[&payload.code],
        )
        .await?
        .ok_or(AppError::NotFound)?;

    let coupon_id: Uuid = coupon.get(0);
    let coupon_type: String = coupon.get(2);
    let coupon_value: i64 = coupon.get(3);

    let product = tx
        .query_one(
            "SELECT * FROM products WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await?;

    let original_price: i64 = product.get(5);
    let mut new_price = original_price;

    match coupon_type.as_str() {
        "fixed" => new_price -= coupon_value,
        "percent" => new_price = original_price * (10000 - coupon_value) / 10000,
        _ => return Err(AppError::UnprocessableEntity("Invalid coupon type".into())),
    }

    if new_price < 1 {
        return Err(AppError::UnprocessableEntity(
            "Final price must be at least 1 cent".into(),
        ));
    }

    tx.execute(
        "UPDATE products SET price = $1 WHERE id = $2",
        &[&new_price, &id],
    )
    .await?;

    tx.execute(
        "INSERT INTO product_discounts (product_id, coupon_id) VALUES ($1, $2)",
        &[&id, &coupon_id],
    )
    .await?;

    tx.execute(
        "UPDATE coupons SET uses_count = uses_count + 1 WHERE id = $1",
        &[&coupon_id],
    )
    .await?;

    tx.commit().await?;

    Ok(Json(Product {
        id: product.get(0),
        name: product.get(1),
        description: product.get(2),
        stock: product.get(3),
        price: new_price,
        original_price,
        created_at: product.get(6),
        updated_at: product.get(7),
        deleted_at: product.get(8),
    }))
}

pub async fn remove_discount(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, AppError> {
    let mut client = pool.get().await?;
    let tx = client.transaction().await?;

    let product = tx
        .query_one(
            "UPDATE products 
             SET price = original_price 
             WHERE id = $1 
             RETURNING *",
            &[&id],
        )
        .await?;

    tx.execute(
        "DELETE FROM product_discounts WHERE product_id = $1",
        &[&id],
    )
    .await?;

    tx.commit().await?;

    Ok(Json(Product {
        id: product.get(0),
        name: product.get(1),
        description: product.get(2),
        stock: product.get(3),
        price: product.get(4),
        original_price: product.get(5),
        created_at: product.get(6),
        updated_at: product.get(7),
        deleted_at: product.get(8),
    }))
}
