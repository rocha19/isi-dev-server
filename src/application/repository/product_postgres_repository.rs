use crate::domain::entity::discount_entity::{PaginatedResponse, PaginationMeta};
use crate::domain::entity::product_entity::{CreateProduct, Product, UpdateProduct};
use crate::domain::repository::product_repository::ProductRepository;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::Row;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresProductRepository {
    pool: Arc<PgPool>,
}

impl PostgresProductRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn find(&self, id: String) -> Result<Product, String> {
        let uuid = Uuid::parse_str(&id).map_err(|_| "Invalid UUID".to_string())?;
        let row = sqlx::query(
            r#"SELECT p.id, p.name, p.description, p.stock, p.price,
                      p.created_at, p.updated_at, p.deleted_at
               FROM products p
               WHERE p.id = $1 AND p.deleted_at IS NULL"#,
        )
        .bind(uuid)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                "Product not found".to_string()
            } else {
                e.to_string()
            }
        })?;

        Ok(Product {
            id: row.get::<Uuid, _>("id"),
            name: row.get::<String, _>("name"),
            description: row.get::<Option<String>, _>("description"),
            stock: row.get::<i32, _>("stock") as u32,
            price: row.get::<i32, _>("price") as u64,
            created_at: row.get::<chrono::NaiveDateTime, _>("created_at").and_utc(),
            updated_at: row
                .get::<Option<chrono::NaiveDateTime>, _>("updated_at")
                .map(|dt| dt.and_utc()),
            deleted_at: row
                .get::<Option<chrono::NaiveDateTime>, _>("deleted_at")
                .map(|dt| dt.and_utc()),
        })
    }

    async fn find_all(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        search: String,
        min_price: u32,
        max_price: u32,
        has_discount: bool,
    ) -> Result<PaginatedResponse<Product>, String> {
        let page = page.unwrap_or(1) as i64;
        let limit = limit.unwrap_or(10) as i64;
        let offset = (page - 1) * limit;
        let search_pattern = format!("%{}%", search.to_lowercase());

        let discount_condition = if has_discount {
            "AND EXISTS (
                SELECT 1 FROM product_coupon_applications pc
                WHERE pc.product_id = p.id AND pc.removed_at IS NULL
            )"
        } else {
            ""
        };

        let condition_str = format!(
            "p.deleted_at IS NULL
             {discount_condition}
             AND (LOWER(p.name) LIKE $1 OR LOWER(COALESCE(p.description, '')) LIKE $1)
             AND p.price BETWEEN $2 AND $3",
            discount_condition = discount_condition
        );

        let query = format!(
            r#"SELECT p.id, p.name, p.description, p.stock, p.price,
                       p.created_at, p.updated_at, p.deleted_at
               FROM products p
               WHERE {}
               ORDER BY p.created_at DESC
               LIMIT $4 OFFSET $5"#,
            condition_str
        );

        let rows = sqlx::query(&query)
            .bind(&search_pattern)
            .bind(min_price as i64)
            .bind(max_price as i64)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut products = Vec::with_capacity(rows.len());
        for row in rows {
            products.push(Product {
                id: row.get::<Uuid, _>("id"),
                name: row.get::<String, _>("name"),
                description: row.get::<Option<String>, _>("description"),
                stock: row.get::<i32, _>("stock") as u32,
                price: row.get::<i32, _>("price") as u64,
                created_at: row.get::<chrono::NaiveDateTime, _>("created_at").and_utc(),
                updated_at: row
                    .get::<Option<chrono::NaiveDateTime>, _>("updated_at")
                    .map(|dt| dt.and_utc()),
                deleted_at: row
                    .get::<Option<chrono::NaiveDateTime>, _>("deleted_at")
                    .map(|dt| dt.and_utc()),
            });
        }

        let count_query = format!("SELECT COUNT(*) FROM products p WHERE {}", condition_str);

        let total_row: i32 = sqlx::query_scalar(&count_query)
            .bind(&search_pattern)
            .bind(min_price as i32)
            .bind(max_price as i32)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let total_items = total_row as u64;
        let total_pages = ((total_items as f64) / (limit as f64)).ceil() as u32;

        Ok(PaginatedResponse {
            data: products,
            meta: PaginationMeta {
                page: page as u32,
                limit: limit as u32,
                total_items,
                total_pages,
            },
        })
    }

    async fn create(&self, create: CreateProduct) -> Result<Product, String> {
        let now = Utc::now();
        let row = sqlx::query(
            r#"INSERT INTO products (name, description, stock, price, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6)
               RETURNING id, name, description, stock, price, created_at, updated_at, deleted_at"#,
        )
        .bind(&create.name)
        .bind(&create.description)
        .bind(create.stock as i32)
        .bind(create.price as i32)
        .bind(now.naive_utc())
        .bind(now.naive_utc())
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("products_name_key") {
                    return "Product already exists".to_string();
                }
            }
            e.to_string()
        })?;

        Ok(Product {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            stock: row.get::<i32, _>("stock") as u32,
            price: row.get::<i32, _>("price") as u64,
            created_at: row.get::<chrono::NaiveDateTime, _>("created_at").and_utc(),
            updated_at: row
                .get::<Option<chrono::NaiveDateTime>, _>("updated_at")
                .map(|dt| dt.and_utc()),
            deleted_at: row
                .get::<Option<chrono::NaiveDateTime>, _>("deleted_at")
                .map(|dt| dt.and_utc()),
        })
    }

    async fn update(&self, id: String, update: UpdateProduct) -> Result<Product, String> {
        let uuid = Uuid::parse_str(&id).map_err(|_| "Invalid UUID".to_string())?;
        let now = Utc::now();
        let row = sqlx::query(
            r#"UPDATE products SET
                  name = COALESCE($1, name),
                  description = COALESCE($2, description),
                  stock = COALESCE($3, stock),
                  price = COALESCE($4, price),
                  updated_at = $5,
                  deleted_at = $6
               WHERE id = $7
               RETURNING id, name, description, stock, price, created_at, updated_at, deleted_at"#,
        )
        .bind(update.name)
        .bind(update.description)
        .bind(update.stock.map(|s| s as i32))
        .bind(update.price.map(|p| p as i32))
        .bind(now.naive_utc())
        .bind(update.deleted_at.map(|dt| dt.naive_utc()))
        .bind(uuid)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                "Product not found".to_string()
            } else {
                e.to_string()
            }
        })?;

        Ok(Product {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            stock: row.get::<i32, _>("stock") as u32,
            price: row.get::<i32, _>("price") as u64,
            created_at: row.get::<chrono::NaiveDateTime, _>("created_at").and_utc(),
            updated_at: row
                .get::<Option<chrono::NaiveDateTime>, _>("updated_at")
                .map(|dt| dt.and_utc()),
            deleted_at: row
                .get::<Option<chrono::NaiveDateTime>, _>("deleted_at")
                .map(|dt| dt.and_utc()),
        })
    }

    async fn delete(&self, id: String) -> Result<(), String> {
        let uuid = Uuid::parse_str(&id).map_err(|_| "Invalid UUID".to_string())?;
        let now = Utc::now().naive_utc();

        let result =
            sqlx::query("UPDATE products SET deleted_at = $1 WHERE id = $2 AND deleted_at IS NULL")
                .bind(now)
                .bind(uuid)
                .execute(&*self.pool)
                .await
                .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err("Product not found".to_string());
        }

        Ok(())
    }

    async fn has_discount(&self, product_id: String) -> bool {
        let uuid = match Uuid::parse_str(&product_id) {
            Ok(u) => u,
            Err(_) => return false,
        };

        match sqlx::query_scalar::<_, bool>(
            r#"
        SELECT EXISTS(
            SELECT 1 FROM product_coupon_applications
            WHERE product_id = $1 
            AND removed_at IS NULL
        )
        "#,
        )
        .bind(uuid)
        .fetch_one(&*self.pool)
        .await
        {
            Ok(exists) => exists,
            Err(_) => false,
        }
    }
}
