use bevy::prelude::*;
use bevy_vello_renderer::vello::kurbo;

use super::VelloVector;

#[derive(Component, Default, Clone)]
pub struct VelloCircle {
    pub radius: f64,
}

impl VelloCircle {
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
}

impl VelloVector for VelloCircle {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::Circle::new(kurbo::Point::default(), self.radius)
    }
}
