use bevy_ecs::prelude::*;
use bevy_time::Time;
use bevy_utils::prelude::*;

use crate::sequence::Sequence;

#[derive(Component)]
pub struct Timeline {
    pub(crate) sequence_id: Option<Entity>,
    pub(crate) curr_time: f32,
    pub is_playing: bool,
    pub time_scale: f32,
    pub target_time: f32,
}

impl Timeline {
    pub fn new(sequence_id: Entity) -> Self {
        Self {
            sequence_id: Some(sequence_id),
            ..default()
        }
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            sequence_id: None,
            curr_time: 0.0,
            is_playing: false,
            time_scale: 1.0,
            target_time: 0.0,
        }
    }
}

/// Safely update the timings in the `Timeline` after performing all the necessary actions.
pub(crate) fn timeline_update_system(
    q_sequences: Query<&Sequence>,
    mut q_timelines: Query<&mut Timeline>,
    time: Res<Time>,
) {
    for mut timeline in q_timelines.iter_mut() {
        let Some(target_sequence) = timeline.sequence_id else {
            return;
        };

        let Ok(sequence) = q_sequences.get(target_sequence) else {
            return;
        };

        timeline.curr_time = timeline.target_time;

        if timeline.is_playing {
            timeline.target_time += time.delta_seconds() * timeline.time_scale;
        }

        timeline.target_time = f32::clamp(timeline.target_time, 0.0, sequence.duration);
    }
}
