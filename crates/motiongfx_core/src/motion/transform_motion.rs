pub use motiongfx_core_macros::TransformMotion;

use bevy::prelude::*;

use crate::{act, prelude::Action};

use super::GetId;

pub trait TransformMotion: GetId {
    fn get_transform(&mut self) -> &mut Transform;

    fn to_transform(&mut self, transfrom: Transform) -> Action<Transform, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { *self.get_transform() },
            end = transfrom,
        )
    }

    fn to_translation(&mut self, translation: Vec3) -> Action<Vec3, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.translation,
            end = translation,
        )
    }

    fn to_scale(&mut self, scale: Vec3) -> Action<Vec3, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.scale,
            end = scale,
        )
    }

    fn to_rotation(&mut self, rotation: Quat) -> Action<Quat, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.rotation,
            end = rotation,
        )
    }
}
