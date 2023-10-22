use bevy::prelude::*;
use prelude::*;

pub mod action;
pub mod action_group;
pub mod animation_states;
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
    pub use super::MotionGfx;

    pub use super::animation_states::prelude::*;
}

pub struct MotionGfx;

impl Plugin for MotionGfx {
    fn build(&self, app: &mut App) {
        app.insert_resource(Timeline::new())
            .insert_resource(Sequence::new())
            .insert_resource(EmptyRes)
            .add_systems(PreUpdate, timeline::timeline_update_system)
            .add_systems(
                PostUpdate,
                (
                    sequence::sequence_player_system::<Transform, Vec3, EmptyRes>,
                    sequence::sequence_player_system::<Transform, Quat, EmptyRes>,
                    sequence::sequence_player_system::<
                        Handle<StandardMaterial>,
                        Vec4,
                        Assets<StandardMaterial>,
                    >,
                ),
            );
    }
}
