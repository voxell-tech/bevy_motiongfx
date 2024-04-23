pub use bevy_vello_renderer;

use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::{prelude::*, vello::kurbo};
use motiongfx_core::sequence::update_sequence;
use vello_vector::{
    bezpath::VelloBezPath, build_vector, circle::VelloCircle, line::VelloLine, rect::VelloRect,
    Brush, Fill, Stroke,
};

pub mod svg;
pub mod vello_vector;

pub mod prelude {
    pub use crate::{
        vello_vector::{
            bezpath::VelloBezPath, circle::VelloCircle, line::VelloLine, rect::VelloRect, Brush,
            Fill, Stroke,
        },
        MotionGfxVello,
    };

    pub use bevy_vello_renderer::prelude::*;
}

pub struct MotionGfxVello;

impl Plugin for MotionGfxVello {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloRenderPlugin).add_systems(
            PostUpdate,
            (
                // Vector builders
                build_vector::<VelloRect>,
                build_vector::<VelloCircle>,
                build_vector::<VelloLine>,
                build_vector::<VelloBezPath>,
                // Sequences
                update_sequence::<Fill, Brush>,
                update_sequence::<Stroke, Brush>,
                update_sequence::<Stroke, kurbo::Stroke>,
                update_sequence::<VelloCircle, f64>,
                update_sequence::<VelloRect, DVec2>,
                update_sequence::<VelloLine, DVec2>,
                update_sequence::<VelloBezPath, f32>,
            ),
        );
    }
}
