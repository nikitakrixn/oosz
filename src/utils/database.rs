use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn create_pool(database_url: String) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}