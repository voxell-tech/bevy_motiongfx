use bevy_render::prelude::*;

pub trait Lerp {
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

impl Lerp for Color {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::rgba(
            f32::lerp(&self.r(), &other.r(), t),
            f32::lerp(&self.g(), &other.g(), t),
            f32::lerp(&self.b(), &other.b(), t),
            f32::lerp(&self.a(), &other.a(), t),
        )
    }
}

impl Lerp for f32 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        (other - self) * t + self
    }
}
