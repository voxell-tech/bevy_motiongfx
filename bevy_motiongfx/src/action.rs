use crate::{
    ease::{quad, EaseFn},
    sequence::Sequence,
};
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
    pub(super) action_id: Entity,
    /// Time at which animation should begin.
    start_time: f32,
    /// Duration of animation in seconds.
    duration: f32,
    /// Easing function to be used for animation.
    pub(crate) ease_fn: EaseFn,
}

impl ActionMeta {
    pub fn new(action_id: Entity, start_time: f32, duration: f32) -> Self {
        Self {
            action_id,
            start_time,
            duration,
            ease_fn: quad::ease_in_out,
        }
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

// TODO: Testing only, remove when done
fn translate_action(target_id: Entity, begin: Vec3, end: Vec3) -> Action<Transform, Vec3> {
    Action {
        target_id,
        begin,
        end,
        interp_fn: translate_interp,
    }
}

fn translate_interp(transform: &mut Transform, begin: &Vec3, end: &Vec3, t: f32) {
    transform.translation = Vec3::lerp(*begin, *end, t);
}

pub fn test<'a>(
    mut commands: Commands<'a, 'a>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sequence: ResMut<Sequence>,
) {
    let target_id: Entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            ..default()
        })
        .id();

    let action: Action<Transform, Vec3> = translate_action(target_id, Vec3::ZERO, Vec3::ONE);

    sequence
        .all(&mut commands)
        .add_action(action, 1.0)
        .add_action(action, 1.0)
        .add_action(action, 1.0)
        .add_action(action, 1.0);
    sequence.all(&mut commands).add_action(action.clone(), 1.0);
    // sequence.all(&mut commands).add_action(action.clone(), 1.0);
}
