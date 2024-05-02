use bevy::prelude::*;
use prelude::update_sequence;
use sequence::{sequence_controller, sequence_player};
use slide::slide_controller;

pub mod action;
pub mod color_palette;
pub mod cross_lerp;
pub mod ease;
pub mod f32lerp;
pub mod sequence;
pub mod slide;

pub mod prelude {
    pub use crate::{
        action::{act, act_interp, Action, ActionBuilderExtension},
        color_palette::{ColorKey, ColorPalette},
        cross_lerp::*,
        ease,
        f32lerp::*,
        sequence::{
            all, any, chain, delay, flow, update_sequence, MultiSequenceOrdering, Sequence,
            SequenceBundle, SequenceController, SequencePlayer, SequencePlayerBundle,
            SingleSequenceOrdering,
        },
        slide::{create_slide, SlideBundle, SlideController, SlideCurrState, SlideTargetState},
        MotionGfxPlugin, UpdateSequenceSet,
    };
}

pub struct MotionGfxPlugin;

impl Plugin for MotionGfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (sequence_player, slide_controller).before(UpdateSequenceSet),
        )
        .add_systems(
            Update,
            (
                update_sequence::<Transform, f32>,
                update_sequence::<Transform, Vec3>,
                update_sequence::<Transform, Quat>,
                update_sequence::<Sprite, Color>,
            )
                .in_set(UpdateSequenceSet),
        )
        .add_systems(Update, sequence_controller.after(UpdateSequenceSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateSequenceSet;
