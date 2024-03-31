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

mod impl_builder_macros;

pub mod prelude {
    pub use crate::{
        convert::*,
        fill_style::{FillStyle, FillStyleMotion},
        stroke_style::{StrokeStyle, StrokeStyleMotion},
        vello_motion::{
            bezpath_motion::{VelloBezPathBundleMotion, VelloBezPathMotion},
            circle_motion::{VelloCircleBundleMotion, VelloCircleMotion},
            line_motion::{VelloLineBundleMotion, VelloLineMotion},
            rect_motion::{VelloRectBundleMotion, VelloRectMotion},
        },
        vello_vector::{
            bezpath::{VelloBezPath, VelloBezPathBundle},
            circle::{VelloCircle, VelloCircleBundle},
            line::{VelloLine, VelloLineBundle},
            rect::{RectAnchor, VelloRect, VelloRectBundle, _VelloRect},
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
                vello_motion::circle_motion::VelloCircleMotionPlugin,
                vello_motion::rect_motion::VelloRectMotionPlugin,
                vello_motion::line_motion::VelloLineMotionPlugin,
                vello_motion::bezpath_motion::VelloBezPathMotionPlugin,
            ))
            .add_systems(
                PostUpdate,
                (
                    // Vector builders
                    vello_vector::vector_builder_system::<vello_vector::rect::VelloRect>,
                    vello_vector::vector_builder_system::<vello_vector::circle::VelloCircle>,
                    vello_vector::vector_builder_system::<vello_vector::line::VelloLine>,
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
