use bevy::prelude::*;
use motiongfx_core::sequence::sequence_update_system;

// mod sprite_motion;
// mod standard_material_motion;
// mod transform_motion;

pub mod prelude {
    pub use crate::{
        // sprite_motion::SpriteMotion, standard_material_motion::StandardMaterialMotion,
        // transform_motion::TransformMotion,
        MotionGfxBevy,
    };
}

pub struct MotionGfxBevy;

impl Plugin for MotionGfxBevy {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            ((
                sequence_update_system::<f32, Transform>,
                sequence_update_system::<Vec3, Transform>,
                sequence_update_system::<Quat, Transform>,
                sequence_update_system::<Color, Sprite>,
            ),),
        );
    }
}
