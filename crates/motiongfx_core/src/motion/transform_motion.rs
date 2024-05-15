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

    fn to_translation_x(&mut self, x: f32) -> Action<f32, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.translation.x,
            end = x,
        )
    }

    fn to_translation_y(&mut self, y: f32) -> Action<f32, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.translation.y,
            end = y,
        )
    }

    fn to_translation_z(&mut self, z: f32) -> Action<f32, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.translation.z,
            end = z,
        )
    }

    fn to_scale(&mut self, scale: Vec3) -> Action<Vec3, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.scale,
            end = scale,
        )
    }

    fn to_scale_x(&mut self, x: f32) -> Action<f32, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.scale.x,
            end = x,
        )
    }

    fn to_scale_y(&mut self, y: f32) -> Action<f32, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.scale.y,
            end = y,
        )
    }

    fn to_scale_z(&mut self, z: f32) -> Action<f32, Transform> {
        act!(
            (self.get_id(), Transform),
            start = { self.get_transform() }.scale.z,
            end = z,
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
