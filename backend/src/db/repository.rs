use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::Post;

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, post: &Post) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO posts (id, content, created_at, updated_at)
                VALUES ($1, $2, $3, $4);
            "#,
            post.id(),
            post.content(),
            post.created_at(),
            post.updated_at(),
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) ->  Result<Option<Post>, sqlx::Error> {
        todo!()
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sqlx::Error> {
        todo!()
    }

    pub async fn get_posts(
        &self,
        limit: u64,
        offset: u64
    ) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as!(
            Post,
            r#"
                SELECT id, content, created_at, updated_at
                FROM posts
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64,
        )
            .fetch_all(&self.pool)
        .await
    }
}
