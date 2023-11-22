pub use bevy_vello_renderer;

use bevy_app::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{kurbo, peniko},
};
use motiongfx_core::prelude::*;

pub mod convert;
pub mod fill_style;
pub mod stroke_style;
pub mod vello_motion;
pub mod vello_vector;

pub struct MotionGfxVello;

impl Plugin for MotionGfxVello {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloRenderPlugin)
            .add_plugins((vello_motion::rect_motion::VelloRectMotionPlugin,))
            .add_systems(
                PostUpdate,
                (
                    // Vector builders
                    vello_vector::vector_builder::<vello_vector::rect::VelloRect>,
                    vello_vector::vector_builder::<vello_vector::circle::VelloCircle>,
                    // Sequences
                    sequence_player_system::<fill_style::FillStyle, peniko::Brush, EmptyRes>,
                    sequence_player_system::<stroke_style::StrokeStyle, peniko::Brush, EmptyRes>,
                    sequence_player_system::<stroke_style::StrokeStyle, kurbo::Stroke, EmptyRes>,
                ),
            );
    }
}
