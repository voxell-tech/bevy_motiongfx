use bevy::prelude::*;

use super::{transform_motion::TransformMotion, AddNewAssetCommandExtension, GetId};

#[derive(TransformMotion, GetId, Clone)]
pub struct PbrMotion {
    #[id]
    pub id: Entity,
    #[transform]
    pub transform: Transform,
    pub material: StandardMaterial,
}

pub trait BuildPbrMotionExt {
    /// Builds a [`PbrMotion`].
    fn build_pbr(
        &mut self,
        transform: Transform,
        mesh: Handle<Mesh>,
        material: StandardMaterial,
    ) -> PbrMotion;
}

impl BuildPbrMotionExt for Commands<'_, '_> {
    fn build_pbr(
        &mut self,
        transform: Transform,
        mesh: Handle<Mesh>,
        material: StandardMaterial,
    ) -> PbrMotion {
        let id = self
            .spawn(PbrBundle {
                transform,
                mesh,
                ..default()
            })
            .add_new_asset(material.clone())
            .id();

        PbrMotion {
            id,
            transform,
            material,
        }
    }
}
