use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Comment {
    pub comment_id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub marker_id: Option<Uuid>,
    pub rich_text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
