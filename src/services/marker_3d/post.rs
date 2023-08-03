use sqlx::{Pool, Postgres};

use crate::{
    models::{
        linear_algebra::{Vec3, Vec3Array},
        marker_3d::Marker3dKind,
    },
    services::DatabaseError,
    AppState,
};
use axum::{extract::State, response::Json};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CreateMarker3d {
    Point3d { coord: Vec3 },
    Polyline3d { coords: Vec3Array },
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct CreateMarker3dResponse {
    pub marker_id: uuid::Uuid,
}

pub async fn insert_marker_3d(
    pool: &Pool<Postgres>,
    create_marker_3d: CreateMarker3d,
) -> Result<CreateMarker3dResponse, DatabaseError> {
    let respnse = match create_marker_3d {
        CreateMarker3d::Point3d { coord } => {
            let result = sqlx::query_as(
                r#"
                WITH inserted_point AS (
                    INSERT INTO point_3d (coord)
                    VALUES ($1)
                    RETURNING point_id
                ),
                inserted_marker AS (
                    INSERT INTO marker_3d (marker_3d_kind, point_3d_id)
                    SELECT $2 as marker_3d_kind, point_id FROM inserted_point
                    RETURNING marker_id
                )
                SELECT * FROM inserted_marker
                "#,
            )
            .bind(coord)
            .bind(Marker3dKind::Point3d as Marker3dKind)
            .fetch_one(pool)
            .await?;

            tracing::debug!("point created: {:?}", result);

            result
        }
        CreateMarker3d::Polyline3d { coords } => {
            let result = sqlx::query_as(
                r#"
                WITH inserted_polyline AS (
                    INSERT INTO polyline_3d (coords)
                    VALUES ($1)
                    RETURNING polyline_id
                ),
                inserted_marker AS (
                    INSERT INTO marker_3d (marker_3d_kind, polyline_3d_id)
                    SELECT $2 as marker_3d_kind, polyline_id FROM inserted_polyline
                    RETURNING marker_id
                )
                SELECT * FROM inserted_marker
                "#,
            )
            .bind(coords)
            .bind(Marker3dKind::Polyline3d as Marker3dKind)
            .fetch_one(pool)
            .await?;

            tracing::debug!("polyline marker: {:?}", result);

            result
        }
    };
    Ok(respnse)
}

pub async fn create_marker_3d(
    State(state): State<AppState>,
    Json(create_marker_3d): Json<CreateMarker3d>,
) -> Result<Json<CreateMarker3dResponse>, DatabaseError> {
    let respnse = insert_marker_3d(&state.pool, create_marker_3d).await?;

    Ok(Json(respnse))
}
