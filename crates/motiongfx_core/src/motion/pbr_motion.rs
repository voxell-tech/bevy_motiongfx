use bevy::prelude::*;

use super::AddNewAssetCommandExtension;

use crate::{
    act,
    prelude::{Action, GetId, TransformMotion},
};

#[derive(TransformMotion, GetId, Clone)]
pub struct PbrMotion {
    #[id]
    pub id: Entity,
    #[transform]
    pub transform: Transform,
    pub material: StandardMaterial,
}

impl PbrMotion {
    pub fn to_emissive(&mut self, color: Color) -> Action<Color, StandardMaterial> {
        act!(
            (self.get_id(), StandardMaterial),
            start = { self.material }.emissive,
            end = color,
        )
    }

    pub fn to_base_color(&mut self, color: Color) -> Action<Color, StandardMaterial> {
        act!(
            (self.get_id(), StandardMaterial),
            start = { self.material }.base_color,
            end = color,
        )
    }
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
