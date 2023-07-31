use sqlx::postgres::PgPool;
use sqlx::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "marker_3d_kind", rename_all = "SCREAMING_SNAKE_CASE")]
enum Marker3dKind {
    Polyline3d,
    Point3d,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
struct Marker3d {
    marker_id: uuid::Uuid,
    marker_3d_kind: String,
    point_3d: Option<uuid::Uuid>,
    polyline_3d: Option<uuid::Uuid>,
}

async fn insert_point_3d(pool: &PgPool, point_3d: Option<uuid::Uuid>) -> Result<(), Error> {
    let result = sqlx::query_as!(
        Marker3d,
        r#"INSERT INTO marker_3d (marker_3d_kind, point_3d) VALUES ($1, $2) returning marker_id, marker_3d_kind as "marker_3d_kind!: Marker3dKind", point_3d"#,
        Marker3dKind::Point3d as Marker3dKind,
        point_3d,
    )
    .execute(pool)
    .await?;

    Ok(())
}
