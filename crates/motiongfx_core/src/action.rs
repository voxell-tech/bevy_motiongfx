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
        $target_id:expr,
        $type:ty = $root:expr => $($path:tt).+,
        $value:expr,
        $interp_fn:expr
    ) => {
        {
            let action = $crate::action::Action::new(
                $target_id,
                $root.$($path).+.clone(),
                $value.clone(),
                $interp_fn,
                |source: &mut $type| &mut source.$($path).+
            );

            $root.$($path).+ = $value;

            action
        }
    };
    (
        $target_id:expr,
        $type:ty = $root:expr => $($path:tt).+,
        $value:expr
    ) => {
        {
            let action = $crate::action::Action::new_f32lerp(
                $target_id,
                $root.$($path).+.clone(),
                $value.clone(),
                |source: &mut $type| &mut source.$($path).+
            );

            $root.$($path).+ = $value;

            action
        }
    };
}

pub use act;

/// Basic data structure to describe an animation action.
#[derive(Component, Clone, Copy)]
pub struct Action<T: Clone, C: Component> {
    /// Target [`Entity`] for [`Component`] manipulation.
    pub(crate) target_id: Entity,
    /// Initial value of the action.
    pub(crate) start: T,
    /// Final value of the action.
    pub(crate) end: T,
    /// Function for interpolating the value based on a [`f32`] time.
    pub(crate) interp_fn: InterpFn<T>,
    /// Function for getting a mutable reference of a field (or itself) from the component.
    pub(crate) get_field_fn: GetFieldMut<T, C>,
    /// Function for easing the [`f32`] time value for the action.
    pub(crate) ease_fn: EaseFn,
}

impl<T: Clone, C: Component> Action<T, C> {
    pub fn new(
        target_id: Entity,
        start: T,
        end: T,
        interp_fn: InterpFn<T>,
        get_field_fn: GetFieldMut<T, C>,
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
}

impl<T: Clone, C: Component> Action<T, C>
where
    T: F32Lerp,
{
    pub fn new_f32lerp(
        target_id: Entity,
        start: T,
        end: T,
        get_field_fn: GetFieldMut<T, C>,
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

pub trait ActionBuilder {
    fn play<T: Clone, C: Component>(&mut self, action: Action<T, C>, duration: f32) -> Sequence
    where
        T: Send + Sync + 'static;

    fn sleep(&mut self, duration: f32) -> Sequence;
}

impl ActionBuilder for Commands<'_, '_> {
    fn play<T: Clone, C: Component>(&mut self, action: Action<T, C>, duration: f32) -> Sequence
    where
        T: Send + Sync + 'static,
    {
        let action_id = self.spawn(action).id();
        let mut action_meta = ActionMeta::new(action_id);
        action_meta.duration = duration;

        // TODO: create single sequence
        Sequence::single(action_meta)
    }

    fn sleep(&mut self, duration: f32) -> Sequence {
        Sequence::empty(duration)
    }
}
