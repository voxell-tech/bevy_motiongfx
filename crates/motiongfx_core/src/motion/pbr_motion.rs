use bevy::{ecs::system::EntityCommands, prelude::*};

use super::AddNewAssetCommandExtension;

use crate::prelude::{GetId, StandardMaterialMotion, TransformMotion};

#[derive(StandardMaterialMotion, TransformMotion, GetId, Clone)]
pub struct PbrMotion {
    #[id]
    pub id: Entity,
    #[transform]
    pub transform: Transform,
    #[standard_material]
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

impl BuildPbrMotionExt for EntityCommands<'_> {
    fn build_pbr(
        &mut self,
        transform: Transform,
        mesh: Handle<Mesh>,
        material: StandardMaterial,
    ) -> PbrMotion {
        let id = self
            .insert(PbrBundle {
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

impl BuildPbrMotionExt for Commands<'_, '_> {
    fn build_pbr(
        &mut self,
        transform: Transform,
        mesh: Handle<Mesh>,
        material: StandardMaterial,
    ) -> PbrMotion {
        self.spawn_empty().build_pbr(transform, mesh, material)
    }
}
