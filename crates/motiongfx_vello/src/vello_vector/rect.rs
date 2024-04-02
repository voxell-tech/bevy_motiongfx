use bevy::{
    math::{DVec2, DVec4},
    prelude::*,
};
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo, peniko},
};
use motiongfx_core::{action::Action, EmptyRes};

use crate::{
    fill_style::FillStyle,
    impl_builder_macros::{
        impl_brush_builder, impl_optional_stroke_builder, impl_transform_motion,
    },
    stroke_style::StrokeStyle,
    vello_vector::VelloVector,
};

use super::_VelloVector;

#[derive(Bundle, Clone, Default)]
pub struct VelloRectBundle {
    pub rect: VelloRect,
    pub fill: FillStyle,
    pub stroke: StrokeStyle,
    pub scene_bundle: VelloSceneBundle,
}

#[derive(Component, Clone, Default)]
pub struct VelloRect {
    /// Coordinates of the rectangle.
    pub rect: kurbo::Rect,
    /// Radius of all four corners.
    pub radii: kurbo::RoundedRectRadii,
}

impl VelloRect {
    pub fn new(rect: kurbo::Rect, radii: kurbo::RoundedRectRadii) -> Self {
        let radii: kurbo::RoundedRectRadii = radii.into();

        Self {
            rect,
            radii,
            ..default()
        }
    }

    pub fn percentage_anchor(size: DVec2, radius: DVec4, percentage: DVec2) -> Self {
        Self::new(
            kurbo::Rect::new(
                -size.x * percentage.x,
                -size.y * percentage.y,
                size.x * (1.0 - percentage.x),
                size.y * (1.0 - percentage.y),
            ),
            kurbo::RoundedRectRadii::new(radius.x, radius.y, radius.z, radius.w),
        )
    }

    #[inline]
    pub fn anchor_center(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 0.5))
    }

    #[inline]
    pub fn anchor_left(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(1.0, 0.5))
    }

    #[inline]
    pub fn anchor_right(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.0, 0.5))
    }

    #[inline]
    pub fn anchor_bottom(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 0.0))
    }

    #[inline]
    pub fn anchor_top(size: DVec2, radius: DVec4) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 1.0))
    }
}

impl VelloVector for VelloRect {
    #[inline]
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::RoundedRect::from_rect(self.rect, self.radii)
    }
}

#[derive(Component, Default, Clone)]
pub struct _VelloRect {
    pub size: DVec2,
    pub anchor: DVec2,
    // Fill
    pub fill_brush: peniko::Brush,
    pub fill_transform: Option<kurbo::Affine>,
    // Stroke
    pub stroke: Option<kurbo::Stroke>,
    pub stroke_brush: peniko::Brush,
    pub stroke_transform: Option<kurbo::Affine>,
}

impl _VelloRect {
    pub fn new(width: f64, height: f64) -> Self {
        Self::default().with_size(width, height)
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = DVec2::new(width, height);
        self
    }

    pub fn with_anchor(mut self, x: f64, y: f64) -> Self {
        self.anchor = DVec2::new(x, y);
        self
    }

    pub fn build(
        self,
        commands: &mut Commands,
        scenes: &mut Assets<VelloScene>,
    ) -> _VelloRectMotion {
        let target_id = commands
            .spawn((
                self.clone(),
                VelloSceneBundle {
                    scene: scenes.add(VelloScene::default()),
                    ..default()
                },
            ))
            .id();

        _VelloRectMotion::new(target_id, self, Transform::default())
    }
}

impl_brush_builder!(fill, _VelloRect, fill_brush);
impl_brush_builder!(stroke, _VelloRect, stroke_brush);
impl_optional_stroke_builder!(_VelloRect, stroke);

impl _VelloVector for _VelloRect {
    fn build_scene(&self) -> vello::Scene {
        let mut scene = vello::Scene::new();

        let rect = kurbo::Rect::new(
            -self.size.x * self.anchor.x,
            -self.size.y * self.anchor.y,
            self.size.x * (1.0 - self.anchor.x),
            self.size.y * (1.0 - self.anchor.y),
        );

        scene.fill(
            peniko::Fill::NonZero,
            default(),
            &self.fill_brush,
            self.fill_transform,
            &rect,
        );

        if let Some(stroke) = &self.stroke {
            scene.stroke(
                stroke,
                default(),
                &self.stroke_brush,
                self.stroke_transform,
                &rect,
            );
        }

        scene
    }
}

pub struct _VelloRectMotion {
    target_id: Entity,
    rect: _VelloRect,
    transform: Transform,
}

impl _VelloRectMotion {
    pub fn new(target_id: Entity, rect: _VelloRect, transform: Transform) -> Self {
        Self {
            target_id,
            rect,
            transform,
        }
    }

    pub fn size_to(&mut self, width: f64, height: f64) -> Action<_VelloRect, DVec2, EmptyRes> {
        let new_size = DVec2::new(width, height);

        let action = Action::new(
            self.target_id,
            self.rect.size,
            new_size,
            |rect: &mut _VelloRect, begin, end, t, _| {
                rect.size = DVec2::lerp(*begin, *end, t as f64);
            },
        );

        self.rect.size = new_size;
        action
    }

    pub fn size_add(&mut self, width: f64, height: f64) -> Action<_VelloRect, DVec2, EmptyRes> {
        let new_size = self.rect.size + DVec2::new(width, height);

        let action = Action::new(
            self.target_id,
            self.rect.size,
            new_size,
            |rect: &mut _VelloRect, begin, end, t, _| {
                rect.size = DVec2::lerp(*begin, *end, t as f64);
            },
        );

        self.rect.size = new_size;
        action
    }

    pub fn anchor_to(&mut self, x: f64, y: f64) -> Action<_VelloRect, DVec2, EmptyRes> {
        let new_anchor = DVec2::new(x, y);

        let action = Action::new(
            self.target_id,
            self.rect.anchor,
            new_anchor,
            |rect: &mut _VelloRect, begin, end, t, _| {
                rect.anchor = DVec2::lerp(*begin, *end, t as f64);
            },
        );

        self.rect.anchor = new_anchor;
        action
    }
}

impl_transform_motion!(_VelloRectMotion, transform, target_id);
