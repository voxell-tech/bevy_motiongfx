use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo, peniko},
};
use motiongfx_core::{action::Action, EmptyRes};

use crate::{
    fill_style::FillStyle,
    impl_builder_macros::{
        impl_brush_builder, impl_optional_stroke_builder, impl_transform_motion,
    },
    stroke_style::StrokeStyle,
    vello_vector::VelloVector,
};

use super::_VelloVector;

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

#[derive(Component, Default, Clone)]
pub struct _VelloCircle {
    pub radius: f64,
    // Fill
    pub fill_brush: peniko::Brush,
    pub fill_transform: Option<kurbo::Affine>,
    // Stroke
    pub stroke: Option<kurbo::Stroke>,
    pub stroke_brush: peniko::Brush,
    pub stroke_transform: Option<kurbo::Affine>,
}

impl _VelloCircle {
    pub fn new(radius: f64) -> Self {
        Self::default().with_radius(radius)
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
}

impl_brush_builder!(fill, _VelloCircle, fill_brush);
impl_brush_builder!(stroke, _VelloCircle, stroke_brush);
impl_optional_stroke_builder!(_VelloCircle, stroke);

impl _VelloVector for _VelloCircle {
    fn build_scene(&self) -> bevy_vello_renderer::vello::Scene {
        let mut scene = vello::Scene::new();

        let circle = kurbo::Circle::new(kurbo::Point::default(), self.radius);

        scene.fill(
            peniko::Fill::NonZero,
            default(),
            &self.fill_brush,
            self.fill_transform,
            &circle,
        );

        if let Some(stroke) = &self.stroke {
            scene.stroke(
                stroke,
                default(),
                &self.stroke_brush,
                self.stroke_transform,
                &circle,
            );
        }

        scene
    }
}

pub struct _VelloCircleMotion {
    target_id: Entity,
    circle: _VelloCircle,
    transform: Transform,
}

impl _VelloCircleMotion {
    pub fn new(target_id: Entity, circle: _VelloCircle, transform: Transform) -> Self {
        Self {
            target_id,
            circle,
            transform,
        }
    }

    pub fn radius_to(&mut self, radius: f64) -> Action<_VelloCircle, f64, EmptyRes> {
        let action = Action::new(
            self.target_id,
            self.circle.radius,
            radius,
            |circle: &mut _VelloCircle, begin, end, t, _| {
                circle.radius = f64::lerp(*begin, *end, t as f64);
            },
        );

        self.circle.radius = radius;
        action
    }
}

impl_transform_motion!(_VelloCircleMotion, transform, target_id);
