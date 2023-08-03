#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "vec3")]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "quat")]
pub struct Quat {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "vec3_array")]
pub struct Vec3Array(Vec<Vec3>);

impl From<bevy::math::Vec3> for Vec3 {
    fn from(bevy::math::Vec3 { x, y, z }: bevy::math::Vec3) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }
}

impl From<bevy::math::Quat> for Quat {
    fn from(value: bevy::math::Quat) -> Self {
        let [x, y, z, w] = value.to_array();

        Self {
            x: x as f64,
            y: y as f64,
            z: z as f64,
            w: w as f64,
        }
    }
}
