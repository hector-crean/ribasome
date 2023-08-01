use crate::{
    models::user::{Role, User},
    services::DatabaseError,
    AppState,
};
use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(CreateUser {
        username,
        email,
        password_hash,
        role,
    }): Json<CreateUser>,
) -> Result<Json<User>, DatabaseError> {
    let user: User = sqlx::query_as(
        r#"insert into "user"(username, email, password_hash, role) values ($1, $2, $3, $4) returning user_id, username, email, password_hash, role, updated_at"#,
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .fetch_one(&state.pool)
    .await
    .expect("Unable to insert a user");

    tracing::debug!("create user: {:?}", user);

    Ok(Json(user))
}
