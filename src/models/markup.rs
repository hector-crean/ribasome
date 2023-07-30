use bevy::{math::Vec3, render::color::Color};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

enum Marker {
    Sphere {
        centre: Vec3,
        radius: f64,
    },
    Circle {
        centre: Vec3,
        azimuthal: f64,
        radius: f64,
    },
    Polyline {
        points: Vec<Vec3>,
        open: bool,
        fill: Color,
        stroke: Color,
    },
}

enum Linker {
    Arrow,
    Line,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub rich_text: String,
    pub category: Option<String>,
    pub published: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct Markup {
    marker: Option<Marker>,
    linker: Option<Linker>,
    label: Option<Comment>,
}
