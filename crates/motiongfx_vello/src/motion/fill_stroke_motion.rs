use bevy::prelude::*;

use crate::prelude::{Fill, Stroke};

use super::AddVelloSceneCommandExt;

#[derive(Clone)]
pub struct FillStrokeMotion<T> {
    pub id: Entity,
    pub transform: Transform,
    pub vector: T,
    pub fill: Fill,
    pub stroke: Stroke,
}

pub trait BuildFillStrokeMotionExt<T> {
    /// Builds a vector into [`FillStrokeMotion`].
    fn build_fill_stroke(
        &mut self,
        transform: Transform,
        vector: T,
        fill: Fill,
        stroke: Stroke,
    ) -> FillStrokeMotion<T>;
}

impl<T: Component + Clone> BuildFillStrokeMotionExt<T> for Commands<'_, '_> {
    fn build_fill_stroke(
        &mut self,
        transform: Transform,
        vector: T,
        fill: Fill,
        stroke: Stroke,
    ) -> FillStrokeMotion<T> {
        let id = self
            .spawn((transform, vector.clone(), fill.clone(), stroke.clone()))
            .add_vello_scene()
            .id();

        FillStrokeMotion {
            id,
            transform,
            vector,
            fill,
            stroke,
        }
    }
}
