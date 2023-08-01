use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub marker_id: Option<Uuid>,
    pub rich_text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
