use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use prelude::*;

pub mod action;
pub mod ease;
pub mod sequence;
pub mod style;
pub mod timeline;

pub mod prelude {
    pub use super::action::{Action, ActionBuilder, ActionMeta, ActionMetaGroup, InterpFn};
    pub use super::ease;
    pub use super::sequence::{all, any, chain, delay, flow, sequence_player_system, Sequence};
    pub use super::style;
    pub use super::timeline::Timeline;
    pub use super::EmptyComp;
    pub use super::EmptyRes;
    pub use super::MotionGfx;
}

pub struct MotionGfx;

impl Plugin for MotionGfx {
    fn build(&self, app: &mut App) {
        app.insert_resource(Timeline::new())
            .insert_resource(Sequence::new())
            .insert_resource(EmptyRes)
            .add_systems(PreUpdate, timeline::timeline_update_system);
    }
}

#[derive(Resource)]
pub struct EmptyRes;

#[derive(Component)]
pub struct EmptyComp;
