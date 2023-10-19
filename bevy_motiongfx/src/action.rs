use crate::ease::{quad, EaseFn};
use bevy::prelude::*;

pub type InterpFn<C, T> = fn(component: &mut C, begin: &T, end: &T, t: f32);

/// Basic data structure to describe an animation action.
#[derive(Component, Clone, Copy)]
pub struct Action<C: Component, T: Send + Sync + 'static> {
    /// Target `Entity` for `Component` manipulation.
    pub(crate) target_id: Entity,
    /// Initial state of the animation.
    pub(crate) begin: T,
    /// Final state of the animation.
    pub(crate) end: T,
    /// Interpolation function to be used for animation.
    pub(crate) interp_fn: InterpFn<C, T>,
}

impl<C: Component, T: Send + Sync + 'static> Action<C, T> {
    pub fn new(target_id: Entity, begin: T, end: T, interp_fn: InterpFn<C, T>) -> Self {
        Self {
            target_id,
            begin,
            end,
            interp_fn,
        }
    }
}

#[derive(Clone)]
pub struct ActionMeta {
    /// Target `Entity` for `Action`.
    action_id: Entity,
    /// Time at which animation should begin.
    start_time: f32,
    /// Duration of animation in seconds.
    duration: f32,
    /// Easing function to be used for animation.
    pub(crate) ease_fn: EaseFn,
}

impl ActionMeta {
    pub fn new(action_id: Entity) -> Self {
        Self {
            action_id,
            start_time: 0.0,
            duration: 0.0,
            ease_fn: quad::ease_in_out,
        }
    }

    pub fn id(&self) -> Entity {
        self.action_id
    }

    #[inline]
    pub fn with_start_time(mut self, start_time: f32) -> Self {
        self.start_time = start_time;
        self
    }

    #[inline]
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    #[inline]
    pub fn with_ease(mut self, ease_fn: EaseFn) -> Self {
        self.ease_fn = ease_fn;
        self
    }

    #[inline]
    pub fn start_time(&self) -> f32 {
        self.start_time
    }

    #[inline]
    pub fn end_time(&self) -> f32 {
        self.start_time + self.duration
    }

    #[inline]
    pub fn duration(&self) -> f32 {
        self.duration
    }
}
