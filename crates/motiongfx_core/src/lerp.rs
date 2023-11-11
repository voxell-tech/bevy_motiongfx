use bevy_render::prelude::*;

pub trait Lerp<T> {
    fn lerp(&self, other: &Self, t: T) -> Self;
}

impl Lerp<f32> for Color {
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

impl Lerp<f32> for f32 {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        (other - self) * t + self
    }
}

impl Lerp<f64> for f64 {
    #[inline]
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (other - self) * t + self
    }
}

impl Lerp<f32> for f64 {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        (other - self) * (t as f64) + self
    }
}
