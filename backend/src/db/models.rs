use chrono::{DateTime, Utc};
use derive_getters::Getters;
use uuid::Uuid;

#[derive(Getters)]
pub struct Post {
    id: Uuid,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Post {
    pub fn new(content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_id(
        id: Uuid,
        content: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            content,
            created_at,
            updated_at,
        }
    }
}

