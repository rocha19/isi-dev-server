use std::sync::Arc;

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn add_pool(database: String) -> Arc<PgPool> {
    let pool = PgPoolOptions::new().connect(&database).await.unwrap();
    let arc_pool = Arc::new(pool);
    arc_pool
}
