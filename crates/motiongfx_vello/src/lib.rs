pub use bevy_vello_graphics;

use bevy::{math::DVec2, prelude::*};
use bevy_vello_graphics::prelude::*;
use motiongfx_core::{sequence::animate_component, UpdateSequenceSet};

pub mod motion;

pub mod prelude {
    pub use crate::motion::{fill_motion::FillMotion, stroke_motion::StrokeMotion};

    pub use bevy_vello_graphics::prelude::*;
}

pub struct MotionGfxVelloPlugin;

impl Plugin for MotionGfxVelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // Fill & Stroke
                animate_component::<Fill, Brush>,
                animate_component::<Stroke, Brush>,
                animate_component::<Stroke, f64>,
                // VelloCircle
                animate_component::<VelloCircle, VelloCircle>,
                animate_component::<VelloCircle, f64>,
                // VelloRect
                animate_component::<VelloRect, VelloRect>,
                animate_component::<VelloRect, DVec2>,
                animate_component::<VelloRect, f64>,
                // VelloLine
                animate_component::<VelloLine, VelloLine>,
                animate_component::<VelloLine, DVec2>,
                animate_component::<VelloLine, f64>,
                // VelloBezPath
                animate_component::<VelloBezPath, f32>,
            )
                .in_set(UpdateSequenceSet),
        );
    }
}
