use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub type DbPool = Pool;

pub async fn create_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    let mut cfg = Config::new();
    cfg.dbname = Some(std::env::var("DB_NAME")?);
    cfg.user = Some(std::env::var("DB_USER")?);
    cfg.password = Some(std::env::var("DB_PASSWORD")?);
    cfg.host = Some(std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".into()));
    cfg.port = Some(
        std::env::var("DB_PORT")
            .unwrap_or_else(|_| "5432".into())
            .parse()?,
    );
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}

pub async fn init_db(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let client = pool.get().await?;

    client
        .batch_execute(
            r#"
        CREATE TABLE IF NOT EXISTS products (
            id UUID PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            description VARCHAR(300),
            stock INT NOT NULL CHECK (stock >= 0),
            price BIGINT NOT NULL,
            original_price BIGINT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ,
            deleted_at TIMESTAMPTZ
        );
        
        CREATE TABLE IF NOT EXISTS coupons (
            id UUID PRIMARY KEY,
            code VARCHAR(20) UNIQUE NOT NULL,
            type VARCHAR(10) NOT NULL CHECK (type IN ('fixed', 'percent')),
            value BIGINT NOT NULL,
            one_shot BOOLEAN NOT NULL,
            valid_from TIMESTAMPTZ NOT NULL,
            valid_until TIMESTAMPTZ NOT NULL,
            uses_count INT NOT NULL DEFAULT 0,
            max_uses INT
        );
        
        CREATE TABLE IF NOT EXISTS product_discounts (
            product_id UUID PRIMARY KEY REFERENCES products(id),
            coupon_id UUID REFERENCES coupons(id),
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
        )
        .await?;

    Ok(())
}
