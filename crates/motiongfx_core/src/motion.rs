pub use motiongfx_core_macros::GetId;

use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};

pub mod pbr_motion;
pub mod transform_motion;

pub trait GetId {
    fn get_id(&self) -> Entity;
}

pub trait AddNewAssetCommandExtension<A: Asset> {
    /// Adds a new asset and attach the handle to this entity.
    fn add_new_asset(&mut self, asset: A) -> &mut Self;
}

impl<A: Asset> AddNewAssetCommandExtension<A> for EntityCommands<'_> {
    fn add_new_asset(&mut self, asset: A) -> &mut Self {
        self.add(AddNewAssetCommand(asset))
    }
}

pub struct AddNewAssetCommand<A: Asset>(A);

impl<A: Asset> EntityCommand for AddNewAssetCommand<A> {
    fn apply(self, id: Entity, world: &mut World) {
        let mut materials = world.get_resource_mut::<Assets<A>>().unwrap_or_else(|| {
            panic!(
                "Assets<{}> resource not initialized.",
                A::type_ident().unwrap()
            )
        });

        let material = materials.add(self.0);

        world.entity_mut(id).insert(material);
    }
}
