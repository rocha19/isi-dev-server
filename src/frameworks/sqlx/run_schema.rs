use sqlx::{Executor, PgPool};
use std::{fs, sync::Arc};

pub async fn run_schema(pool: &Arc<PgPool>) -> Result<(), sqlx::Error> {
    let schema_sql = fs::read_to_string("schema.sql").expect("schema.sql não encontrado");
    pool.execute(schema_sql.as_str()).await?;
    Ok(())
}
