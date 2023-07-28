use bevy::{math::Vec3, render::color::Color};
use serde::{Deserialize, Serialize};

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

struct Label {
    position: Vec3,
}

struct Markup {
    marker: Option<Marker>,
    linker: Option<Linker>,
    label: Option<Label>,
}
