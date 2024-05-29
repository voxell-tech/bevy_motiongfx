use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_vello_graphics::prelude::*;
use motiongfx_core::motion::{transform_motion::TransformMotion, GetId};

use super::{fill_motion::FillMotion, stroke_motion::StrokeMotion};

#[derive(FillMotion, TransformMotion, GetId, Clone)]
pub struct FVectorMotion<T> {
    #[id]
    pub id: Entity,
    #[transform]
    pub transform: Transform,
    pub vector: T,
    #[fill]
    pub fill: Fill,
}

#[derive(StrokeMotion, TransformMotion, GetId, Clone)]
pub struct SVectorMotion<T> {
    #[id]
    pub id: Entity,
    #[transform]
    pub transform: Transform,
    pub vector: T,
    #[stroke]
    pub stroke: Stroke,
}

#[derive(FillMotion, StrokeMotion, TransformMotion, GetId, Clone)]
pub struct FSVectorMotion<T> {
    #[id]
    pub id: Entity,
    #[transform]
    pub transform: Transform,
    pub vector: T,
    #[fill]
    pub fill: Fill,
    #[stroke]
    pub stroke: Stroke,
}

pub trait BuildVectorMotionExt<T> {
    /// Builds a [`FVectorMotion`].
    fn build_fvector(&mut self, transform: Transform, vector: T, fill: Fill) -> FVectorMotion<T>;

    /// Builds a [`SVectorMotion`].
    fn build_svector(
        &mut self,
        transform: Transform,
        vector: T,
        stroke: Stroke,
    ) -> SVectorMotion<T>;

    /// Builds a [`FSVectorMotion`].
    fn build_fsvector(
        &mut self,
        transform: Transform,
        vector: T,
        fill: Fill,
        stroke: Stroke,
    ) -> FSVectorMotion<T>;
}

impl<T: Component + Clone> BuildVectorMotionExt<T> for EntityCommands<'_> {
    fn build_fvector(&mut self, transform: Transform, vector: T, fill: Fill) -> FVectorMotion<T> {
        let id = self
            .insert(VelloSceneBundle::default())
            .insert((transform, vector.clone(), fill.clone()))
            .id();

        FVectorMotion {
            id,
            transform,
            vector,
            fill,
        }
    }

    fn build_svector(
        &mut self,
        transform: Transform,
        vector: T,
        stroke: Stroke,
    ) -> SVectorMotion<T> {
        let id = self
            .insert(VelloSceneBundle::default())
            .insert((transform, vector.clone(), stroke.clone()))
            .id();

        SVectorMotion {
            id,
            transform,
            vector,
            stroke,
        }
    }

    fn build_fsvector(
        &mut self,
        transform: Transform,
        vector: T,
        fill: Fill,
        stroke: Stroke,
    ) -> FSVectorMotion<T> {
        let id = self
            .insert(VelloSceneBundle::default())
            .insert((transform, vector.clone(), fill.clone(), stroke.clone()))
            .id();

        FSVectorMotion {
            id,
            transform,
            vector,
            fill,
            stroke,
        }
    }
}

impl<T: Component + Clone> BuildVectorMotionExt<T> for Commands<'_, '_> {
    fn build_fvector(&mut self, transform: Transform, vector: T, fill: Fill) -> FVectorMotion<T> {
        self.spawn_empty().build_fvector(transform, vector, fill)
    }

    fn build_svector(
        &mut self,
        transform: Transform,
        vector: T,
        stroke: Stroke,
    ) -> SVectorMotion<T> {
        self.spawn_empty().build_svector(transform, vector, stroke)
    }

    fn build_fsvector(
        &mut self,
        transform: Transform,
        vector: T,
        fill: Fill,
        stroke: Stroke,
    ) -> FSVectorMotion<T> {
        self.spawn_empty()
            .build_fsvector(transform, vector, fill, stroke)
    }
}
