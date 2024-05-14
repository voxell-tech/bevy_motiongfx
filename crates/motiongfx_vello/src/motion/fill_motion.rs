pub use motiongfx_vello_macros::FillMotion;

use bevy::prelude::*;
use bevy_vello_graphics::prelude::*;
use bevy_vello_renderer::vello::peniko;
use motiongfx_core::{act, motion::GetId, prelude::Action};

pub trait FillMotion: GetId {
    fn get_fill(&mut self) -> &mut Fill;

    fn to_color(&mut self, color: Color) -> Action<peniko::Brush, Fill> {
        act!(
            (self.get_id(), Fill),
            start = { self.get_fill() }.brush.value,
            end = peniko::Brush::Solid(peniko::Color::rgba(
                color.r() as f64,
                color.g() as f64,
                color.b() as f64,
                color.a() as f64
            )),
        )
    }
}
