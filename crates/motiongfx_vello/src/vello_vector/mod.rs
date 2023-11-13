use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_vello_renderer::prelude::*;

pub mod rect;

pub(crate) trait VelloVector {
    fn build(&self, fragment: &mut VelloFragment);
}

pub(crate) fn vello_rect_init<Vector: VelloVector + Component>(
    q_vectors: Query<(&Vector, &Handle<VelloFragment>)>,
    mut fragments: ResMut<Assets<VelloFragment>>,
) {
    for (vector, fragment_handle) in q_vectors.iter() {
        if let Some(fragment) = fragments.get_mut(fragment_handle.id()) {
            vector.build(fragment);
        }
    }
}
