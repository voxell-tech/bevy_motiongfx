use bevy::prelude::*;
use bevy_vello_renderer::vello::kurbo;
use motiongfx_core::f32lerp::F32Lerp;

use super::VelloVector;

#[derive(Component, Default, Clone, Copy)]
pub struct VelloCircle {
    pub radius: f64,
}

impl VelloCircle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

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

impl F32Lerp for VelloCircle {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        VelloCircle {
            radius: f64::lerp(self.radius, rhs.radius, t as f64),
        }
    }
}
