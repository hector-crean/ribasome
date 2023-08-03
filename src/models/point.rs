use geo::Polygon;

use sqlx::FromRow; // or any other types you need from rust-geo

#[derive(Debug, FromRow)]
struct Area {
    id: i32,
    name: String,
    polygon: Polygon, // or any other spatial type you're using
}
