use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::linear_algebra::{Quat, Vec3};

pub struct Scene3d {
    pub scene_id: Uuid,
    pub title: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
