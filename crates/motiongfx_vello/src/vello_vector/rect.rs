use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::vello::kurbo;
use motiongfx_core::f32lerp::F32Lerp;

use super::VelloVector;

#[derive(Component, Clone, Copy)]
pub struct VelloRect {
    pub size: DVec2,
    pub anchor: DVec2,
}

impl VelloRect {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            size: DVec2::new(width, height),
            anchor: DVec2::splat(0.5),
        }
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = DVec2::new(width, height);
        self
    }

    pub fn with_anchor(mut self, x: f64, y: f64) -> Self {
        self.anchor = DVec2::new(x, y);
        self
    }
}

impl VelloVector for VelloRect {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::Rect::new(
            -self.size.x * self.anchor.x,
            -self.size.y * self.anchor.y,
            self.size.x * (1.0 - self.anchor.x),
            self.size.y * (1.0 - self.anchor.y),
        )
    }
}

impl F32Lerp for VelloRect {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Self {
            size: DVec2::lerp(self.size, rhs.size, t as f64),
            anchor: DVec2::lerp(self.anchor, rhs.anchor, t as f64),
        }
    }
}
