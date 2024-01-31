use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

use crate::sequence::{Sequence, SequenceTime};

#[derive(Component)]
pub struct Slide {
    sequence_ids: Vec<Entity>,
}

impl Slide {
    pub fn new(sequence_ids: &[Entity]) -> Slide {
        Slide {
            sequence_ids: sequence_ids.to_vec(),
        }
    }
}

#[derive(Component)]
pub struct SlidePlayer {
    pub time_scale: f32,
    pub slide_index: usize,
}

pub(crate) fn slide_player_system(
    q_slides: Query<(&Slide, &SlidePlayer)>,
    mut q_sequences: Query<(&Sequence, &mut SequenceTime)>,
    time: Res<Time>,
) {
    for (slide, slide_player) in q_slides.iter() {}
}
