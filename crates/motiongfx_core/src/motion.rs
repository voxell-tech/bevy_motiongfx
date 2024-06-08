use motiongfx_core_macros::tuple_combinations;

use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};

pub mod standard_material_motion;
pub mod transform_motion;

pub trait GetId {
    fn id(&self) -> Entity;
}

impl<T> GetId for (Entity, T) {
    fn id(&self) -> Entity {
        self.0
    }
}

pub trait GetMutValue<T, const N: usize> {
    fn get_mut_value(&mut self) -> &mut T;
}

impl<T> GetMutValue<T, 0> for T {
    fn get_mut_value(&mut self) -> &mut T {
        self
    }
}

pub trait GetMut<const N: usize, U> {
    fn get_mut<T>(&mut self) -> &mut T
    where
        U: GetMutValue<T, N>;
}

impl<const N: usize, U> GetMut<N, U> for (Entity, U) {
    fn get_mut<T>(&mut self) -> &mut T
    where
        U: GetMutValue<T, N>,
    {
        self.1.get_mut_value()
    }
}

macro_rules! impl_get_mut_value {
    ([$($generic:ident),+], $main_generic:ident, $number:tt) => {
        impl <$($generic),+> GetMutValue<$main_generic, $number> for ($($generic),+) {
            fn get_mut_value(&mut self) -> &mut $main_generic {
                &mut self.$number
            }
        }
    };
}

tuple_combinations!(impl_get_mut_value, 20);

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
