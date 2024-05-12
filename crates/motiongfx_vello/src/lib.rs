pub use bevy_vello_renderer;

use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::{prelude::*, vello::kurbo};
use motiongfx_core::{sequence::update_component, UpdateSequenceSet};
use vector::{
    bezpath::VelloBezPath, build_vector, circle::VelloCircle, line::VelloLine, rect::VelloRect,
    Brush, Fill, Stroke,
};

pub mod motion;
pub mod svg;
pub mod vector;

pub mod prelude {
    pub use crate::{
        motion::{
            fill_motion::BuildFillMotionExt, fill_stroke_motion::BuildFillStrokeMotionExt,
            stroke_motion::BuildStrokeMotionExt, AddVelloSceneCommandExt,
        },
        vector::{
            bezpath::VelloBezPath, circle::VelloCircle, line::VelloLine, rect::VelloRect, Brush,
            Fill, Stroke,
        },
        MotionGfxVelloPlugin,
    };

    pub use bevy_vello_renderer::prelude::*;
}

pub struct MotionGfxVelloPlugin;

impl Plugin for MotionGfxVelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloRenderPlugin).add_systems(
            Update,
            (
                // Vector builders
                build_vector::<VelloRect>(),
                build_vector::<VelloCircle>(),
                build_vector::<VelloLine>(),
                build_vector::<VelloBezPath>(),
                // Sequence updates
                // Fill & Stroke
                update_component::<Fill, Brush>,
                update_component::<Stroke, Brush>,
                update_component::<Stroke, kurbo::Stroke>,
                update_component::<Stroke, f64>,
                // VelloCircle
                update_component::<VelloCircle, VelloCircle>,
                update_component::<VelloCircle, f64>,
                // VelloRect
                update_component::<VelloRect, VelloRect>,
                update_component::<VelloRect, DVec2>,
                update_component::<VelloRect, f64>,
                // VelloLine
                update_component::<VelloLine, VelloLine>,
                update_component::<VelloLine, DVec2>,
                update_component::<VelloLine, f64>,
                // VelloBezPath
                update_component::<VelloBezPath, f32>,
            )
                .in_set(UpdateSequenceSet),
        );
    }
}
