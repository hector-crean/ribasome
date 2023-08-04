use crate::{
    errors::authentication::SignupError, models::user::User, services::DatabaseError, AppState,
};

use axum::{extract::State, response::Json};

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, DatabaseError> {
    let users = sqlx::query_as::<_, User>(r#"select * from "user"#)
        .fetch_all(&state.pool)
        .await
        .expect("Unable to collect users");

    Ok(Json(users))
}
