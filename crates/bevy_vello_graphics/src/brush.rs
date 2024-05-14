use bevy::prelude::*;
use bevy_vello_renderer::vello::{kurbo, peniko};

#[derive(Default, Clone)]
pub struct Brush {
    pub value: peniko::Brush,
    pub transform: kurbo::Affine,
}

impl Brush {
    pub fn from_brush(brush: peniko::Brush) -> Self {
        Self {
            value: brush,
            ..default()
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            value: peniko::Brush::Solid(peniko::Color::rgba(
                color.r() as f64,
                color.g() as f64,
                color.b() as f64,
                color.a() as f64,
            )),
            ..default()
        }
    }

    pub fn from_gradient(gradient: peniko::Gradient) -> Self {
        Self {
            value: peniko::Brush::Gradient(gradient),
            ..default()
        }
    }

    pub fn with_transform(mut self, transform: kurbo::Affine) -> Self {
        self.transform = transform;
        self
    }
}
