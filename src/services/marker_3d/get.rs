

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::{
        coord::{Coords, Vec3f64},
    },
    services::DatabaseError,
};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GetMarker3d {
    FromPost { post_id: Uuid },
    FromComment { comment_id: Uuid },
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct GetMarkers3dResponse {
    pub marker_id: uuid::Uuid,
}

// New function to fetch all markers associated with a post_id
pub async fn get_markers_by_post_id(
    pool: &Pool<Postgres>,
    post_id: uuid::Uuid,
) -> Result<Vec<GetMarkers3dResponse>, DatabaseError> {
    let markers = sqlx::query_as::<_, GetMarkers3dResponse>(
        r#"
        SELECT m3.marker_id
        FROM marker_3d m3
        JOIN post p ON m3.marker_id = p.marker_id
        WHERE p.post_id = $1;
        "#,
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;

    Ok(markers)
}

#[derive(Debug, sqlx::FromRow)]
pub struct Point3dRow {
    pub point_id: uuid::Uuid,
    pub coord: Vec3f64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Polyline3dRow {
    pub polyline_id: uuid::Uuid,
    pub coords: Coords,
}

pub async fn get_point_3d_by_marker_id(
    pool: &Pool<Postgres>,
    marker_id: uuid::Uuid,
) -> Result<Option<Point3dRow>, DatabaseError> {
    let point_3d = sqlx::query_as::<_, Point3dRow>(
        r#"
        SELECT point_id, coord
        FROM point_3d
        WHERE point_id = (
            SELECT point_3d_id
            FROM marker_3d
            WHERE marker_id = $1
        );
        "#,
    )
    .bind(marker_id)
    .fetch_optional(pool)
    .await?;

    Ok(point_3d)
}

pub async fn get_polyline_3d_by_marker_id(
    pool: &Pool<Postgres>,
    marker_id: uuid::Uuid,
) -> Result<Option<Polyline3dRow>, DatabaseError> {
    let polyline_3d = sqlx::query_as::<_, Polyline3dRow>(
        r#"
        SELECT polyline_id, coords
        FROM polyline_3d
        WHERE polyline_id = (
            SELECT polyline_3d_id
            FROM marker_3d
            WHERE marker_id = $1
        );
        "#,
    )
    .bind(marker_id)
    .fetch_optional(pool)
    .await?;

    Ok(polyline_3d)
}

// pub async fn get_markers_by_post_id(
//     pool: &Pool<Postgres>,
//     post_id: uuid::Uuid,
// ) -> Result<Vec<GetMarkers3dResponse>, DatabaseError> {
//     let markers = sqlx::query_as::<_, GetMarkers3dResponse>(
//         r#"
//         WITH markers_cte AS (
//             SELECT m3.marker_id,
//                    m3.marker_3d_kind,
//                    m3.point_3d_id,
//                    m3.polyline_3d_id
//             FROM marker_3d m3
//             JOIN post p ON m3.marker_id = p.marker_id
//             WHERE p.post_id = $1
//         )
//         SELECT m.marker_id,
//                m.marker_3d_kind,
//                m.point_3d_id,
//                m.polyline_3d_id,
//                p.point_id,
//                p.coord,
//                pl.polyline_id,
//                pl.coords
//         FROM markers_cte m
//         LEFT JOIN point_3d p ON m.point_3d_id = p.point_id
//         LEFT JOIN polyline_3d pl ON m.polyline_3d_id = pl.polyline_id;
//         "#,
//     )
//     .bind(post_id)
//     .fetch_all(pool)
//     .await?;

//     Ok(markers)
// }
