use bevy::{ecs::schedule::SystemConfigs, prelude::*};
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo, peniko},
};

pub mod bezpath;
pub mod circle;
pub mod line;
pub mod rect;

#[derive(Default, Clone)]
pub struct Brush {
    value: peniko::Brush,
    transform: Option<kurbo::Affine>,
}

impl Brush {
    pub fn from_brush(brush: peniko::Brush) -> Self {
        Self {
            value: brush,
            ..default()
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            value: peniko::Brush::Solid(peniko::Color::rgba(
                color.r() as f64,
                color.g() as f64,
                color.b() as f64,
                color.a() as f64,
            )),
            ..default()
        }
    }

    pub fn from_gradient(gradient: peniko::Gradient) -> Self {
        Self {
            value: peniko::Brush::Gradient(gradient),
            ..default()
        }
    }

    pub fn with_transform(mut self, transform: kurbo::Affine) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn clear_transform(&mut self) {
        self.transform = None;
    }
}

#[derive(Component, Clone)]
pub struct Fill {
    pub style: peniko::Fill,
    pub brush: Brush,
}

impl Fill {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_style(style: peniko::Fill) -> Self {
        Self { style, ..default() }
    }

    pub fn with_brush(mut self, brush: Brush) -> Self {
        self.brush = brush;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.brush = Brush::from_color(color);
        self
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            style: peniko::Fill::NonZero,
            brush: default(),
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct Stroke {
    pub style: kurbo::Stroke,
    pub brush: Brush,
}

impl Stroke {
    pub fn new(width: f64) -> Self {
        Self {
            style: kurbo::Stroke::new(width),
            ..default()
        }
    }

    pub fn from_style(style: kurbo::Stroke) -> Self {
        Self { style, ..default() }
    }

    pub fn with_brush(mut self, brush: Brush) -> Self {
        self.brush = brush;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.brush = Brush::from_color(color);
        self
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
            fill.brush.transform,
            &self.shape(),
        );
    }

    #[inline]
    fn build_stroke(&self, stroke: &Stroke, scene: &mut vello::Scene) {
        scene.stroke(
            &stroke.style,
            default(),
            &stroke.brush.value,
            stroke.brush.transform,
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
        (&Vector, &Fill, &Handle<VelloScene>),
        (Without<Stroke>, Or<(Changed<Vector>, Changed<Fill>)>),
    >,
    mut scenes: ResMut<Assets<VelloScene>>,
) {
    for (vector, fill, scene_handle) in q_vectors.iter_mut() {
        if let Some(vello_scene) = scenes.get_mut(scene_handle.id()) {
            let mut scene = vello::Scene::new();

            // Build the vector to the VelloScene
            vector.build_fill(fill, &mut scene);

            // Replace with new scene
            vello_scene.scene = scene.into();
        }
    }
}

#[allow(clippy::type_complexity)]
fn build_stroke_only_vector<Vector: VelloVector + Component>(
    mut q_vectors: Query<
        (&Vector, &Stroke, &Handle<VelloScene>),
        (Without<Fill>, Or<(Changed<Vector>, Changed<Stroke>)>),
    >,
    mut scenes: ResMut<Assets<VelloScene>>,
) {
    for (vector, stroke, scene_handle) in q_vectors.iter_mut() {
        if let Some(vello_scene) = scenes.get_mut(scene_handle.id()) {
            let mut scene = vello::Scene::new();

            // Build the vector to the VelloScene
            vector.build_stroke(stroke, &mut scene);

            // Replace with new scene
            vello_scene.scene = scene.into();
        }
    }
}

#[allow(clippy::type_complexity)]
fn build_fill_and_stroke_vector<Vector: VelloVector + Component>(
    mut q_vectors: Query<
        (&Vector, &Fill, &Stroke, &Handle<VelloScene>),
        Or<(Changed<Vector>, Changed<Fill>, Changed<Stroke>)>,
    >,
    mut scenes: ResMut<Assets<VelloScene>>,
) {
    for (vector, fill, stroke, scene_handle) in q_vectors.iter_mut() {
        if let Some(vello_scene) = scenes.get_mut(scene_handle.id()) {
            let mut scene = vello::Scene::new();

            // Build the vector to the VelloScene
            vector.build_fill(fill, &mut scene);
            // Skip building stroke if there is no width to it
            if stroke.style.width != 0.0 {
                vector.build_stroke(stroke, &mut scene);
            }

            // Replace with new scene
            vello_scene.scene = scene.into();
        }
    }
}
