use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::{DVec2, DVec4};
use bevy_utils::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{self, kurbo, peniko},
};
use motiongfx_core::prelude::*;

use crate::vector_style::{FillStyle, StrokeStyle};
use crate::vello_vector::VelloVector;

#[derive(Bundle, Clone, Default)]
pub struct VelloRectBundle {
    pub rect: VelloRect,
    pub fragment_bundle: VelloFragmentBundle,
}

#[derive(Component, Clone, Default)]
pub struct VelloRect {
    /// Coordinates of the rectangle.
    rect: kurbo::Rect,
    /// Radius of all four corners.
    radii: kurbo::RoundedRectRadii,
    fill: FillStyle,
    stroke: StrokeStyle,
}

impl VelloRect {
    #[inline]
    pub fn new(rect: impl Into<kurbo::Rect>, radii: impl Into<kurbo::RoundedRectRadii>) -> Self {
        let rect: kurbo::Rect = rect.into();
        let radii: kurbo::RoundedRectRadii = radii.into();

        Self {
            rect,
            radii,
            ..default()
        }
    }

    pub fn from_vec(points: impl Into<DVec4>, radii: impl Into<DVec4>) -> Self {
        let points: DVec4 = points.into();
        let radii: DVec4 = radii.into();

        Self::new(
            kurbo::Rect::new(points.x, points.y, points.z, points.w),
            kurbo::RoundedRectRadii::new(radii.x, radii.y, radii.z, radii.w),
        )
    }

    pub fn with_rect(mut self, rect: impl Into<kurbo::Rect>) -> Self {
        let rect: kurbo::Rect = rect.into();

        self.rect = rect;
        self
    }

    pub fn with_radii(mut self, radii: impl Into<kurbo::RoundedRectRadii>) -> Self {
        let radii: kurbo::RoundedRectRadii = radii.into();

        self.radii = radii;
        self
    }

    pub fn with_fill(mut self, fill: FillStyle) -> Self {
        self.fill = fill;
        self
    }

    pub fn with_fill_style(mut self, style: peniko::Fill) -> Self {
        self.fill.style = style;
        self
    }

    pub fn with_fill_brush(mut self, brush: impl Into<peniko::Brush>) -> Self {
        self.fill.brush = brush.into();
        self
    }

    pub fn with_stroke(mut self, stroke: StrokeStyle) -> Self {
        self.stroke = stroke;
        self
    }

    pub fn with_stroke_brush(mut self, brush: impl Into<peniko::Brush>) -> Self {
        self.stroke.brush = brush.into();
        self
    }

    fn with_stroke_style(mut self, style: impl Into<kurbo::Stroke>) -> VelloRect {
        self.stroke.style = style.into();
        self
    }

    pub fn percentage_anchor(
        size: impl Into<DVec2>,
        radius: impl Into<DVec4>,
        percentage: impl Into<DVec2>,
    ) -> Self {
        let size: DVec2 = size.into();
        let radius: DVec4 = radius.into();
        let percentage: DVec2 = percentage.into();

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
    pub fn anchor_left(size: impl Into<DVec2>, radius: impl Into<DVec4>) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(1.0, 0.5))
    }

    #[inline]
    pub fn anchor_right(size: impl Into<DVec2>, radius: impl Into<DVec4>) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.0, 0.5))
    }

    #[inline]
    pub fn anchor_bottom(size: impl Into<DVec2>, radius: impl Into<DVec4>) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 0.0))
    }

    #[inline]
    pub fn anchor_top(size: impl Into<DVec2>, radius: impl Into<DVec4>) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 1.0))
    }

    #[inline]
    pub fn anchor_center(size: impl Into<DVec2>, radius: impl Into<DVec4>) -> Self {
        Self::percentage_anchor(size, radius, DVec2::new(0.5, 0.5))
    }
}

impl VelloVector for VelloRect {
    fn build(&self, fragment: &mut VelloFragment) {
        let mut frag: vello::SceneFragment = vello::SceneFragment::new();
        let mut sb: vello::SceneBuilder = vello::SceneBuilder::for_fragment(&mut frag);

        sb.fill(
            self.fill.style,
            kurbo::Affine::default(),
            &self.fill.brush,
            None,
            &kurbo::RoundedRect::from_rect(self.rect, self.radii),
        );

        sb.stroke(
            &self.stroke.style,
            kurbo::Affine::default(),
            &self.stroke.brush,
            None,
            &kurbo::RoundedRect::from_rect(self.rect, self.radii),
        );

        fragment.fragment = frag.into();
    }
}

pub struct VelloRectMotion {
    target_id: Entity,
    vello_rect: VelloRect,
}

impl VelloRectMotion {
    pub fn new(target_id: Entity, vello_rect: VelloRect) -> Self {
        Self {
            target_id,
            vello_rect,
        }
    }

    pub fn inflate(
        &mut self,
        inflation: impl Into<DVec2>,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        let inflation: DVec2 = inflation.into();

        let new_rect: kurbo::Rect = self.vello_rect.rect.inflate(inflation.x, inflation.y);

        let action: Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> = Action::new(
            self.target_id,
            self.vello_rect.clone(),
            self.vello_rect.clone().with_rect(new_rect),
            Self::interp,
        );

        self.vello_rect.rect = new_rect;

        action
    }

