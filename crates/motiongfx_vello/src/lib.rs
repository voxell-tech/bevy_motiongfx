pub use bevy_vello_renderer;

use bevy_app::prelude::*;
use bevy_vello_renderer::prelude::*;

pub mod shapes;

pub struct MotionGfxVello;

impl Plugin for MotionGfxVello {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloRenderPlugin);
    }
}
