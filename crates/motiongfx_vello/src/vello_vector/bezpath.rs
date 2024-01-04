use bevy_ecs::prelude::*;
use bevy_utils::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo},
};

use crate::{
    fill_style::FillStyle,
    stroke_style::StrokeStyle,
    vello_vector::{VelloBuilder, VelloVector},
};

#[derive(Bundle, Clone, Default)]
pub struct VelloBezPathBundle {
    pub path: VelloBezPath,
    pub fill: FillStyle,
    pub stroke: StrokeStyle,
    pub fragment_bundle: VelloFragmentBundle,
}

#[derive(Component, Clone)]
pub struct VelloBezPath {
    pub(crate) path: kurbo::BezPath,
    built: bool,
}

impl VelloBezPath {
    pub fn new(path: impl Into<kurbo::BezPath>) -> Self {
        let path: kurbo::BezPath = path.into();

        Self { path, ..default() }
    }
}

impl VelloVector for VelloBezPath {
    #[inline]
    fn build_fill(&self, fill: &FillStyle, builder: &mut vello::SceneBuilder) {
        builder.fill(
            fill.style,
            kurbo::Affine::default(),
            &fill.brush,
            None,
            &self.path,
        );
    }

    #[inline]
    fn build_stroke(&self, stroke: &StrokeStyle, builder: &mut vello::SceneBuilder) {
        builder.stroke(
            &stroke.style,
            kurbo::Affine::default(),
            &stroke.brush,
            None,
            &self.path,
        );
    }
}

impl VelloBuilder for VelloBezPath {
    #[inline]
    fn is_built(&self) -> bool {
        self.built
    }

    #[inline]
    fn set_built(&mut self, built: bool) {
        self.built = built;
    }
}

impl Default for VelloBezPath {
    fn default() -> Self {
        Self {
            path: kurbo::BezPath::new(),
            built: false,
        }
    }
}
