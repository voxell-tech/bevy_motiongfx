use bevy::{math::DVec2, prelude::*};
use bevy_vello::prelude::*;
#[cfg(feature = "motiongfx")]
use motiongfx_core::f32lerp::F32Lerp;

use super::VelloVector;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VelloLine {
    pub p0: DVec2,
    pub p1: DVec2,
}

impl VelloLine {
    pub fn new(p0: DVec2, p1: DVec2) -> Self {
        Self::default().with_p0(p0).with_p1(p1)
    }

    pub fn with_p0(mut self, p0: DVec2) -> Self {
        self.p0 = p0;
        self
    }

    pub fn with_p1(mut self, p1: DVec2) -> Self {
        self.p1 = p1;
        self
    }

    pub fn extend(mut self, extension: f64) -> Self {
        let dir = DVec2::normalize_or_zero(self.p1 - self.p0);
        self.p0 -= dir * extension;
        self.p1 += dir * extension;
        self
    }
}

impl VelloVector for VelloLine {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::Line::new(
            kurbo::Point::new(self.p0.x, self.p0.y),
            kurbo::Point::new(self.p1.x, self.p1.y),
        )
    }
}

#[cfg(feature = "motiongfx")]
impl F32Lerp for VelloLine {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Self {
            p0: DVec2::lerp(self.p0, rhs.p0, t as f64),
            p1: DVec2::lerp(self.p1, rhs.p1, t as f64),
        }
    }
}
