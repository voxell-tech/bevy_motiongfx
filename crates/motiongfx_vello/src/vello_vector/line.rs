use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo, peniko},
};
use motiongfx_core::{action::Action, EmptyRes};

use crate::{
    impl_builder_macros::{impl_brush_builder, impl_stroke_builder, impl_transform_motion},
    stroke_style::StrokeStyle,
    vello_vector::VelloVector,
};

use super::_VelloVector;

#[derive(Bundle, Default, Clone)]
pub struct VelloLineBundle {
    pub line: VelloLine,
    pub stroke: StrokeStyle,
    pub scene_bundle: VelloSceneBundle,
}

#[derive(VelloVector, Component, Clone)]
pub struct VelloLine {
    #[shape]
    pub line: kurbo::Line,
}

impl VelloLine {
    #[inline]
    pub fn new(line: kurbo::Line) -> Self {
        Self { line, ..default() }
    }

    pub fn from_points(p0: DVec2, p1: DVec2) -> Self {
        Self {
            line: kurbo::Line::new(kurbo::Point::new(p0.x, p0.y), kurbo::Point::new(p1.x, p1.y)),
            ..default()
        }
    }

    pub fn origin_to(to: DVec2) -> Self {
        Self {
            line: kurbo::Line::new(kurbo::Point::default(), kurbo::Point::new(to.x, to.y)),
            ..default()
        }
    }
}

impl Default for VelloLine {
    fn default() -> Self {
        Self {
            line: kurbo::Line::new(kurbo::Point::default(), kurbo::Point::default()),
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct _VelloLine {
    pub p0: DVec2,
    pub p1: DVec2,
    // Stroke
    pub stroke: kurbo::Stroke,
    pub stroke_brush: peniko::Brush,
    pub stroke_transform: Option<kurbo::Affine>,
}

impl _VelloLine {
    pub fn new(p0: DVec2, p1: DVec2) -> Self {
        Self::default().with_p0(p0).with_p1(p1)
    }

    pub fn with_p0(mut self, p0: DVec2) -> Self {
        self.p0 = p0;
        self
    }

    pub fn with_p1(mut self, p1: DVec2) -> Self {
        self.p1 = p1;
        self
    }

    pub fn build(
        self,
        commands: &mut Commands,
        scenes: &mut Assets<VelloScene>,
    ) -> _VelloLineMotion {
        let target_id = commands
            .spawn((
                self.clone(),
                VelloSceneBundle {
                    scene: scenes.add(VelloScene::default()),
                    ..default()
                },
            ))
            .id();

        _VelloLineMotion::new(target_id, self, Transform::default())
    }
}

impl_brush_builder!(stroke, _VelloLine, stroke_brush);
impl_stroke_builder!(_VelloLine, stroke);

impl _VelloVector for _VelloLine {
    fn build_scene(&self) -> bevy_vello_renderer::vello::Scene {
        let mut scene = vello::Scene::new();

        let line = kurbo::Line::new(
            kurbo::Point::new(self.p0.x, self.p0.y),
            kurbo::Point::new(self.p1.x, self.p1.y),
        );

        scene.stroke(
            &self.stroke,
            default(),
            &self.stroke_brush,
            self.stroke_transform,
            &line,
        );

        scene
    }
}

pub struct _VelloLineMotion {
    target_id: Entity,
    line: _VelloLine,
    transform: Transform,
}

impl _VelloLineMotion {
    pub fn new(target_id: Entity, line: _VelloLine, transform: Transform) -> Self {
        Self {
            target_id,
            line,
            transform,
        }
    }

    pub fn p0_to(&mut self, p0: DVec2) -> Action<_VelloLine, DVec2, EmptyRes> {
        let action = Action::new(
            self.target_id,
            self.line.p0,
            p0,
            |line: &mut _VelloLine, begin, end, t, _| {
                line.p0 = DVec2::lerp(*begin, *end, t as f64);
            },
        );

        self.line.p0 = p0;
        action
    }

    pub fn p1_to(&mut self, p1: DVec2) -> Action<_VelloLine, DVec2, EmptyRes> {
        let action = Action::new(
            self.target_id,
            self.line.p1,
            p1,
            |line: &mut _VelloLine, begin, end, t, _| {
                line.p1 = DVec2::lerp(*begin, *end, t as f64);
            },
        );

        self.line.p1 = p1;
        action
    }
}

impl_transform_motion!(_VelloLineMotion, transform, target_id);
