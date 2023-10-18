use crate::{
    action::{Action, ActionMeta},
    ease::EaseFn,
    timeline::Timeline,
};
use bevy::prelude::*;

/// An array of `Action`.
#[derive(Resource)]
pub struct Sequence {
    duration: f32,
    action_metas: Vec<ActionMeta>,
}

impl Sequence {
    pub fn new() -> Self {
        Sequence {
            duration: 0.0,
            action_metas: Vec::new(),
        }
    }

    pub fn play(
        &mut self,
        commands: &mut Commands,
        action: Action<impl Component, impl Send + Sync + 'static>,
        duration: f32,
    ) {
        // Spawn action into a new entity
        let action_id: Entity = commands.spawn(action).id();

        self.action_metas.push(ActionMeta::new(
            action_id,
            self.duration,
            self.duration + duration,
        ));

        // Increment the total duration
        self.duration += duration;
    }

    pub fn all<'a, 'w, 's>(
        &'a mut self,
        commands: &'a mut Commands<'w, 's>,
    ) -> impl SeqBuilder<'a, 'w, 's> {
        AllSeqBuilder::<'a, 'w, 's>::new(self, commands, self.duration)
    }

    pub fn chain<'a, 'w, 's>(
        &'a mut self,
        commands: &'a mut Commands<'w, 's>,
    ) -> ChainSeqBuilder<'a, 'w, 's> {
        ChainSeqBuilder::<'a, 'w, 's>::new(self, commands, self.duration)
    }

    // pub fn delay(&mut self, delay: f32) {}

    // pub fn flow(&mut self, delay: f32) {}

    pub fn duration(&self) -> f32 {
        self.duration
    }
}

pub struct AllSeqBuilder<'a, 'w, 's> {
    sequence: &'a mut Sequence,
    commands: &'a mut Commands<'w, 's>,
    start_time: f32,
    duration: f32,
    action_metas: Vec<ActionMeta>,
}

impl<'a, 'w, 's> SeqBuilder<'a, 'w, 's> for AllSeqBuilder<'a, 'w, 's> {
    fn new(
        sequence: &'a mut Sequence,
        commands: &'a mut Commands<'w, 's>,
        start_time: f32,
    ) -> Self {
        Self {
            sequence,
            commands,
            start_time,
            duration: 0.0,
            action_metas: Vec::new(),
        }
    }

    fn play(
        mut self,
        action: Action<impl Component, impl Send + Sync + 'static>,
        duration: f32,
    ) -> Self {
        let action_id: Entity = self.commands.spawn(action).id();

        self.action_metas
            .push(ActionMeta::new(action_id, self.start_time, duration));

        self.duration = f32::max(self.duration, duration);

        self
    }

    fn play_ease(
        mut self,
        action: Action<impl Component, impl Send + Sync + 'static>,
        duration: f32,
        ease_fn: EaseFn,
    ) -> Self {
        let action_id: Entity = self.commands.spawn(action).id();

        self.action_metas
            .push(ActionMeta::new(action_id, self.start_time, duration).with_ease(ease_fn));

        self.duration = f32::max(self.duration, duration);

        self
    }

    fn build(mut self) {
        self.sequence.action_metas.append(&mut self.action_metas);
        self.sequence.duration += self.duration;
    }
}

pub struct ChainSeqBuilder<'a, 'w, 's> {
    sequence: &'a mut Sequence,
    commands: &'a mut Commands<'w, 's>,
    start_time: f32,
    duration: f32,
    action_metas: Vec<ActionMeta>,
}

impl<'a, 'w, 's> SeqBuilder<'a, 'w, 's> for ChainSeqBuilder<'a, 'w, 's> {
    fn new(
        sequence: &'a mut Sequence,
        commands: &'a mut Commands<'w, 's>,
        start_time: f32,
    ) -> Self {
        Self {
            sequence,
            commands,
            start_time,
            duration: 0.0,
            action_metas: Vec::new(),
        }
    }

    fn play(
        mut self,
        action: Action<impl Component, impl Send + Sync + 'static>,
        duration: f32,
    ) -> Self {
        let action_id: Entity = self.commands.spawn(action).id();

        self.action_metas.push(ActionMeta::new(
            action_id,
            self.start_time + self.duration,
            duration,
        ));

        self.duration += duration;

        self
    }

    fn play_ease(
        mut self,
        action: Action<impl Component, impl Send + Sync + 'static>,
        duration: f32,
        ease_fn: EaseFn,
    ) -> Self {
        let action_id: Entity = self.commands.spawn(action).id();

        self.action_metas.push(
            ActionMeta::new(action_id, self.start_time + self.duration, duration)
                .with_ease(ease_fn),
        );

        self.duration += duration;

        self
    }

    fn build(mut self) {
        self.sequence.action_metas.append(&mut self.action_metas);
        self.sequence.duration += self.duration;
    }
}

pub trait SeqBuilder<'a, 'w, 's> {
    fn new(sequence: &'a mut Sequence, commands: &'a mut Commands<'w, 's>, start_time: f32)
        -> Self;

    fn play(
        self,
        action: Action<impl Component, impl Send + Sync + 'static>,
        duration: f32,
    ) -> Self;

    fn play_ease(
        self,
        action: Action<impl Component, impl Send + Sync + 'static>,
        duration: f32,
        ease_fn: EaseFn,
    ) -> Self;

    fn build(self);
}

pub fn sequence_player_system<C: Component, T: Send + Sync + 'static>(
    mut q_component: Query<&mut C>,
    q_actions: Query<&Action<C, T>>,
    scene: Res<Sequence>,
    timeline: Res<Timeline>,
) {
    // Do not perform any actions if there are no changes to the timeline timings.
    if timeline.curr_time == timeline.target_time {
        return;
    }

    let direction: i32 = f32::signum(timeline.target_time - timeline.curr_time) as i32;

    let timeline_start: f32 = f32::min(timeline.curr_time, timeline.target_time);
    let timeline_end: f32 = f32::max(timeline.curr_time, timeline.target_time);

    let mut start_index: usize = 0;
    let mut end_index: usize = scene.action_metas.len() - 1;

    // Swap direction if needed
    if direction == -1 {
        start_index = end_index;
        end_index = 0;
    }

    let mut action_index: usize = start_index;

    loop {
        if action_index == (end_index as i32 + direction) as usize {
            break;
        }

        let action_meta: &ActionMeta = &scene.action_metas[action_index];
        let action_id: Entity = action_meta.action_id;

        action_index = (action_index as i32 + direction) as usize;

        // Ignore if `ActionMeta` not in range
        if !time_range_overlap(
            action_meta.start_time(),
            action_meta.end_time(),
            timeline_start,
            timeline_end,
        ) {
            continue;
        }

        if let Ok(action) = q_actions.get(action_id) {
            // Get component to mutate based on action id
            if let Ok(mut component) = q_component.get_mut(action.target_id) {
                let mut unit_time: f32 =
                    (timeline.target_time - action_meta.start_time()) / action_meta.duration();

                // In case of division by 0.0
                if f32::is_nan(unit_time) {
                    unit_time = 0.0;
                }

                unit_time = f32::clamp(unit_time, 0.0, 1.0);
                // Calculate unit time using ease function
                unit_time = (action_meta.ease_fn)(unit_time);

                // Mutate the component using interpolate function
                (action.interp_fn)(&mut component, &action.begin, &action.end, unit_time);
            }
        }
    }
}

/// Calculate if 2 time range (in float) overlaps.
fn time_range_overlap(a_begin: f32, a_end: f32, b_begin: f32, b_end: f32) -> bool {
    a_begin <= b_end && b_begin <= a_end
}
