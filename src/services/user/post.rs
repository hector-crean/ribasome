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
    pub password: String,
    pub role: Role,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(CreateUser {
        username,
        email,
        password,
        role,
    }): Json<CreateUser>,
) -> Result<Json<User>, DatabaseError> {
    let user: User = sqlx::query_as!(
        User,
        r#"insert into users(username, email, password, role) values ($1, $2, $3, $4) returning *"#,
        username,
        email,
        password,
        role
    )
    .fetch_one(&state.pool)
    .await
    .expect("Unable to insert a user");

    tracing::debug!("create comment: {:?}", user);

    Ok(Json(user))
}
