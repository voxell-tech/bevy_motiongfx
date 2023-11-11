use bevy_ecs::prelude::*;
use bevy_math::{DVec2, DVec4};
use bevy_vello_renderer::vello::kurbo;
use motiongfx_core::prelude::*;

#[derive(Component)]
pub struct VelloRect {
    rounded_rect: kurbo::RoundedRect,
}

impl VelloRect {
    #[inline]
    pub fn new(rounded_rect: kurbo::RoundedRect) -> Self {
        Self { rounded_rect }
    }

    pub fn from_vec(points: impl Into<DVec4>, radii: impl Into<DVec4>) -> Self {
        let points: DVec4 = points.into();
        let radii: DVec4 = radii.into();

        Self {
            rounded_rect: kurbo::RoundedRect::new(
                points.x,
                points.y,
                points.z,
                points.w,
                (radii.x, radii.y, radii.z, radii.w),
            ),
        }
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

    pub fn percentage_anchor(
        size: impl Into<DVec2>,
        radius: impl Into<DVec4>,
        anchor: impl Into<DVec2>,
    ) -> Self {
        let size: DVec2 = size.into();
        let radius: DVec4 = radius.into();
        let anchor: DVec2 = anchor.into();

        Self {
            rounded_rect: kurbo::RoundedRect::new(
                -size.x * anchor.x,
                -size.y * anchor.y,
                size.x * (1.0 - anchor.x),
                size.y * (1.0 - anchor.y),
                (radius.x, radius.y, radius.z, radius.w),
            ),
        }
    }
}

pub struct VelloRectMotion {
    target_id: Entity,
    rounded_rect: kurbo::RoundedRect,
}

impl VelloRectMotion {
    pub fn new(target_id: Entity, vello_rect: VelloRect) -> Self {
        Self {
            target_id,
            rounded_rect: vello_rect.rounded_rect,
        }
    }

    pub fn expand_left(
        &mut self,
        expansion: f64,
    ) -> Action<VelloRect, kurbo::RoundedRect, EmptyRes> {
        let mut new_rect: kurbo::Rect = self.rounded_rect.rect();
        new_rect.x0 -= expansion;

        let new_rounded_rect: kurbo::RoundedRect =
            kurbo::RoundedRect::from_rect(new_rect, self.rounded_rect.radii());

        let action: Action<VelloRect, kurbo::RoundedRect, EmptyRes> = Action::new(
            self.target_id,
            self.rounded_rect,
            new_rounded_rect,
            Self::interp,
        );

        self.rounded_rect = new_rounded_rect;

        action
    }

    pub fn expand_right(
        &mut self,
        expansion: f64,
    ) -> Action<VelloRect, kurbo::RoundedRect, EmptyRes> {
        let mut new_rect: kurbo::Rect = self.rounded_rect.rect();
        new_rect.x1 += expansion;

        let new_rounded_rect: kurbo::RoundedRect =
            kurbo::RoundedRect::from_rect(new_rect, self.rounded_rect.radii());

        let action: Action<VelloRect, kurbo::RoundedRect, EmptyRes> = Action::new(
            self.target_id,
            self.rounded_rect,
            new_rounded_rect,
            Self::interp,
        );

        self.rounded_rect = new_rounded_rect;

        action
    }

    pub fn expand_bottom(
        &mut self,
        expansion: f64,
    ) -> Action<VelloRect, kurbo::RoundedRect, EmptyRes> {
        let mut new_rect: kurbo::Rect = self.rounded_rect.rect();
        new_rect.y0 -= expansion;

        let new_rounded_rect: kurbo::RoundedRect =
            kurbo::RoundedRect::from_rect(new_rect, self.rounded_rect.radii());

        let action: Action<VelloRect, kurbo::RoundedRect, EmptyRes> = Action::new(
            self.target_id,
            self.rounded_rect,
            new_rounded_rect,
            Self::interp,
        );

        self.rounded_rect = new_rounded_rect;

        action
    }

    pub fn expand_top(
        &mut self,
        expansion: f64,
    ) -> Action<VelloRect, kurbo::RoundedRect, EmptyRes> {
        let mut new_rect: kurbo::Rect = self.rounded_rect.rect();
        new_rect.y1 += expansion;

        let new_rounded_rect: kurbo::RoundedRect =
            kurbo::RoundedRect::from_rect(new_rect, self.rounded_rect.radii());

        let action: Action<VelloRect, kurbo::RoundedRect, EmptyRes> = Action::new(
            self.target_id,
            self.rounded_rect,
            new_rounded_rect,
            Self::interp,
        );

        self.rounded_rect = new_rounded_rect;

        action
    }

    pub fn radii_to(
        &mut self,
        radii: impl Into<kurbo::RoundedRectRadii>,
    ) -> Action<VelloRect, kurbo::RoundedRect, EmptyRes> {
        let new_rounded_rect: kurbo::RoundedRect =
            kurbo::RoundedRect::from_rect(self.rounded_rect.rect(), radii);

        let action: Action<VelloRect, kurbo::RoundedRect, EmptyRes> = Action::new(
            self.target_id,
            self.rounded_rect,
            new_rounded_rect,
            Self::interp,
        );

        self.rounded_rect = new_rounded_rect;

        action
    }

    fn interp(
        vello_rect: &mut VelloRect,
        begin: &kurbo::RoundedRect,
        end: &kurbo::RoundedRect,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        let begin_rect: kurbo::Rect = begin.rect();
        let end_rect: kurbo::Rect = end.rect();

        let begin_radii: kurbo::RoundedRectRadii = begin.radii();
        let end_radii: kurbo::RoundedRectRadii = end.radii();

        vello_rect.rounded_rect = kurbo::RoundedRect::from_rect(
            kurbo::Rect {
                x0: f64::lerp(&begin_rect.x0, &end_rect.x0, t),
                y0: f64::lerp(&begin_rect.y0, &end_rect.y0, t),
                x1: f64::lerp(&begin_rect.x1, &end_rect.x1, t),
                y1: f64::lerp(&begin_rect.y1, &end_rect.y1, t),
            },
            kurbo::RoundedRectRadii {
                top_left: f64::lerp(&begin_radii.top_left, &end_radii.top_left, t),
                top_right: f64::lerp(&begin_radii.top_right, &end_radii.top_right, t),
                bottom_right: f64::lerp(&begin_radii.bottom_right, &end_radii.bottom_right, t),
                bottom_left: f64::lerp(&begin_radii.bottom_left, &end_radii.bottom_left, t),
            },
        );
    }
}
