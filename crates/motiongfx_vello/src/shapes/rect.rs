use bevy_ecs::prelude::*;
use bevy_math::{DVec2, DVec4};
use bevy_vello_renderer::vello::kurbo;
use motiongfx_core::prelude::Action;

// impl Into<kurbo::Point> for DVec2 {
//     fn into(self) -> kurbo::Point {
//         todo!()
//     }
// }

#[derive(Component)]
pub struct Rect {
    half_size: DVec2,
    radius: DVec4,
}

impl Rect {
    pub fn new(half_size: DVec2, radius: DVec4) -> Self {
        Self { half_size, radius }
    }

    pub fn kurbo_rect(&self) -> kurbo::RoundedRect {
        kurbo::RoundedRect::new(
            -self.half_size.x,
            -self.half_size.y,
            self.half_size.x,
            self.half_size.y,
            self.radius,
        )
    }
}

pub struct RectMotion {
    target_id: Entity,
    rect: Rect,
}

impl RectMotion {
    pub fn new(target_id: Entity, rect: Rect) -> Self {
        Self { target_id, rect }
    }

    // pub fn size_to(&mut self, half_size: DVec2) -> Action<Rect, Quat, EmptyRes> {}
}
