use std::str::FromStr;

use crate::{
    errors::authentication::SignupError,
    models::user::{Role, User},
    services::DatabaseError,
    AppState,
};
use axum::{extract::State, response::Json};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand_core::{OsRng, RngCore};
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
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let password_hash = Pbkdf2.hash_password(password.as_bytes(), &salt);

    let hashed_password = if let Ok(password) = password_hash {
        password.to_string()
    } else {
        return Err(SignupError::InvalidPassword);
    };

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
