use bevy::prelude::*;

pub mod standard_material;
pub mod transform;

pub mod prelude {
    pub use super::standard_material::BaseColor;
    pub use super::transform::{Rotation, Scale, Translation};
    pub use super::EmptyRes;
}

#[derive(Resource)]
pub struct EmptyRes;

#[derive(Component)]
pub struct EmptyComp;
