use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};
use bevy_vello_renderer::vello_scene::{VelloScene, VelloSceneBundle};
use motiongfx_core::motion::AddNewAssetCommandExtension;

pub mod fill_motion;
pub mod stroke_motion;
pub mod vector_motion;

struct AddVelloSceneCommand;

impl EntityCommand for AddVelloSceneCommand {
    fn apply(self, id: Entity, world: &mut World) {
        let entity_ref = world.entity(id);
        let transform = entity_ref.get::<Transform>().copied().unwrap_or_default();
        let visibility = entity_ref.get::<Visibility>().copied().unwrap_or_default();

        world.entity_mut(id).insert(VelloSceneBundle {
            transform,
            visibility,
            ..default()
        });
    }
}

pub trait AddVelloSceneCommandExt {
    /// Adds [`VelloSceneBundle`] with a new default [`VelloScene`] asset and attach the handle to this entity.
    fn add_vello_scene(&mut self) -> &mut Self;
}

impl AddVelloSceneCommandExt for EntityCommands<'_> {
    fn add_vello_scene(&mut self) -> &mut Self {
        self.add(AddVelloSceneCommand)
            .add_new_asset(VelloScene::default())
    }
}
