use crate::{models::markup::Comment, services::DatabaseError, AppState};
use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};


use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateComment {
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub title: String,
    pub rich_text: String,
}

pub async fn create_comment(
    State(state): State<AppState>,
    Json(CreateComment {
        user_id,
        post_id,
        title,
        rich_text,
    }): Json<CreateComment>,
) -> Result<Json<Comment>, DatabaseError> {
    let inserted_comment: Comment = sqlx::query_as(
        "insert into comment(user_id, post_id, title, rich_text) values ($1, $2, $3, $4) returning comment_id")
        .bind(user_id)
        .bind(post_id)
        .bind(title)
        .bind(rich_text)
    .fetch_one(&state.pool)
    .await
    .expect("Unable to insert a comment");

    tracing::debug!("create comment: {:?}", inserted_comment);

    Ok(Json(inserted_comment))
}
