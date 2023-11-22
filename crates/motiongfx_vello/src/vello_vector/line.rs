use bevy_ecs::prelude::*;
use bevy_math::DVec2;
use bevy_utils::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo},
};
use motiongfx_core::prelude::*;

use crate::{
    stroke_style::{StrokeStyle, StrokeStyleMotion},
    vello_vector::{VelloBuilder, VelloVector},
};

#[derive(Bundle, Clone, Default)]
pub struct VelloLineBundle {
    pub line: VelloLine,
    pub stroke: StrokeStyle,
    pub fragment_bundle: VelloFragmentBundle,
}

pub struct VelloLineBundleMotion {
    pub line: VelloLineMotion,
    pub stroke: StrokeStyleMotion,
}

impl VelloLineBundleMotion {
    pub fn new(target_id: Entity, bundle: VelloLineBundle) -> Self {
        Self {
            line: VelloLineMotion::new(target_id, bundle.line),
            stroke: StrokeStyleMotion::new(target_id, bundle.stroke),
        }
    }
}

#[derive(Component, Clone)]
pub struct VelloLine {
    line: kurbo::Line,
    should_build: bool,
}

impl VelloLine {
    #[inline]
    pub fn new(line: impl Into<kurbo::Line>) -> Self {
        let line: kurbo::Line = line.into();

        Self { line, ..default() }
    }

    pub fn from_points(p0: impl Into<DVec2>, p1: impl Into<DVec2>) -> Self {
        let p0: DVec2 = p0.into();
        let p1: DVec2 = p1.into();

        Self {
            line: kurbo::Line::new(kurbo::Point::new(p0.x, p0.y), kurbo::Point::new(p1.x, p1.y)),
            ..default()
        }
    }

    pub fn origin_to(to: impl Into<DVec2>) -> Self {
        let to: DVec2 = to.into();

        Self {
            line: kurbo::Line::new(kurbo::Point::default(), kurbo::Point::new(to.x, to.y)),
            ..default()
        }
    }
}

impl VelloVector for VelloLine {
    #[inline]
    fn build_stroke(&self, stroke: &StrokeStyle, builder: &mut vello::SceneBuilder) {
        builder.stroke(
            &stroke.style,
            kurbo::Affine::default(),
            &stroke.brush,
            None,
            &self.line,
        );
    }
}

impl VelloBuilder for VelloLine {
    #[inline]
    fn should_build(&self) -> bool {
        self.should_build
    }

    #[inline]
    fn set_should_build(&mut self, should_build: bool) {
        self.should_build = should_build
    }
}

impl Default for VelloLine {
    fn default() -> Self {
        Self {
            line: kurbo::Line::new(kurbo::Point::default(), kurbo::Point::default()),
            should_build: false,
        }
    }
}

pub struct VelloLineMotion {
    target_id: Entity,
    vello_line: VelloLine,
}

impl VelloLineMotion {
    pub fn new(target_id: Entity, vello_line: VelloLine) -> Self {
        Self {
            target_id,
            vello_line,
        }
    }

    pub fn line_to(
        &mut self,
        new_line: impl Into<kurbo::Line>,
    ) -> Action<VelloLine, kurbo::Line, EmptyRes> {
        let new_line: kurbo::Line = new_line.into();

        let action: Action<VelloLine, kurbo::Line, EmptyRes> = Action::new(
            self.target_id,
            self.vello_line.line,
            new_line,
            Self::line_interp,
        );

        self.vello_line.line = new_line;

        action
    }

    /// Extend the line based on given `DVec2` percentage where `x` represents `p0` and `y` represents `p1`.
    pub fn percentage_extend(
        &mut self,
        extension: f64,
        percentage: impl Into<DVec2>,
    ) -> Action<VelloLine, kurbo::Line, EmptyRes> {
        let percentage: DVec2 = percentage.into();

        let mut new_line: kurbo::Line = self.vello_line.line;
        let direction: kurbo::Vec2 = self.get_p0_direction();

        new_line.p0 += direction * extension * percentage.x;
        new_line.p1 -= direction * extension * percentage.y;

        let action: Action<VelloLine, kurbo::Line, EmptyRes> = Action::new(
            self.target_id,
            self.vello_line.line,
            new_line,
            Self::line_interp,
        );

        self.vello_line.line = new_line;

        action
    }

    /// Extend the line only using `p0`.
    pub fn extend_p0(&mut self, extension: f64) -> Action<VelloLine, kurbo::Line, EmptyRes> {
        self.percentage_extend(extension, DVec2::new(1.0, 0.0))
    }

    /// Extend the line only using `p1`.
    pub fn extend_p1(&mut self, extension: f64) -> Action<VelloLine, kurbo::Line, EmptyRes> {
        self.percentage_extend(extension, DVec2::new(0.0, 1.0))
    }

    /// Extend the line on both `p0` and `p1`.
    pub fn extend(&mut self, extension: f64) -> Action<VelloLine, kurbo::Line, EmptyRes> {
        self.percentage_extend(extension, DVec2::new(1.0, 1.0))
    }

    fn get_p0_direction(&self) -> kurbo::Vec2 {
        (self.vello_line.line.p0 - self.vello_line.line.p1).normalize()
    }

    fn line_interp(
        vello_line: &mut VelloLine,
        begin: &kurbo::Line,
        end: &kurbo::Line,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        vello_line.line = kurbo::Line::lerp(begin, end, t);
        vello_line.set_should_build(true);
    }
}
