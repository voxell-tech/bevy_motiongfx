use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_vello_renderer::vello::kurbo;
use motiongfx_bevy::prelude::TransformMotion;
use motiongfx_core::{prelude::*, sequence::sequence_update_system};

use crate::{
    fill_style::FillStyleMotion,
    stroke_style::StrokeStyleMotion,
    vello_vector::{
        circle::{VCircle, VCircleBundle},
        VelloBuilder,
    },
};

pub(crate) struct VCircleMotionPlugin;

impl Plugin for VCircleMotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                sequence_update_system::<VCircle, kurbo::Circle, EmptyRes>,
                sequence_update_system::<VCircle, f64, EmptyRes>,
            ),
        );
    }
}

pub struct VCircleBundleMotion {
    pub circle: VCircleMotion,
    pub fill: FillStyleMotion,
    pub stroke: StrokeStyleMotion,
    pub transform: TransformMotion,
}

impl VCircleBundleMotion {
    pub fn new(target_id: Entity, bundle: VCircleBundle) -> Self {
        Self {
            circle: VCircleMotion::new(target_id, bundle.circle),
            fill: FillStyleMotion::new(target_id, bundle.fill),
            stroke: StrokeStyleMotion::new(target_id, bundle.stroke),
            transform: TransformMotion::new(target_id, bundle.scene_bundle.transform),
        }
    }
}

pub struct VCircleMotion {
    target_id: Entity,
    vello_circle: VCircle,
}

impl VCircleMotion {
    pub fn new(target_id: Entity, vello_circle: VCircle) -> Self {
        Self {
            target_id,
            vello_circle,
        }
    }

    // =====================
    // Circle
    // =====================
    pub fn circle_to(
        &mut self,
        new_circle: impl Into<kurbo::Circle>,
    ) -> Action<VCircle, kurbo::Circle, EmptyRes> {
        let new_circle: kurbo::Circle = new_circle.into();

        let action: Action<VCircle, kurbo::Circle, EmptyRes> = Action::new(
            self.target_id,
            self.vello_circle.circle,
            new_circle,
            Self::circle_interp,
        );

        self.vello_circle.circle = new_circle;

        action
    }

    fn circle_interp(
        vello_circle: &mut VCircle,
        begin: &kurbo::Circle,
        end: &kurbo::Circle,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        vello_circle.circle = kurbo::Circle::lerp(begin, end, t);
        vello_circle.set_built(false);
    }

    // =====================
    // Circle.radius
    // =====================
    pub fn inflate(&mut self, inflation: f64) -> Action<VCircle, f64, EmptyRes> {
        let new_radius: f64 = self.vello_circle.circle.radius + inflation;

        let action: Action<VCircle, f64, EmptyRes> = Action::new(
            self.target_id,
            self.vello_circle.circle.radius,
            new_radius,
            Self::circle_radius_interp,
        );

        self.vello_circle.circle.radius = new_radius;

        action
    }

    fn circle_radius_interp(
        vello_circle: &mut VCircle,
        begin: &f64,
        end: &f64,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        vello_circle.circle.radius = f64::lerp(begin, end, t);
        vello_circle.set_built(false);
    }
}
