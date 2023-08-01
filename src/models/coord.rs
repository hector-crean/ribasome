#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "vec3_f64")]
pub struct Vec3f64 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "coords")] // only for PostgreSQL to match a type definition
pub struct Coords(Vec<Vec3f64>);
