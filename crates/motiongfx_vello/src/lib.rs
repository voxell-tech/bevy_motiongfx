pub use bevy_vello_renderer;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_vello_renderer::prelude::*;
use motiongfx_core::prelude::*;

pub mod vector_style;
pub mod vello_vector;

pub struct MotionGfxVello;

impl Plugin for MotionGfxVello {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloRenderPlugin)
            .add_systems(
                PostStartup,
                vello_vector::vello_rect_init::<vello_vector::rect::VelloRect>,
            )
            .add_systems(
                PostUpdate,
                sequence_player_system::<
                    Handle<VelloFragment>,
                    vello_vector::rect::VelloRect,
                    Assets<VelloFragment>,
                >,
            );
    }
}
