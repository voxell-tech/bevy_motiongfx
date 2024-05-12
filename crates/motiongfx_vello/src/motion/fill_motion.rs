use bevy::prelude::*;
use bevy_vello_renderer::vello::peniko;
use motiongfx_core::{act, motion::GetId, prelude::Action};

use crate::prelude::Fill;

use super::AddVelloSceneCommandExt;

#[derive(Clone)]
pub struct FillMotion<T> {
    pub id: Entity,
    pub transform: Transform,
    pub vector: T,
    pub fill: Fill,
}

pub trait BuildFillMotionExt<T> {
    /// Builds a vector into [`FillMotion`].
    fn build_fill(&mut self, transform: Transform, vector: T, fill: Fill) -> FillMotion<T>;
}

impl<T: Component + Clone> BuildFillMotionExt<T> for Commands<'_, '_> {
    fn build_fill(&mut self, transform: Transform, vector: T, fill: Fill) -> FillMotion<T> {
        let id = self
            .spawn((transform, vector.clone(), fill.clone()))
            .add_vello_scene()
            .id();

        FillMotion {
            id,
            transform,
            vector,
            fill,
        }
    }
}

pub trait FillMotionExt: GetId {
    fn get_fill(&mut self) -> &mut Fill;

    fn to_color(&mut self, color: Color) -> Action<peniko::Brush, Fill> {
        act!(
            (self.get_id(), Fill),
            start = { self.get_fill() }.brush.value,
            end = peniko::Brush::Solid(peniko::Color::rgba(
                color.r() as f64,
                color.g() as f64,
                color.b() as f64,
                color.a() as f64
            )),
        )
    }
}
