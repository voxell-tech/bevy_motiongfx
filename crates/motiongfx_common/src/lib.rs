use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};
use motiongfx_core::{prelude::*, UpdateSequenceSet};

pub mod motion;

pub mod prelude {
    pub use crate::{
        motion::{
            standard_material_motion::StandardMaterialMotion, transform_motion::TransformMotion,
        },
        AddNewAssetCommandExt, MotionGfxCommonPlugin,
    };
}

pub struct MotionGfxCommonPlugin;

impl Plugin for MotionGfxCommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_component::<Transform, Vec3>,
                animate_component::<Transform, Quat>,
                animate_component::<Transform, f32>,
                animate_component::<Sprite, Color>,
                animate_component::<Sprite, f32>,
                animate_asset::<StandardMaterial, Color>,
                animate_asset::<StandardMaterial, LinearRgba>,
                animate_asset::<StandardMaterial, f32>,
                animate_asset::<ColorMaterial, Color>,
                animate_asset::<ColorMaterial, f32>,
            )
                .in_set(UpdateSequenceSet),
        );
    }
}

pub trait AddNewAssetCommandExt<A: Asset> {
    /// Adds a new asset and attach the handle to this entity.
    fn add_new_asset(&mut self, asset: A) -> &mut Self;
}

impl<A: Asset> AddNewAssetCommandExt<A> for EntityCommands<'_> {
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
