use bevy_ecs::prelude::*;
use bevy_time::Time;

use crate::sequence::{Sequence, SequenceTime};

/// Bundle to encapsulate [`Sequence`], [`SequenceTime`], and [`SequencePlayer`].
#[derive(Bundle, Default)]
pub struct SequencePlayerBundle {
    pub sequence: Sequence,
    pub sequence_time: SequenceTime,
    pub sequence_player: SequencePlayer,
}

/// Manipulates the `target_time` variable of the [`SequenceTime`] component attached to this entity with a `time_scale`.
#[derive(Component, Default)]
pub struct SequencePlayer {
    pub time_scale: f32,
}

/// Update [`SequenceTime`] based on `time_scale` of [`SequencePlayer`].
pub(crate) fn sequence_player_system(
    mut q_sequences: Query<(&Sequence, &mut SequenceTime, &SequencePlayer)>,
    time: Res<Time>,
) {
    for (sequence, mut sequence_time, sequence_player) in q_sequences.iter_mut() {
        sequence_time.target_time = f32::clamp(
            sequence_time.target_time + time.delta_seconds() * sequence_player.time_scale,
            0.0,
            sequence.duration(),
        );
    }
}
