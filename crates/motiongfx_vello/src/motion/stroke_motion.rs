use bevy::prelude::*;
use motiongfx_core::{act, action::Action, motion::GetId};

use crate::prelude::Stroke;

use super::AddVelloSceneCommandExt;

#[derive(Clone)]
pub struct StrokeMotion<T> {
    pub id: Entity,
    pub vector: T,
    pub stroke: Stroke,
    pub transform: Transform,
}

pub trait BuildStrokeMotionExt<T> {
    /// Builds a vector into [`StrokeMotion`].
    fn build_stroke(&mut self, transform: Transform, vector: T, stroke: Stroke) -> StrokeMotion<T>;
}

impl<T: Component + Clone> BuildStrokeMotionExt<T> for Commands<'_, '_> {
    fn build_stroke(&mut self, transform: Transform, vector: T, stroke: Stroke) -> StrokeMotion<T> {
        let id = self
            .spawn((transform, vector.clone(), stroke.clone()))
            .add_vello_scene()
            .id();

        StrokeMotion {
            id,
            transform,
            vector,
            stroke,
        }
    }
}

pub trait StrokeMotionExt: GetId {
    fn get_stroke(&mut self) -> &mut Stroke;

    fn to_width(&mut self, width: f64) -> Action<f64, Stroke> {
        act!(
            (self.get_id(), Stroke),
            start = { self.get_stroke() }.style.width,
            end = width,
        )
    }
}
