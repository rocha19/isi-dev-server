use crate::domain::entity::{
    coupon_entity::{Coupon, CreateCoupon, UpdateCoupon},
    discount_entity::{PaginatedResponse, PaginationMeta},
};
use crate::domain::repository::coupon_repository::CouponRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{QueryBuilder, Row, postgres::PgPool};
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresCouponRepository {
    pool: Arc<PgPool>,
}

impl PostgresCouponRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CouponRepository for PostgresCouponRepository {
    async fn create(&self, coupon: CreateCoupon) -> Result<Coupon, String> {
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();

        let row = sqlx::query(
            r#"
            INSERT INTO coupons (
                id, code, type, value, one_shot,
                valid_from, valid_until, max_uses, uses_count, created_at, updated_at
            )
            VALUES ($1, $2, $3::coupon_discount_type, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, code, type, value, one_shot, valid_from,
                      valid_until, uses_count, max_uses, created_at, updated_at, deleted_at
            "#,
        )
        .bind(id)
        .bind(&coupon.code)
        .bind(&coupon.coupon_type)
        .bind(coupon.value as i32)
        .bind(coupon.one_shot)
        .bind(coupon.valid_from)
        .bind(coupon.valid_until)
        .bind(coupon.max_uses.map(|m| m as i32))
        .bind(0i32)
        .bind(now)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("coupons_code_key") {
                    return "Coupon code already exists".to_string();
                }
            }
            e.to_string()
        })?;

        Ok(Self::map_row_to_coupon(row))
    }

    async fn find(&self, code: &str) -> Result<Coupon, String> {
        let row = sqlx::query(
            r#"
            SELECT id, code, type, value, one_shot, valid_from,
                   valid_until, uses_count, max_uses, created_at,
                   updated_at, deleted_at
            FROM coupons
            WHERE code = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(code)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                "Coupon not found".to_string()
            } else {
                e.to_string()
            }
        })?;

        Ok(Self::map_row_to_coupon(row))
    }

    async fn find_all(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        search: Option<String>,
        valid_from: Option<DateTime<Utc>>,
        valid_until: Option<DateTime<Utc>>,
        is_active: Option<bool>,
    ) -> Result<PaginatedResponse<Coupon>, String> {
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        let now = Utc::now();

        let mut count_builder =
            QueryBuilder::new("SELECT COUNT(*) FROM coupons c WHERE c.deleted_at IS NULL");

        if let Some(search_str) = &search {
            count_builder
                .push(" AND LOWER(c.code) LIKE ")
                .push_bind(format!("%{}%", search_str));
        }

        if let Some(vf) = valid_from {
            count_builder.push(" AND c.valid_from >= ").push_bind(vf);
        }

        if let Some(vu) = valid_until {
            count_builder.push(" AND c.valid_until <= ").push_bind(vu);
        }

        if let Some(active) = is_active {
            if active {
                count_builder
                    .push(" AND c.valid_from <= ")
                    .push_bind(now)
                    .push(" AND c.valid_until >= ")
                    .push_bind(now);
            } else {
                count_builder
                    .push(" AND (c.valid_from > ")
                    .push_bind(now)
                    .push(" OR c.valid_until < ")
                    .push_bind(now)
                    .push(")");
            }
        }

        let total_items: i64 = count_builder
            .build_query_scalar()
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut query_builder = QueryBuilder::new(
            r#"
        SELECT id, code, type, value, one_shot, valid_from, valid_until,
               uses_count, max_uses, created_at, updated_at, deleted_at
        FROM coupons c
        WHERE c.deleted_at IS NULL
        "#,
        );

        if let Some(search_str) = &search {
            query_builder
                .push(" AND LOWER(c.code) LIKE ")
                .push_bind(format!("%{}%", search_str));
        }

        if let Some(vf) = valid_from {
            query_builder.push(" AND c.valid_from >= ").push_bind(vf);
        }

        if let Some(vu) = valid_until {
            query_builder.push(" AND c.valid_until <= ").push_bind(vu);
        }

        if let Some(active) = is_active {
            if active {
                query_builder
                    .push(" AND c.valid_from <= ")
                    .push_bind(now)
                    .push(" AND c.valid_until >= ")
                    .push_bind(now);
            } else {
                query_builder
                    .push(" AND (c.valid_from > ")
                    .push_bind(now)
                    .push(" OR c.valid_until < ")
                    .push_bind(now)
                    .push(")");
            }
        }

        query_builder
            .push(" ORDER BY created_at DESC LIMIT ")
            .push_bind(limit as i64)
            .push(" OFFSET ")
            .push_bind(offset as i64);

        let query = query_builder.build();

        let rows = query
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let coupons = rows.into_iter().map(Self::map_row_to_coupon).collect();

        let total_pages = (total_items as f64 / limit as f64).ceil() as u32;

        Ok(PaginatedResponse {
            data: coupons,
            meta: PaginationMeta {
                page,
                limit,
                total_items: total_items as u64,
                total_pages,
            },
        })
    }

    async fn update(&self, code: String, update: UpdateCoupon) -> Result<Coupon, String> {
        let now = Utc::now().naive_utc();

        let row = sqlx::query(
            r#"
            UPDATE coupons SET
                type = COALESCE($1::coupon_discount_type, type),
                value = COALESCE($2, value),
                one_shot = COALESCE($3, one_shot),
                valid_from = COALESCE($4, valid_from),
                valid_until = COALESCE($5, valid_until),
                max_uses = COALESCE($6, max_uses),
                updated_at = $7
            WHERE code = $8 AND deleted_at IS NULL
            RETURNING id, code, type, value, one_shot, valid_from,
                      valid_until, uses_count, max_uses, created_at,
                      updated_at, deleted_at
            "#,
        )
        .bind(update.coupon_type.as_ref().map(|t| t.to_string()))
        .bind(update.value.map(|v| v as i32))
        .bind(update.one_shot)
        .bind(update.valid_from)
        .bind(update.valid_until)
        .bind(update.max_uses.map(|m| m as i32))
        .bind(now)
        .bind(code)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                "Coupon not found".to_string()
            } else {
                e.to_string()
            }
        })?;

        Ok(Self::map_row_to_coupon(row))
    }

    async fn delete(&self, code: String) -> Result<(), String> {
        let now = Utc::now().naive_utc();

        let result = sqlx::query(
            "UPDATE coupons SET deleted_at = $1 WHERE code = $2 AND deleted_at IS NULL",
        )
        .bind(now)
        .bind(code)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err("Coupon not found".to_string());
        }

        Ok(())
    }

    async fn find_valid_coupon_by_code(&self, code: &str) -> Result<Coupon, String> {
        let now = Utc::now();

        let row = sqlx::query(
            r#"
            SELECT id, code, type, value, one_shot, valid_from,
                   valid_until, uses_count, max_uses, created_at,
                   updated_at, deleted_at
            FROM coupons
            WHERE code = $1
              AND deleted_at IS NULL
              AND valid_from <= $2
              AND valid_until >= $2
              AND (max_uses IS NULL OR uses_count < max_uses)
            "#,
        )
        .bind(code)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                "Coupon not found".to_string()
            } else {
                e.to_string()
            }
        })?;

        Ok(Self::map_row_to_coupon(row))
    }

    async fn increment_uses(&self, coupon_id: String) {
        if let Ok(uuid) = Uuid::parse_str(&coupon_id) {
            let _ = sqlx::query("UPDATE coupons SET uses_count = uses_count + 1 WHERE id = $1")
                .bind(uuid)
                .execute(&*self.pool)
                .await;
        }
    }
}

impl PostgresCouponRepository {
    fn map_row_to_coupon(row: sqlx::postgres::PgRow) -> Coupon {
        Coupon {
            id: row.get("id"),
            code: row.get("code"),
            coupon_type: row.get("type"),
            value: row.get::<i32, _>("value") as u64,
            one_shot: row.get("one_shot"),
            valid_from: row.get::<chrono::NaiveDateTime, _>("valid_from").and_utc(),
            valid_until: row.get::<chrono::NaiveDateTime, _>("valid_until").and_utc(),
            uses_count: row.get::<i32, _>("uses_count") as u32,
            max_uses: row.get::<Option<i32>, _>("max_uses").map(|v| v as u32),
            created_at: row.get::<chrono::NaiveDateTime, _>("created_at").and_utc(),
            updated_at: row
                .get::<Option<chrono::NaiveDateTime>, _>("updated_at")
                .map(|dt| dt.and_utc()),
            deleted_at: row
                .get::<Option<chrono::NaiveDateTime>, _>("deleted_at")
                .map(|dt| dt.and_utc()),
        }
    }
}
