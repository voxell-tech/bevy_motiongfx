use bevy_render::prelude::*;
use bevy_utils::prelude::*;
use bevy_vello_renderer::vello::{kurbo, peniko};

use crate::CrossLerp;

pub trait Lerp<Time> {
    fn lerp(&self, other: &Self, t: Time) -> Self;
}

impl Lerp<f32> for kurbo::Stroke {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            width: f64::lerp(&self.width, &other.width, t),
            miter_limit: f64::lerp(&self.miter_limit, &other.miter_limit, t),
            dash_offset: f64::lerp(&self.dash_offset, &other.dash_offset, t),
            ..default()
        }
    }
}

impl Lerp<f32> for peniko::Brush {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        match self {
            peniko::Brush::Solid(self_color) => match other {
                // Solid -> Solid
                peniko::Brush::Solid(other_color) => {
                    return peniko::Brush::Solid(peniko::Color::lerp(self_color, other_color, t));
                }
                // Solid -> Gradient
                peniko::Brush::Gradient(other_grad) => {
                    return peniko::Brush::Gradient(peniko::Gradient {
                        kind: other_grad.kind,
                        extend: other_grad.extend,
                        stops: peniko::Color::cross_lerp(self_color, &other_grad.stops, t),
                    });
                }
                // Image interpolation is not supported
                peniko::Brush::Image(_) => {}
            },
            peniko::Brush::Gradient(self_grad) => match other {
                // Gradient -> Solid
                peniko::Brush::Solid(other_color) => {
                    return peniko::Brush::Gradient(peniko::Gradient {
                        kind: self_grad.kind,
                        extend: self_grad.extend,
                        stops: peniko::ColorStops::cross_lerp(&self_grad.stops, other_color, t),
                    });
                }
                // Gradient -> Gradient
                peniko::Brush::Gradient(other_grad) => 'grad: {
                    // Gradient kind and extend must be the same, otherwise, fallback
                    if self_grad.kind != other_grad.kind && self_grad.extend != other_grad.extend {
                        break 'grad;
                    }

                    return peniko::Brush::Gradient(peniko::Gradient {
                        kind: self_grad.kind,
                        extend: self_grad.extend,
                        stops: peniko::ColorStops::lerp(&self_grad.stops, &other_grad.stops, t),
                    });
                }
                // Image interpolation is not supported
                peniko::Brush::Image(_) => {}
            },
            // Image interpolation is not supported
            peniko::Brush::Image(_) => {}
        }

        // Fallback to discrete interpolation
        if t < 0.5 {
            return self.clone();
        } else {
            return other.clone();
        }
    }
}

impl<Item, Array> Lerp<f32> for smallvec::SmallVec<Array>
where
    Item: Lerp<f32> + Default + Eq + Clone,
    Array: smallvec::Array<Item = Item>,
{
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let mut self_iter: std::slice::Iter<Item> = self.iter();
        let mut other_iter: std::slice::Iter<Item> = other.iter();

        let mut last_self_stop: Item = Item::default();
        let mut last_other_stop: Item = Item::default();

        let mut interp_stops: smallvec::SmallVec<Array> = smallvec::SmallVec::new();

        loop {
            let self_stop: Option<&Item> = self_iter.next();
            let other_stop: Option<&Item> = other_iter.next();

            if self_stop == None && other_stop == None {
                break;
            }

            last_self_stop = self_stop.unwrap_or(&last_self_stop).clone();
            last_other_stop = other_stop.unwrap_or(&last_other_stop).clone();

            interp_stops.push(Item::lerp(&last_self_stop, &last_other_stop, t));
        }

        interp_stops
    }
}

impl Lerp<f32> for peniko::ColorStop {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            offset: f32::lerp(&self.offset, &other.offset, t),
            color: peniko::Color::lerp(&self.color, &other.color, t),
        }
    }
}

impl Lerp<f32> for peniko::Color {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::rgba8(
            u8::lerp(&self.r, &other.r, t),
            u8::lerp(&self.g, &other.g, t),
            u8::lerp(&self.b, &other.b, t),
            u8::lerp(&self.a, &other.a, t),
        )
    }
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

impl Lerp<f32> for u8 {
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let other: f32 = *other as f32;
        let self_: f32 = *self as f32;

        ((other - self_) * t + self_) as u8
    }
}
