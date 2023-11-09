use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_vello_renderer::vello::kurbo;
use motiongfx_core::prelude::Action;

pub struct Rect {
    target_id: Entity,
    size: Vec2,
    radius: Vec4,
}

impl Rect {
    pub fn new(target_id: Entity, size: Vec2, radius: Vec4) -> Self {
        Self {
            target_id,
            size,
            radius,
        }
    }

    // pub fn resize_to(&mut self, size: Vec2) -> Action<> {
    //     self.size = size;
    // }

    // pub fn radius_to(&mut self, radius: Vec4) {}
}
