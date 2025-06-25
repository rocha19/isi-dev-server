use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Row, postgres::PgPool};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entity::{
        coupon_entity::{Coupon, CouponType},
        discount_entity::ProductDiscount,
    },
    repository::discount_repository::DiscountRepository,
};

pub struct PostgresDiscountRepository {
    pool: Arc<PgPool>,
}

impl PostgresDiscountRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DiscountRepository for PostgresDiscountRepository {
    async fn apply_coupon(
        &self,
        product_id: String,
        coupon_code: String,
    ) -> Result<ProductDiscount, String> {
        let now = Utc::now().naive_utc();
        let mut transaction = self.pool.begin().await.map_err(|e| e.to_string())?;

        let coupon_id: Uuid = match sqlx::query(
            r#"
            SELECT id FROM coupons 
            WHERE code = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(&coupon_code)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?
        {
            Some(row) => row.get("id"),
            None => return Err("Coupon not found".to_string()),
        };

        let valid_coupon = sqlx::query(
            r#"
            SELECT id FROM coupons
            WHERE id = $1
            AND deleted_at IS NULL
            AND valid_from <= $2
            AND valid_until >= $2
            AND (max_uses IS NULL OR uses_count < max_uses)
            FOR UPDATE
            "#,
        )
        .bind(coupon_id)
        .bind(now)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;

        if valid_coupon.is_none() {
            return Err("Coupon is not valid".to_string());
        }

        let application_id = Uuid::new_v4();
        let result = sqlx::query(
            r#"
            INSERT INTO product_coupon_applications (
                id, product_id, coupon_id, applied_at
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id, product_id, coupon_id, applied_at, removed_at
            "#,
        )
        .bind(application_id)
        .bind(product_id)
        .bind(coupon_id)
        .bind(now)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("idx_unique_active_coupon") {
                    return "Product already has an active coupon".to_string();
                }
            }
            e.to_string()
        })?;

        sqlx::query(
            r#"
            UPDATE coupons
            SET uses_count = uses_count + 1
            WHERE id = $1
            "#,
        )
        .bind(coupon_id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;

        transaction.commit().await.map_err(|e| e.to_string())?;

        Ok(ProductDiscount {
            id: result.get("id"),
            product_id: result.get("product_id"),
            coupon_id: result.get("coupon_id"),
            applied_at: result
                .get::<chrono::NaiveDateTime, _>("applied_at")
                .and_utc(),
            removed_at: result
                .get::<Option<chrono::NaiveDateTime>, _>("removed_at")
                .map(|dt| dt.and_utc()),
        })
    }

    async fn remove_coupon(
        &self,
        product_id: String,
        coupon_code: String,
    ) -> Result<ProductDiscount, String> {
        let now = Utc::now().naive_utc();

        let result = sqlx::query(
            r#"
            UPDATE product_coupon_applications
            SET removed_at = $1
            WHERE product_id = $2
            AND coupon_id IN (
                SELECT id FROM coupons
                WHERE code = $3
            )
            AND removed_at IS NULL
            RETURNING id, product_id, coupon_id, applied_at, removed_at
            "#,
        )
        .bind(now)
        .bind(product_id)
        .bind(coupon_code)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                "No active coupon found for product".to_string()
            } else {
                e.to_string()
            }
        })?;

        Ok(ProductDiscount {
            id: result.get("id"),
            product_id: result.get("product_id"),
            coupon_id: result.get("coupon_id"),
            applied_at: result
                .get::<chrono::NaiveDateTime, _>("applied_at")
                .and_utc(),
            removed_at: result
                .get::<Option<chrono::NaiveDateTime>, _>("removed_at")
                .map(|dt| dt.and_utc()),
        })
    }

    async fn find_active_discount(
        &self,
        product_id: String,
    ) -> Result<Option<(ProductDiscount, Coupon)>, String> {
        let product_uuid =
            Uuid::parse_str(&product_id).map_err(|e| format!("Invalid product ID: {}", e))?;

        let row = sqlx::query(
            r#"
        SELECT 
            pca.id, pca.product_id, pca.coupon_id, pca.applied_at, pca.removed_at,
            c.code, c.type AS coupon_type, c.value, c.one_shot, 
            c.valid_from, c.valid_until, c.uses_count, c.max_uses,
            c.created_at AS coupon_created, c.updated_at AS coupon_updated, c.deleted_at
        FROM product_coupon_applications pca
        JOIN coupons c ON c.id = pca.coupon_id
        WHERE pca.product_id = $1 
        AND pca.removed_at IS NULL
        AND c.deleted_at IS NULL
        AND c.valid_from <= NOW() AT TIME ZONE 'UTC'
        AND c.valid_until >= NOW() AT TIME ZONE 'UTC'
        "#,
        )
        .bind(product_uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| {
            log::error!("{}", e);
            e.to_string()
        })?;

        match row {
            Some(row) => {
                let discount = ProductDiscount {
                    id: row.get("id"),
                    product_id: row.get("product_id"),
                    coupon_id: row.get("coupon_id"),
                    applied_at: row.get::<chrono::NaiveDateTime, _>("applied_at").and_utc(),
                    removed_at: row
                        .get::<Option<chrono::NaiveDateTime>, _>("removed_at")
                        .map(|dt| dt.and_utc()),
                };

                let coupon_type: CouponType = row.get("coupon_type");

                let coupon = Coupon {
                    id: discount.coupon_id,
                    code: row.get("code"),
                    coupon_type,
                    value: row.get::<i32, _>("value") as u64,
                    one_shot: row.get("one_shot"),
                    valid_from: row.get::<chrono::NaiveDateTime, _>("valid_from").and_utc(),
                    valid_until: row.get::<chrono::NaiveDateTime, _>("valid_until").and_utc(),
                    uses_count: row.get::<i32, _>("uses_count") as u32,
                    max_uses: row.get::<Option<i32>, _>("max_uses").map(|v| v as u32),
                    created_at: row
                        .get::<chrono::NaiveDateTime, _>("coupon_created")
                        .and_utc(),
                    updated_at: row
                        .get::<Option<chrono::NaiveDateTime>, _>("coupon_updated")
                        .map(|dt| dt.and_utc()),
                    deleted_at: row
                        .get::<Option<chrono::NaiveDateTime>, _>("deleted_at")
                        .map(|dt| dt.and_utc()),
                };

                Ok(Some((discount, coupon)))
            }
            None => Ok(None),
        }
    }
}
