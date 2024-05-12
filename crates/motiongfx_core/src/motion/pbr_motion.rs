use bevy::prelude::*;

use super::AddNewAssetCommandExtension;

pub struct PbrMotion {
    pub id: Entity,
    pub transform: Transform,
    pub material: StandardMaterial,
}

impl PbrMotion {
    fn new(
        commands: &mut Commands,
        transform: Transform,
        mesh: Handle<Mesh>,
        material: StandardMaterial,
    ) -> Self {
        let id = commands
            .spawn(PbrBundle {
                transform,
                mesh,
                ..default()
            })
            .add_new_asset(material.clone())
            .id();

        Self {
            id,
            transform,
            material,
        }
    }
}
