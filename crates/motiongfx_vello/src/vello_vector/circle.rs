use bevy_ecs::prelude::*;
use bevy_math::DVec2;
use bevy_utils::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo},
};
use motiongfx_core::prelude::*;

use crate::{
    vector_style::{FillStyle, FillStyleMotion, StrokeStyle, StrokeStyleMotion},
    vello_vector::{VelloBuilder, VelloVector},
};

#[derive(Bundle, Clone, Default)]
pub struct VelloCircleBundle {
    pub circle: VelloCircle,
    pub fill: FillStyle,
    pub stroke: StrokeStyle,
    pub fragment_bundle: VelloFragmentBundle,
}

pub struct VelloCircleBundleMotion {
    pub circle: VelloCircleMotion,
    pub fill: FillStyleMotion,
    pub stroke: StrokeStyleMotion,
}

impl VelloCircleBundleMotion {
    pub fn new(target_id: Entity, bundle: VelloCircleBundle) -> Self {
        Self {
            circle: VelloCircleMotion::new(target_id, bundle.circle),
            fill: FillStyleMotion::new(target_id, bundle.fill),
            stroke: StrokeStyleMotion::new(target_id, bundle.stroke),
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct VelloCircle {
    circle: kurbo::Circle,
    should_build: bool,
}

impl VelloCircle {
    #[inline]
    pub fn new(circle: impl Into<kurbo::Circle>) -> Self {
        let circle: kurbo::Circle = circle.into();

        Self {
            circle,
            ..default()
        }
    }

    pub fn from_vec(center: impl Into<DVec2>, radius: f64) -> Self {
        let center: DVec2 = center.into();

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

impl VelloVector for VelloCircle {
    fn build_fill(&self, fill: &FillStyle, builder: &mut vello::SceneBuilder) {
        builder.fill(
            fill.style,
            kurbo::Affine::default(),
            &fill.brush,
            None,
            &self.circle,
        );
    }

    fn build_stroke(&self, stroke: &StrokeStyle, builder: &mut vello::SceneBuilder) {
        builder.stroke(
            &stroke.style,
            kurbo::Affine::default(),
            &stroke.brush,
            None,
            &self.circle,
        );
    }
}

impl VelloBuilder for VelloCircle {
    #[inline]
    fn should_build(&self) -> bool {
        self.should_build
    }

    #[inline]
    fn set_should_build(&mut self, should_build: bool) {
        self.should_build = should_build
    }
}

pub struct VelloCircleMotion {
    target_id: Entity,
    vello_circle: VelloCircle,
}

impl VelloCircleMotion {
    pub fn new(target_id: Entity, vello_circle: VelloCircle) -> Self {
        Self {
            target_id,
            vello_circle,
        }
    }

    // =====================
    // Circle
    // =====================
    pub fn inflate(&mut self, inflation: f64) -> Action<VelloCircle, kurbo::Circle, EmptyRes> {
        let mut new_circle: kurbo::Circle = self.vello_circle.circle;
        new_circle.radius += inflation;

        let action: Action<VelloCircle, kurbo::Circle, EmptyRes> = Action::new(
            self.target_id,
            self.vello_circle.circle,
            new_circle,
            Self::circle_interp,
        );

        self.vello_circle.circle = new_circle;

        action
    }

    pub fn circle_to(
        &mut self,
        new_circle: impl Into<kurbo::Circle>,
    ) -> Action<VelloCircle, kurbo::Circle, EmptyRes> {
        let new_circle: kurbo::Circle = new_circle.into();

        let action: Action<VelloCircle, kurbo::Circle, EmptyRes> = Action::new(
            self.target_id,
            self.vello_circle.circle,
            new_circle,
            Self::circle_interp,
        );

        self.vello_circle.circle = new_circle;

        action
    }

    fn circle_interp(
        vello_rect: &mut VelloCircle,
        begin: &kurbo::Circle,
        end: &kurbo::Circle,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        vello_rect.circle = kurbo::Circle::lerp(begin, end, t);
        vello_rect.set_should_build(true);
    }
}
