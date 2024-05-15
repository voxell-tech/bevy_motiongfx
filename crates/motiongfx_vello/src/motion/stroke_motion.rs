pub use motiongfx_vello_macros::StrokeMotion;

use bevy_vello_graphics::prelude::*;
use motiongfx_core::{act, action::Action, motion::GetId};

pub trait StrokeMotion: GetId {
    fn get_stroke(&mut self) -> &mut Stroke;

    fn to_width(&mut self, width: f64) -> Action<f64, Stroke> {
        act!(
            (self.get_id(), Stroke),
            start = { self.get_stroke() }.style.width,
            end = width,
        )
    }
}
