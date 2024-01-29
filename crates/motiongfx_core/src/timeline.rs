use bevy_ecs::prelude::*;
use bevy_time::Time;

use crate::{action::Action, lerp::*, sequence::Sequence, EmptyRes};

/// Plays the [`Timeline`] component attached to this entity.
#[derive(Component)]
pub struct TimelinePlayer;

/// Controls the target [`Sequence`] through `target_time` manipulation.
#[derive(Component)]
pub struct Timeline {
    pub(crate) sequence_id: Entity,
    pub(crate) curr_time: f32,
    pub time_scale: f32,
    pub target_time: f32,
}

impl Timeline {
    pub fn new(sequence_id: Entity) -> Self {
        Self {
            sequence_id,
            curr_time: 0.0,
            time_scale: 1.0,
            target_time: 0.0,
        }
    }

    /// Create an [`Action`] from a [`Timeline`].
    pub fn to_action(&self, begin: f32, end: f32) -> Option<Action<Timeline, f32, EmptyRes>> {
        Some(Action::new(
            self.sequence_id,
            begin,
            end,
            Self::timeline_interp,
        ))
    }

    fn timeline_interp(
        timeline: &mut Timeline,
        begin: &f32,
        end: &f32,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        timeline.target_time = f32::lerp(begin, end, t);
    }

    pub fn sequence_id(&self) -> Entity {
        self.sequence_id
    }
}

/// Safely update the timings in the `Timeline` after performing all the necessary actions.
pub(crate) fn timeline_update_system(
    q_sequences: Query<&Sequence>,
    mut q_timelines: Query<&mut Timeline, With<TimelinePlayer>>,
    time: Res<Time>,
) {
    for mut timeline in q_timelines.iter_mut() {
        let Ok(sequence) = q_sequences.get(timeline.sequence_id) else {
            return;
        };

        timeline.curr_time = timeline.target_time;

        timeline.target_time = f32::clamp(
            timeline.target_time + time.delta_seconds() * timeline.time_scale,
            0.0,
            sequence.duration,
        );
    }
}
