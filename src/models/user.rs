use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    // #[serde(skip_serializing)]
    pub password: String,
    pub role: Role,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "Role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    User,
    Superuser,
    Admin,
    Moderator,
}
