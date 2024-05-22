use bevy_vello::prelude::*;

use crate::f32lerp::F32Lerp;

pub trait F32CrossLerp<T, U> {
    fn cross_lerp(&self, rhs: &T, t: f32) -> U;
}

impl F32CrossLerp<peniko::ColorStops, peniko::ColorStops> for peniko::Color {
    fn cross_lerp(&self, rhs: &peniko::ColorStops, t: f32) -> peniko::ColorStops {
        let self_stops = peniko::ColorStops::from_vec(vec![peniko::ColorStop {
            offset: 0.0,
            color: *self,
        }]);

        peniko::ColorStops::f32lerp(&self_stops, rhs, t)
    }
}

impl F32CrossLerp<peniko::Color, peniko::ColorStops> for peniko::ColorStops {
    fn cross_lerp(&self, rhs: &peniko::Color, t: f32) -> peniko::ColorStops {
        let other_stops = peniko::ColorStops::from_vec(vec![peniko::ColorStop {
            offset: 0.0,
            color: *rhs,
        }]);

        peniko::ColorStops::f32lerp(self, &other_stops, t)
    }
}
