pub use motiongfx_vello_macros::FillMotion;

use bevy::prelude::*;
use bevy_vello_graphics::prelude::*;
use motiongfx_core::{
    act,
    motion::{GetId, GetMutValue},
    prelude::Action,
};

pub trait FillMotion<const N: usize> {
    fn fill(&mut self) -> FillMotionBuilder;
}

impl<const N: usize, T: GetMutValue<Fill, N>> FillMotion<N> for (Entity, T) {
    fn fill(&mut self) -> FillMotionBuilder {
        FillMotionBuilder::new(self.id(), self.1.get_mut_value())
    }
}

pub struct FillMotionBuilder<'a> {
    id: Entity,
    pub fill: &'a mut Fill,
}

impl<'a> FillMotionBuilder<'a> {
    pub fn new(id: Entity, fill: &'a mut Fill) -> Self {
        Self { id, fill }
    }

    pub fn to_color(&mut self, color: Color) -> Action<peniko::Brush, Fill> {
        act!(
            (self.id, Fill),
            start = { self.fill }.brush.value,
            end = peniko::Brush::Solid(peniko::Color::rgba(
                color.r() as f64,
                color.g() as f64,
                color.b() as f64,
                color.a() as f64
            )),
        )
    }
}
