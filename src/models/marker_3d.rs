#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "marker_3d_kind")] // only for PostgreSQL to match a type definition
#[sqlx(rename_all = "lowercase")]
pub enum Marker3dKind {
    Polyline3d,
    Point3d,
}
