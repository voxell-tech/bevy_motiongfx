pub use bevy_vello_renderer;

use bevy_app::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{kurbo, peniko},
};
use motiongfx_core::{prelude::*, sequence::sequence_update_system};

pub mod convert;
pub mod fill_style;
pub mod stroke_style;
pub mod svg;
pub mod vello_motion;
pub mod vello_vector;

pub mod prelude {
    pub use crate::{
        convert::*,
        fill_style::{FillStyle, FillStyleMotion},
        stroke_style::{StrokeStyle, StrokeStyleMotion},
        vello_motion::{
            bezpath_motion::{VBezPathBundleMotion, VBezPathMotion},
            circle_motion::{VCircleBundleMotion, VCircleMotion},
            line_motion::{VLineBundleMotion, VLineMotion},
            rect_motion::{VRectBundleMotion, VRectMotion},
        },
        vello_vector::{
            bezpath::{VBezPathBundle, VelloBezPath},
            circle::{VCircle, VCircleBundle},
            line::{VLine, VLineBundle},
            rect::{VRect, VRectBundle},
        },
        MotionGfxVello,
    };
    pub use bevy_vello_renderer::prelude::*;
}

pub struct MotionGfxVello;

impl Plugin for MotionGfxVello {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloRenderPlugin)
            .add_plugins((
                // Motion plugins
                vello_motion::circle_motion::VCircleMotionPlugin,
                vello_motion::rect_motion::VRectMotionPlugin,
                vello_motion::line_motion::VLineMotionPlugin,
                vello_motion::bezpath_motion::VBezPathMotionPlugin,
            ))
            .add_systems(
                PostUpdate,
                (
                    // Vector builders
                    vello_vector::vector_builder_system::<vello_vector::rect::VRect>,
                    vello_vector::vector_builder_system::<vello_vector::circle::VCircle>,
                    vello_vector::vector_builder_system::<vello_vector::line::VLine>,
                    vello_vector::vector_builder_system::<vello_vector::bezpath::VelloBezPath>,
                    // Sequences
                    sequence_update_system::<fill_style::FillStyle, peniko::Brush, EmptyRes>,
                    sequence_update_system::<fill_style::FillStyle, f32, EmptyRes>,
                    sequence_update_system::<stroke_style::StrokeStyle, peniko::Brush, EmptyRes>,
                    sequence_update_system::<stroke_style::StrokeStyle, kurbo::Stroke, EmptyRes>,
                ),
            );
    }
}
