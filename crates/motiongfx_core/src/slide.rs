use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

use crate::sequence::{Sequence, SequenceBundle, SequenceController};

#[derive(Component, Default)]
pub struct Slide {
    sequence_ids: Vec<Entity>,
}

#[derive(Component, Default)]
pub struct SlideController {
    /// Reflects the actual `target_time` in [`SequenceTime`].
    sequence_time: f32,
    /// The time we aim the `sequence_time` to go towards.
    target_time: f32,
    curr_index: usize,
    target_index: usize,
}

impl SlideController {
    pub fn next(&mut self) {
        self.target_index = self.target_index.saturating_add(1);
        self.target_time = 0.0;
    }

    pub fn prev(&mut self) {
        self.target_index = self.target_index.saturating_sub(1);
        self.target_time = 0.0;
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.target_index = index;
        self.target_time = 0.0;
        self
    }

    pub fn with_time(mut self, target_time: f32) -> Self {
        self.target_time = target_time;
        self
    }
}

#[derive(Component)]
pub struct SlidePlayer {
    utime_scale: f32,
}

impl SlidePlayer {
    #[inline]
    pub fn set_utime_scale(&mut self, time_scale: f32) {
        self.utime_scale = f32::abs(time_scale);
    }
}

impl Default for SlidePlayer {
    fn default() -> Self {
        Self { utime_scale: 1.0 }
    }
}

#[derive(Bundle, Default)]
pub struct SlideBundle {
    pub slide: Slide,
    pub slide_controller: SlideController,
}

#[derive(Bundle, Default)]
pub struct SlidePlayerBundle {
    pub slide: Slide,
    pub slide_controller: SlideController,
    pub slide_player: SlidePlayer,
}

pub trait SlideBuilder {
    fn create_slide(&mut self, seqeunces: Vec<Sequence>) -> Slide;
}

impl SlideBuilder for Commands<'_, '_> {
    fn create_slide(&mut self, sequences: Vec<Sequence>) -> Slide {
        let mut sequence_ids: Vec<Entity> = Vec::with_capacity(sequences.len());

        for sequence in sequences {
            sequence_ids.push(self.spawn(SequenceBundle::from_sequence(sequence)).id());
        }

        Slide { sequence_ids }
    }
}

pub(crate) fn slide_update_system(
    mut q_slides: Query<(&Slide, &mut SlideController)>,
    mut q_sequences: Query<(&Sequence, &mut SequenceController)>,
) {
    for (slide, mut slide_controller) in q_slides.iter_mut() {
        let Ok((sequence, mut sequence_time)) =
            q_sequences.get_mut(slide.sequence_ids[slide_controller.curr_index])
        else {
            return;
        };

        slide_controller.sequence_time =
            f32::clamp(slide_controller.sequence_time, 0.0, sequence.duration());
        sequence_time.target_time = slide_controller.sequence_time;
    }
}

pub(crate) fn slide_controller_update_system(
    mut q_slides: Query<(&Slide, &mut SlideController, &SlidePlayer)>,
    q_sequences: Query<&Sequence>,
    time: Res<Time>,
) {
    for (slide, mut slide_controller, slide_player) in q_slides.iter_mut() {
        if slide_player.utime_scale <= f32::EPSILON || slide.sequence_ids.is_empty() {
            continue;
        }

        let Ok(sequence) = q_sequences.get(slide.sequence_ids[slide_controller.curr_index]) else {
            continue;
        };

        // Prevent target index from exceeding the available sequence count
        if slide_controller.target_index >= slide.sequence_ids.len() {
            let Ok(last_sequence) = q_sequences.get(*slide.sequence_ids.last().unwrap()) else {
                continue;
            };

            // Set target index to last index and target time at sequence duration
            slide_controller.target_index = slide.sequence_ids.len() - 1;
            slide_controller.target_time = last_sequence.duration();
        }

        // Calculate time flow direction
        let direction: isize = {
            // priority 1: index difference
            if slide_controller.target_index != slide_controller.curr_index {
                isize::signum(
                    slide_controller.target_index as isize - slide_controller.curr_index as isize,
                )
            // priority 2: time difference
            } else {
                f32::signum(slide_controller.target_time - slide_controller.sequence_time) as isize
            }
        };

        if slide_controller.target_index != slide_controller.curr_index {
            // Move target index if we reached either ending of the current sequence
            if slide_controller.sequence_time <= 0.0
                || slide_controller.sequence_time >= sequence.duration()
            {
                slide_controller.curr_index =
                    (slide_controller.curr_index as isize + direction) as usize;

                if direction > 0 {
                    // Start from 0.0 if we are moving to the next sequence
                    slide_controller.sequence_time = 0.0;
                } else if direction < 0 {
                    println!("prev");
                    // Start from duration if we are moving to the previous sequence
                    if let Ok(prev_sequence) =
                        q_sequences.get(slide.sequence_ids[slide_controller.curr_index])
                    {
                        println!("prev in");
                        slide_controller.sequence_time = prev_sequence.duration();
                    }
                }
            }
        }

        // Move sequence time based on time scale and direction
        slide_controller.sequence_time = f32::clamp(
            slide_controller.sequence_time
                + time.delta_seconds() * slide_player.utime_scale * direction as f32,
            0.0,
            sequence.duration(),
        );
    }
}
