pub use bevy_vello_renderer;

use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::{prelude::*, vello::kurbo};
use motiongfx_core::sequence::sequence_update_system;

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
                vello_vector::vector_builder_system::<vello_vector::rect::VelloRect>,
                vello_vector::vector_builder_system::<vello_vector::circle::VelloCircle>,
                vello_vector::vector_builder_system::<vello_vector::line::VelloLine>,
                vello_vector::vector_builder_system::<vello_vector::bezpath::VelloBezPath>,
                // Sequences
                sequence_update_system::<vello_vector::Brush, vello_vector::Fill>,
                sequence_update_system::<vello_vector::Brush, vello_vector::Stroke>,
                sequence_update_system::<kurbo::Stroke, vello_vector::Stroke>,
                sequence_update_system::<f64, vello_vector::circle::VelloCircle>,
                sequence_update_system::<DVec2, vello_vector::rect::VelloRect>,
                sequence_update_system::<DVec2, vello_vector::line::VelloLine>,
                sequence_update_system::<f32, vello_vector::bezpath::VelloBezPath>,
            ),
        );
    }
}
