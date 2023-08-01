use crate::services::marker_3d::post::{insert_marker_3d, CreateMarker3d, CreateMarker3dResponse};
use crate::{models::thread::Post, services::DatabaseError, AppState};
use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::string::ToString;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct CreatePost {
    pub user_id: Uuid,
    pub rich_text: String,
    pub create_marker_3d: Option<CreateMarker3d>,
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(CreatePost {
        user_id,
        rich_text,
        create_marker_3d,
    }): Json<CreatePost>,
) -> Result<Json<Post>, DatabaseError> {
    let marker_id = match create_marker_3d {
        Some(create_marker_3d) => {
            let CreateMarker3dResponse { marker_id } =
                insert_marker_3d(&state.pool, create_marker_3d).await?;
            Some(marker_id)
        }
        None => None,
    };

    // Insert the new post into the database
    let inserted_post: Post = sqlx::query_as(
        r#"
        INSERT INTO post (user_id, marker_id, rich_text)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(marker_id)
    .bind(rich_text)
    .fetch_one(&state.pool)
    .await
    .expect("Failed to insert post");

    Ok(Json(inserted_post))
}
