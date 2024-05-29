use bevy::{ecs::schedule::SystemConfigs, prelude::*};
use bevy_vello::{prelude::*, VelloPlugin};
use bezpath::VelloBezPath;
use circle::VelloCircle;
use fill::Fill;
use line::VelloLine;
use rect::VelloRect;
use stroke::Stroke;

pub mod arrow;
pub mod bezpath;
pub mod brush;
pub mod circle;
pub mod fill;
pub mod line;
pub mod rect;
pub mod stroke;

pub mod prelude {
    pub use crate::VelloGraphicsPlugin;
    pub use crate::{
        arrow::VelloArrow, bezpath::VelloBezPath, circle::VelloCircle, line::VelloLine,
        rect::VelloRect,
    };
    pub use crate::{brush::Brush, fill::Fill, stroke::Stroke};
}

pub struct VelloGraphicsPlugin;

impl Plugin for VelloGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloPlugin).add_systems(
            Update,
            (
                build_vector::<VelloRect>(),
                build_vector::<VelloCircle>(),
                build_vector::<VelloLine>(),
                build_vector::<VelloBezPath>(),
            ),
        );
    }
}

pub trait VelloVector {
    fn shape(&self) -> impl kurbo::Shape;

    #[inline]
    fn build_fill(&self, fill: &Fill, scene: &mut vello::Scene) {
        scene.fill(
            fill.style,
            default(),
            &fill.brush.value,
            Some(fill.brush.transform),
            &self.shape(),
        );
    }

    #[inline]
    fn build_stroke(&self, stroke: &Stroke, scene: &mut vello::Scene) {
        scene.stroke(
            &stroke.style,
            default(),
            &stroke.brush.value,
            Some(stroke.brush.transform),
            &self.shape(),
        );
    }
}

pub(crate) fn build_vector<Vector: VelloVector + Component>() -> SystemConfigs {
    (
        build_fill_only_vector::<Vector>,
        build_stroke_only_vector::<Vector>,
        build_fill_and_stroke_vector::<Vector>,
    )
        .into_configs()
}

#[allow(clippy::type_complexity)]
fn build_fill_only_vector<Vector: VelloVector + Component>(
    mut q_vectors: Query<
        (&Vector, &Fill, &mut VelloScene),
        (Without<Stroke>, Or<(Changed<Vector>, Changed<Fill>)>),
    >,
) {
    for (vector, fill, mut scene) in q_vectors.iter_mut() {
        *scene = VelloScene::default();

        // Build the vector to the VelloScene
        vector.build_fill(fill, &mut scene);
    }
}

#[allow(clippy::type_complexity)]
fn build_stroke_only_vector<Vector: VelloVector + Component>(
    mut q_vectors: Query<
        (&Vector, &Stroke, &mut VelloScene),
        (Without<Fill>, Or<(Changed<Vector>, Changed<Stroke>)>),
    >,
) {
    for (vector, stroke, mut scene) in q_vectors.iter_mut() {
        *scene = VelloScene::default();

        // Build the vector to the VelloScene
        vector.build_stroke(stroke, &mut scene);
    }
}

#[allow(clippy::type_complexity)]
fn build_fill_and_stroke_vector<Vector: VelloVector + Component>(
    mut q_vectors: Query<
        (&Vector, &Fill, &Stroke, &mut VelloScene),
        Or<(Changed<Vector>, Changed<Fill>, Changed<Stroke>)>,
    >,
) {
    for (vector, fill, stroke, mut scene) in q_vectors.iter_mut() {
        *scene = VelloScene::default();

        // Build the vector to the VelloScene
        vector.build_fill(fill, &mut scene);
        vector.build_stroke(stroke, &mut scene);
    }
}
