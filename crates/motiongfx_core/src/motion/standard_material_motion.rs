pub use motiongfx_core_macros::StandardMaterialMotion;

use bevy::prelude::*;

use crate::{act, prelude::Action};

use super::GetId;

pub trait StandardMaterialMotion: GetId {
    fn std_material(&mut self) -> StandardMaterialMotionBuilder;
}

pub struct StandardMaterialMotionBuilder<'a> {
    pub id: Entity,
    pub material: &'a mut StandardMaterial,
}

impl<'a> StandardMaterialMotionBuilder<'a> {
    pub fn new(id: Entity, material: &'a mut StandardMaterial) -> Self {
        Self { id, material }
    }

    pub fn to_emissive(&mut self, color: Color) -> Action<Color, StandardMaterial> {
        act!(
            (self.id, StandardMaterial),
            start = { self.material }.emissive,
            end = color,
        )
    }

    pub fn to_base_color(&mut self, color: Color) -> Action<Color, StandardMaterial> {
        act!(
            (self.id, StandardMaterial),
            start = { self.material }.base_color,
            end = color,
        )
    }
}
