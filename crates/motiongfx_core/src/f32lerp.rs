use bevy::{
    math::{DQuat, DVec2, DVec3, DVec4},
    prelude::*,
};
use bevy_vello_renderer::vello::{kurbo, peniko};

use crate::cross_lerp::CrossLerp;

pub trait F32Lerp {
    /// Lerp between 2 values based on a [`f32`] `t` value.
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self;
}

impl F32Lerp for u8 {
    #[inline]
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        let other = *rhs as f32;
        let self_ = *self as f32;

        ((other - self_) * t + self_) as u8
    }
}

impl F32Lerp for f32 {
    #[inline]
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        f32::lerp(*self, *rhs, t)
    }
}

impl F32Lerp for f64 {
    #[inline]
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        f64::lerp(*self, *rhs, t as f64)
    }
}

impl F32Lerp for Vec2 {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Vec2::lerp(*self, *rhs, t)
    }
}

impl F32Lerp for Vec3 {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Vec3::lerp(*self, *rhs, t)
    }
}

impl F32Lerp for Vec4 {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Vec4::lerp(*self, *rhs, t)
    }
}

impl F32Lerp for Quat {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Quat::lerp(*self, *rhs, t)
    }
}

impl F32Lerp for DVec2 {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        DVec2::lerp(*self, *rhs, t as f64)
    }
}

impl F32Lerp for DVec3 {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        DVec3::lerp(*self, *rhs, t as f64)
    }
}

impl F32Lerp for DVec4 {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        DVec4::lerp(*self, *rhs, t as f64)
    }
}

impl F32Lerp for DQuat {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        DQuat::lerp(*self, *rhs, t as f64)
    }
}

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

impl F32Lerp for kurbo::Stroke {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Self {
            width: f64::lerp(self.width, rhs.width, t as f64),
            miter_limit: f64::lerp(self.miter_limit, rhs.miter_limit, t as f64),
            dash_offset: f64::lerp(self.dash_offset, rhs.dash_offset, t as f64),
            // dash_pattern: kurbo::Dashes::lerp(&self.dash_pattern, &other.dash_pattern, t),
            ..default()
        }
    }
}

impl F32Lerp for peniko::Brush {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        match self {
            Self::Solid(self_color) => match rhs {
                // =====================
                // Solid -> Solid
                // =====================
                Self::Solid(other_color) => {
                    return Self::Solid(peniko::Color::f32lerp(self_color, other_color, t));
                }

                // =====================
                // Solid -> Gradient
                // =====================
                Self::Gradient(other_grad) => {
                    return Self::Gradient(peniko::Gradient {
                        kind: other_grad.kind,
                        extend: other_grad.extend,
                        stops: peniko::Color::cross_lerp(self_color, &other_grad.stops, t),
                    });
                }

                // Image interpolation is not supported
                Self::Image(_) => {}
            },

            Self::Gradient(self_grad) => match rhs {
                // =====================
                // Gradient -> Solid
                // =====================
                Self::Solid(other_color) => {
                    return Self::Gradient(peniko::Gradient {
                        kind: self_grad.kind,
                        extend: self_grad.extend,
                        stops: peniko::ColorStops::cross_lerp(&self_grad.stops, other_color, t),
                    });
                }

                // =====================
                // Gradient -> Gradient
                // =====================
                Self::Gradient(other_grad) => 'grad: {
                    // Gradient kind and extend must be the same, otherwise, fallback
                    if self_grad.kind != other_grad.kind && self_grad.extend != other_grad.extend {
                        break 'grad;
                    }

                    return Self::Gradient(peniko::Gradient {
                        kind: self_grad.kind,
                        extend: self_grad.extend,
                        stops: peniko::ColorStops::f32lerp(&self_grad.stops, &other_grad.stops, t),
                    });
                }

                // Image interpolation is not supported
                Self::Image(_) => {}
            },

            // Image interpolation is not supported
            Self::Image(_) => {}
        }

        // Fallback to discrete interpolation
        if t < 0.5 {
            self.clone()
        } else {
            rhs.clone()
        }
    }
}

impl<Item, Array> F32Lerp for smallvec::SmallVec<Array>
where
    Item: F32Lerp + Default + PartialEq + Clone,
    Array: smallvec::Array<Item = Item>,
{
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        let mut self_iter = self.iter();
        let mut other_iter = rhs.iter();

        let mut last_self_item = Item::default();
        let mut last_other_item = Item::default();

        let mut interp_vec = smallvec::SmallVec::new();

        loop {
            let self_item = self_iter.next();
            let other_item = other_iter.next();

            if self_item.is_none() && other_item.is_none() {
                break;
            }

            last_self_item = self_item.unwrap_or(&last_self_item).clone();
            last_other_item = other_item.unwrap_or(&last_other_item).clone();

            interp_vec.push(Item::f32lerp(&last_self_item, &last_other_item, t));
        }

        interp_vec
    }
}

impl F32Lerp for peniko::ColorStop {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Self {
            offset: f32::lerp(self.offset, rhs.offset, t),
            color: peniko::Color::f32lerp(&self.color, &rhs.color, t),
        }
    }
}

impl F32Lerp for peniko::Color {
    fn f32lerp(&self, rhs: &Self, t: f32) -> Self {
        Self::rgba8(
            u8::f32lerp(&self.r, &rhs.r, t),
            u8::f32lerp(&self.g, &rhs.g, t),
            u8::f32lerp(&self.b, &rhs.b, t),
            u8::f32lerp(&self.a, &rhs.a, t),
        )
    }
}