    pub fn percentage_expand(
        &mut self,
        expansion: impl Into<DVec2>,
        percentage: impl Into<DVec2>,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        let expansion: DVec2 = expansion.into();
        let percentage: DVec2 = percentage.into();

        let mut new_rect: kurbo::Rect = self.vello_rect.rect;
        new_rect.x0 -= expansion.x * (1.0 - percentage.x);
        new_rect.y0 -= expansion.y * (1.0 - percentage.y);
        new_rect.x1 += expansion.x * percentage.x;
        new_rect.y1 += expansion.y * percentage.y;

        let action: Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> = Action::new(
            self.target_id,
            self.vello_rect.clone(),
            self.vello_rect.clone().with_rect(new_rect),
            Self::interp,
        );

        self.vello_rect.rect = new_rect;

        action
    }

    pub fn expand_left(
        &mut self,
        expansion: f64,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        self.percentage_expand(DVec2::new(expansion, 0.0), DVec2::new(0.0, 0.0))
    }

    pub fn expand_right(
        &mut self,
        expansion: f64,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        self.percentage_expand(DVec2::new(expansion, 0.0), DVec2::new(1.0, 0.0))
    }

    pub fn expand_bottom(
        &mut self,
        expansion: f64,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        self.percentage_expand(DVec2::new(0.0, expansion), DVec2::new(0.0, 0.0))
    }

    pub fn expand_top(
        &mut self,
        expansion: f64,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        self.percentage_expand(DVec2::new(0.0, expansion), DVec2::new(0.0, 1.0))
    }

    pub fn rect_to(
        &mut self,
        rect: impl Into<kurbo::Rect>,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        let rect: kurbo::Rect = rect.into();

        let action: Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> = Action::new(
            self.target_id,
            self.vello_rect.clone(),
            self.vello_rect.clone().with_rect(rect),
            Self::interp,
        );

        self.vello_rect.rect = rect;

        action
    }

    pub fn radii_to(
        &mut self,
        radii: impl Into<kurbo::RoundedRectRadii>,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        let radii: kurbo::RoundedRectRadii = radii.into();

        let action: Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> = Action::new(
            self.target_id,
            self.vello_rect.clone(),
            self.vello_rect.clone().with_radii(radii),
            Self::interp,
        );

        self.vello_rect.radii = radii;

        action
    }

    pub fn fill_brush_to(
        &mut self,
        brush: impl Into<peniko::Brush>,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        let brush: peniko::Brush = brush.into();

        let action: Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> = Action::new(
            self.target_id,
            self.vello_rect.clone(),
            self.vello_rect.clone().with_fill_brush(brush.clone()),
            Self::interp,
        );

        self.vello_rect.fill.brush = brush;

        action
    }

    pub fn stroke_style_to(
        &mut self,
        style: impl Into<kurbo::Stroke>,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        let style: kurbo::Stroke = style.into();

        let action: Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> = Action::new(
            self.target_id,
            self.vello_rect.clone(),
            self.vello_rect.clone().with_stroke_style(style.clone()),
            Self::interp,
        );

        self.vello_rect.stroke.style = style;

        action
    }

    pub fn stroke_brush_to(
        &mut self,
        brush: impl Into<peniko::Brush>,
    ) -> Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> {
        let brush: peniko::Brush = brush.into();

        let action: Action<Handle<VelloFragment>, VelloRect, Assets<VelloFragment>> = Action::new(
            self.target_id,
            self.vello_rect.clone(),
            self.vello_rect.clone().with_stroke_brush(brush.clone()),
            Self::interp,
        );

        self.vello_rect.stroke.brush = brush;

        action
    }

    fn interp(
        fragment: &mut Handle<VelloFragment>,
        begin: &VelloRect,
        end: &VelloRect,
        t: f32,
        fragment_asset: &mut ResMut<Assets<VelloFragment>>,
    ) {
        let Some(fragment) = fragment_asset.get_mut(fragment.id()) else {
            return;
        };

        // Interpolate rect
        let rect: kurbo::Rect = kurbo::Rect {
            x0: f64::lerp(&begin.rect.x0, &end.rect.x0, t),
            y0: f64::lerp(&begin.rect.y0, &end.rect.y0, t),
            x1: f64::lerp(&begin.rect.x1, &end.rect.x1, t),
            y1: f64::lerp(&begin.rect.y1, &end.rect.y1, t),
        };
        // Interpolate radii
        let radii: kurbo::RoundedRectRadii = kurbo::RoundedRectRadii {
            top_left: f64::lerp(&begin.radii.top_left, &end.radii.top_left, t),
            top_right: f64::lerp(&begin.radii.top_right, &end.radii.top_right, t),
            bottom_right: f64::lerp(&begin.radii.bottom_right, &end.radii.bottom_right, t),
            bottom_left: f64::lerp(&begin.radii.bottom_left, &end.radii.bottom_left, t),
        };

        // Interpolate fill & stroke style
        let vello_rect: VelloRect = VelloRect {
            rect,
            radii,
            fill: FillStyle::lerp(&begin.fill, &end.fill, t),
            stroke: StrokeStyle::lerp(&begin.stroke, &end.stroke, t),
        };

        vello_rect.build(fragment);
    }
}
