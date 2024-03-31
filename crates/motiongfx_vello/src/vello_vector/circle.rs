use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::{prelude::*, vello::kurbo};

use crate::{fill_style::FillStyle, stroke_style::StrokeStyle, vello_vector::VelloVector};

#[derive(Bundle, Default, Clone)]
pub struct VelloCircleBundle {
    pub circle: VelloCircle,
    pub fill: FillStyle,
    pub stroke: StrokeStyle,
    pub scene_bundle: VelloSceneBundle,
}

#[derive(VelloVector, Component, Default, Clone)]
pub struct VelloCircle {
    #[shape]
    pub circle: kurbo::Circle,
}

impl VelloCircle {
    #[inline]
    pub fn new(circle: kurbo::Circle) -> Self {
        Self {
            circle,
            ..default()
        }
    }

    pub fn from_vec(center: DVec2, radius: f64) -> Self {
        Self::new(kurbo::Circle::new(
            kurbo::Point::new(center.x, center.y),
            radius,
        ))
    }

    #[inline]
    pub fn from_radius(radius: f64) -> Self {
        Self::new(kurbo::Circle::new(kurbo::Point::default(), radius))
    }
}
