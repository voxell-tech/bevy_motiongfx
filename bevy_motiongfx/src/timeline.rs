use bevy::prelude::*;

#[derive(Resource)]
pub struct Timeline {
    pub is_playing: bool,
    pub curr_time: f32,
    pub target_time: f32,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            curr_time: 0.0,
            target_time: 0.0,
        }
    }
}

/// Safely update the timings in the `Timeline` after performing all the necessary actions.
pub fn timeline_update_system(mut timeline: ResMut<Timeline>, time: Res<Time>) {
    timeline.curr_time = timeline.target_time;
    if timeline.is_playing {
        timeline.target_time += time.delta_seconds();
    }
}
