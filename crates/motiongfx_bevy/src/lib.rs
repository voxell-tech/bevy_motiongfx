use bevy::prelude::*;
use motiongfx_core::{prelude::*, sequence::sequence_update_system};

mod sprite_motion;
mod standard_material_motion;
mod transform_motion;

pub mod prelude {
    pub use crate::{
        sprite_motion::SpriteMotion, standard_material_motion::StandardMaterialMotion,
        transform_motion::TransformMotion, MotionGfxBevy,
    };
}

pub struct MotionGfxBevy;

impl Plugin for MotionGfxBevy {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            ((
                sequence_update_system::<Transform, Vec3, EmptyRes>,
                sequence_update_system::<Transform, Quat, EmptyRes>,
                sequence_update_system::<Handle<StandardMaterial>, Color, Assets<StandardMaterial>>,
                sequence_update_system::<Sprite, Color, EmptyRes>,
            ),),
        );
    }
}
