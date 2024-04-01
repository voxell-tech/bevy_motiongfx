pub use motiongfx_vello_macros::VelloVector;

use bevy::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo},
};

use crate::{fill_style::FillStyle, stroke_style::StrokeStyle};

pub mod bezpath;
pub mod circle;
pub mod line;
pub mod rect;

pub trait _VelloVector {
    fn build_scene(&self) -> vello::Scene;
}

pub trait VelloVector {
    fn shape(&self) -> impl kurbo::Shape;

    #[inline]
    fn build_fill(&self, fill: &FillStyle, scene: &mut vello::Scene) {
        fill.build(scene, &self.shape());
    }

    #[inline]
    fn build_stroke(&self, stroke: &StrokeStyle, scene: &mut vello::Scene) {
        stroke.build(scene, &self.shape());
    }
}

pub(crate) fn vector_builder_system<Vector: VelloVector + Component>(
    mut q_fill_only_vectors: Query<
        (&Vector, &FillStyle, &Handle<VelloScene>),
        (
            Without<StrokeStyle>,
            Or<(Changed<Vector>, Changed<FillStyle>)>,
        ),
    >,
    mut q_stroke_only_vectors: Query<
        (&Vector, &StrokeStyle, &Handle<VelloScene>),
        (
            Without<FillStyle>,
            Or<(Changed<Vector>, Changed<StrokeStyle>)>,
        ),
    >,
    mut q_fill_and_stroke_vectors: Query<
        (&Vector, &FillStyle, &StrokeStyle, &Handle<VelloScene>),
        Or<(Changed<Vector>, Changed<FillStyle>, Changed<StrokeStyle>)>,
    >,
    mut scenes: ResMut<Assets<VelloScene>>,
) {
    for (vector, fill, scene_handle) in q_fill_only_vectors.iter_mut() {
        if let Some(vello_scene) = scenes.get_mut(scene_handle.id()) {
            let mut scene = vello::Scene::new();

            // Build the vector to the VelloScene
            vector.build_fill(&fill, &mut scene);

            // Replace with new scene
            vello_scene.scene = scene.into();
        }
    }

    for (vector, stroke, scene_handle) in q_stroke_only_vectors.iter_mut() {
        if let Some(vello_scene) = scenes.get_mut(scene_handle.id()) {
            let mut scene = vello::Scene::new();

            // Build the vector to the VelloScene
            vector.build_stroke(&stroke, &mut scene);

            // Replace with new scene
            vello_scene.scene = scene.into();
        }
    }

    for (vector, fill, stroke, scene_handle) in q_fill_and_stroke_vectors.iter_mut() {
        if let Some(vello_scene) = scenes.get_mut(scene_handle.id()) {
            let mut scene = vello::Scene::new();

            // Build the vector to the VelloScene
            vector.build_fill(&fill, &mut scene);
            vector.build_stroke(&stroke, &mut scene);

            // Replace with new scene
            vello_scene.scene = scene.into();
        }
    }
}

pub(crate) fn _vector_builder_system<Vector: _VelloVector + Component>(
    q_vectors: Query<(&Vector, &Handle<VelloScene>), Changed<Vector>>,
    mut scenes: ResMut<Assets<VelloScene>>,
) {
    for (vector, scene_handle) in q_vectors.iter() {
        if let Some(vello_scene) = scenes.get_mut(scene_handle.id()) {
            let scene = vector.build_scene();
            vello_scene.scene = scene.into();
        }
    }
}
