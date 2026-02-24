use sqlx::{PgPool, postgres::PgDatabaseError};
use uuid::Uuid;

use crate::db::models::Post;

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn save(&self, post: &Post) -> Result<(), PgDatabaseError> {
        sqlx::query!(r#"
            INSERT INTO posts (id, content, created_at, updated_at)
            VALUES (?, ?, ?, ?);
        "#,
        )
    }

    async fn find_by_id(
        &self,
        id: &Uuid,
    ) ->  Result<Option<Post>, PgDatabaseError> {
        todo!()
    }
    async fn delete(&self, id: &Uuid) -> Result<(), PgDatabaseError> {
        todo!()
    }
}
