pub mod models;
pub mod repository;

use sqlx::{PgPool, postgres::PgDatabaseError};

pub async fn create_pool(
    url: &String
) -> Result<PgPool, PgDatabaseError> {
    todo!()
}
