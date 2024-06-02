use bevy::prelude::*;

use super::F32Lerp;

impl F32Lerp for Color {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Self::rgba(
            f32::lerp(self.r(), rhs.r(), t),
            f32::lerp(self.g(), rhs.g(), t),
            f32::lerp(self.b(), rhs.b(), t),
            f32::lerp(self.a(), rhs.a(), t),
        )
    }
}

impl F32Lerp for Transform {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Self {
            translation: Vec3::f32lerp(&self.translation, &rhs.translation, t),
            rotation: Quat::f32lerp(&self.rotation, &rhs.rotation, t),
            scale: Vec3::f32lerp(&self.scale, &rhs.scale, t),
        }
    }
}
