use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub mod action;
pub mod color_palette;
pub mod cross_lerp;
pub mod ease;
pub mod lerp;
pub mod sequence;
pub mod slide;

pub mod prelude {
    pub use crate::{
        action::{Action, ActionBuilder},
        color_palette::{ColorKey, ColorPalette},
        cross_lerp::*,
        ease,
        lerp::*,
        sequence::{
            all, any, chain, delay, flow, Sequence, SequenceBundle, SequencePlayer,
            SequencePlayerBundle, SequenceTime,
        },
        slide::Slide,
        EmptyComp, EmptyRes, MotionGfx,
    };
}

pub struct MotionGfx;

impl Plugin for MotionGfx {
    fn build(&self, app: &mut App) {
        app.insert_resource(EmptyRes)
            .add_systems(PreUpdate, sequence::sequence_time_update_system)
            .add_systems(Update, sequence::sequence_player_system);
    }
}

#[derive(Resource)]
pub struct EmptyRes;

#[derive(Component)]
pub struct EmptyComp;
