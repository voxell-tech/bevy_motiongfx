use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_vello_renderer::vello::kurbo;
use motiongfx_bevy::prelude::*;
use motiongfx_core::{prelude::*, sequence::sequence_update_system};

use crate::{
    fill_style::FillStyleMotion,
    stroke_style::StrokeStyleMotion,
    vello_vector::{
        bezpath::{VelloBezPath, VelloBezPathBundle},
        VelloBuilder,
    },
};

pub(crate) struct VelloBezPathMotionPlugin;

impl Plugin for VelloBezPathMotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                sequence_update_system::<VelloBezPath, kurbo::Rect, EmptyRes>,
                sequence_update_system::<VelloBezPath, f64, EmptyRes>,
                sequence_update_system::<VelloBezPath, kurbo::RoundedRectRadii, EmptyRes>,
            ),
        );
    }
}

pub struct VelloBezPathBundleMotion {
    pub path: VelloBezPathMotion,
    pub fill: FillStyleMotion,
    pub stroke: StrokeStyleMotion,
    pub transform: TransformMotion,
}

impl VelloBezPathBundleMotion {
    pub fn new(target_id: Entity, bundle: VelloBezPathBundle) -> Self {
        Self {
            path: VelloBezPathMotion::new(target_id, bundle.path),
            fill: FillStyleMotion::new(target_id, bundle.fill),
            stroke: StrokeStyleMotion::new(target_id, bundle.stroke),
            transform: TransformMotion::new(target_id, bundle.scene_bundle.transform),
        }
    }
}

pub struct VelloBezPathMotion {
    target_id: Entity,
    vello_path: VelloBezPath,
    /// Path elements that are animatable (not ClosePath).
    anim_pathels: Vec<usize>,
    trace: f32,
}

pub(crate) struct PathElTrace {
    index: usize,
    t: f32,
}

impl VelloBezPathMotion {
    pub fn new(target_id: Entity, vello_path: VelloBezPath) -> Self {
        let pathels = vello_path.path.elements();
        let mut anim_pathels = Vec::new();

        for p in 0..pathels.len() {
            let pathel = pathels[p];
            if matches!(
                pathel,
                kurbo::PathEl::LineTo(..) | kurbo::PathEl::QuadTo(..) | kurbo::PathEl::CurveTo(..)
            ) {
                anim_pathels.push(p);
            }
        }

        Self {
            target_id,
            vello_path,
            anim_pathels,
            trace: 1.0,
        }
    }

    pub fn trace_to(&mut self, new_trace: f32) -> Action<VelloBezPath, f32, EmptyRes> {
        let pathels_count = self.anim_pathels.len();
        let begin_index = (pathels_count as f32 * self.trace) as usize;
        let end_index = (pathels_count as f32 * new_trace) as usize;

        // Action::new(
        //     self.target_id,

        // )

        todo!()
    }

    fn trace_interp(
        vello_path: &mut VelloBezPath,
        begin: &f32,
        end: &f32,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        let pathels_count = vello_path.origin_path.elements().len();
        let begin_index = (pathels_count as f32 * begin) as usize;
        let end_index = (pathels_count as f32 * end) as usize;

        vello_path.set_built(false);
    }

    fn interp_pathel(p0: kurbo::Point, pathel: kurbo::PathEl, t: f32) -> kurbo::PathEl {
        match pathel {
            kurbo::PathEl::MoveTo(p1) => {
                kurbo::PathEl::MoveTo(kurbo::Point::lerp(p0, p1, t as f64))
            }
            kurbo::PathEl::LineTo(p1) => {
                kurbo::PathEl::LineTo(kurbo::Point::lerp(p0, p1, t as f64))
            }
            kurbo::PathEl::QuadTo(p1, p2) => {
                let t = t as f64;
                // Point between p0 and p1
                let x0 = kurbo::Point::lerp(p0, p1, t);
                // Point between p1 and p2
                let x1 = kurbo::Point::lerp(p1, p2, t);
                // Point on curve
                let end_p = kurbo::Point::lerp(x0, x1, t);

                kurbo::PathEl::QuadTo(x0, end_p)
            }
            kurbo::PathEl::CurveTo(p1, p2, p3) => {
                let t = t as f64;
                // Point between p0 and p1
                let x0 = kurbo::Point::lerp(p0, p1, t);
                // Point between p1 and p2
                let x1 = kurbo::Point::lerp(p1, p2, t);
                // Point between p2 and p3
                let x2 = kurbo::Point::lerp(p2, p3, t);
                // Point between x0 and x1
                let y0 = kurbo::Point::lerp(x0, x1, t);
                // Point between x0 and x2
                let y1 = kurbo::Point::lerp(x1, x2, t);
                // Point on curve
                let end_p = kurbo::Point::lerp(y0, y1, t);

                kurbo::PathEl::CurveTo(x0, y0, end_p)
            }
            kurbo::PathEl::ClosePath => kurbo::PathEl::ClosePath,
        }
    }

    fn pathel_end_point(pathel: &kurbo::PathEl) -> Option<&kurbo::Point> {
        match pathel {
            kurbo::PathEl::MoveTo(p) => Some(p),
            kurbo::PathEl::LineTo(p) => Some(p),
            kurbo::PathEl::QuadTo(_, p) => Some(p),
            kurbo::PathEl::CurveTo(.., p) => Some(p),
            kurbo::PathEl::ClosePath => None,
        }
    }
}
