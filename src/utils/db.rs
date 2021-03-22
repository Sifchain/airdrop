use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn connect() -> anyhow::Result<sqlx::postgres::PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    Ok(pool)
}
