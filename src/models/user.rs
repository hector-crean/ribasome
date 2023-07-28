use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub is_active: bool,
    pub is_superuser: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Role {
    Admin,
}
