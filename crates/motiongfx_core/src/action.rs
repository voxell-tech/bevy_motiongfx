use bevy::prelude::*;

use crate::{
    ease::{cubic, EaseFn},
    f32lerp::F32Lerp,
    sequence::Sequence,
};

/// Function for interpolating a type based on a [`f32`] time.
pub type InterpFn<T> = fn(start: &T, end: &T, t: f32) -> T;
/// Function for getting a mutable reference of a field (or itself) of type `T` in type `U`.
pub type GetFieldMut<T, U> = fn(comp: &mut U) -> &mut T;

#[macro_export]
macro_rules! act {
    (
        ($target_id:expr, $comp_ty:ty),
        start = { $root:expr }.$($path:tt).+,
        end = $value:expr,
    ) => {
        {
            let action = $crate::action::Action::new_f32lerp(
                $target_id,
                $root.$($path).+.clone(),
                $value.clone(),
                |source: &mut $comp_ty| &mut source.$($path).+
            );

            $root.$($path).+ = $value;

            action
        }
    };
    (
        ($target_id:expr, $comp_ty:ty),
        start = { $root:expr },
        end = $value:expr,
    ) => {
        {
            let action = $crate::action::Action::new_f32lerp(
                $target_id,
                $root.clone(),
                $value.clone(),
                |source: &mut $comp_ty| source
            );

            #[allow(unused_assignments)]
            {
                $root = $value;
            }

            action
        }
    };
}

#[macro_export]
macro_rules! play {
    ($commands:expr, $motion:expr,) => {
        $crate::action::ActionBuilderExtension::play(&mut $commands, $motion)
    };
    ($commands:expr, $($motion:expr,)+) => {
        {
            let commands = &mut $commands;
            [
                $($crate::action::ActionBuilderExtension::play(commands, ($motion))),+
            ]
        }
    };
}

pub use act;
pub use play;

/// Basic data structure to describe an animation action.
#[derive(Component, Clone, Copy)]
pub struct Action<T, U> {
    /// Target [`Entity`] for [`Component`] manipulation.
    pub(crate) target_id: Entity,
    /// Initial value of the action.
    pub(crate) start: T,
    /// Final value of the action.
    pub(crate) end: T,
    /// Function for interpolating the value based on a [`f32`] time.
    pub(crate) interp_fn: InterpFn<T>,
    /// Function for getting a mutable reference of a field (or itself) from the component.
    pub(crate) get_field_fn: GetFieldMut<T, U>,
    /// Function for easing the [`f32`] time value for the action.
    pub(crate) ease_fn: EaseFn,
}

impl<T, U> Action<T, U> {
    pub fn new(
        target_id: Entity,
        start: T,
        end: T,
        interp_fn: InterpFn<T>,
        get_field_fn: GetFieldMut<T, U>,
    ) -> Self {
        Self {
            target_id,
            start,
            end,
            interp_fn,
            get_field_fn,
            ease_fn: cubic::ease_in_out,
        }
    }

    pub fn with_ease(mut self, ease_fn: EaseFn) -> Self {
        self.ease_fn = ease_fn;
        self
    }

    pub fn with_interp(mut self, interp_fn: InterpFn<T>) -> Self {
        self.interp_fn = interp_fn;
        self
    }

    pub fn animate(self, duration: f32) -> Motion<T, U> {
        Motion {
            action: self,
            duration,
        }
    }
}

impl<T, U> Action<T, U>
where
    T: F32Lerp,
{
    pub fn new_f32lerp(
        target_id: Entity,
        start: T,
        end: T,
        get_field_fn: GetFieldMut<T, U>,
    ) -> Self {
        Self {
            target_id,
            start,
            end,
            interp_fn: T::f32lerp,
            get_field_fn,
            ease_fn: cubic::ease_in_out,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ActionMeta {
    /// Target `Entity` for `Action`.
    action_id: Entity,
    /// Time at which animation should begin.
    pub(crate) start_time: f32,
    /// Duration of animation in seconds.
    pub(crate) duration: f32,
    /// Slide that this action belongs to.
    pub(crate) slide_index: usize,
}

impl ActionMeta {
    pub fn new(action_id: Entity) -> Self {
        Self {
            action_id,
            start_time: 0.0,
            duration: 0.0,
            slide_index: 0,
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
    pub fn end_time(&self) -> f32 {
        self.start_time + self.duration
    }
}

pub struct Motion<T, U> {
    pub action: Action<T, U>,
    pub duration: f32,
}

pub trait ActionBuilderExtension {
    fn play<T, U>(&mut self, motion: Motion<T, U>) -> Sequence
    where
        T: Send + Sync + 'static,
        U: Send + Sync + 'static;

    fn sleep(&mut self, duration: f32) -> Sequence;
}

impl ActionBuilderExtension for Commands<'_, '_> {
    fn play<T, U>(&mut self, motion: Motion<T, U>) -> Sequence
    where
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        let action_id = self.spawn(motion.action).id();
        let mut action_meta = ActionMeta::new(action_id);
        action_meta.duration = motion.duration;

        Sequence::single(action_meta)
    }

    fn sleep(&mut self, duration: f32) -> Sequence {
        Sequence::empty(duration)
    }
}
