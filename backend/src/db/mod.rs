pub mod models;
pub mod repository;

use sqlx::PgPool;

pub async fn create_pool(
    url: &str
) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(url).await
}
