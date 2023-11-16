use bevy_render::prelude::*;
use bevy_utils::prelude::*;
use bevy_vello_renderer::vello::{kurbo, peniko};
use motiongfx_core::prelude::*;

pub struct PenikoBrush(peniko::Brush);
pub struct PenikoColor(peniko::Color);

impl From<Color> for PenikoColor {
    fn from(value: Color) -> Self {
        PenikoColor(peniko::Color::rgba(
            value.r() as f64,
            value.g() as f64,
            value.b() as f64,
            value.a() as f64,
        ))
    }
}

impl From<Color> for PenikoBrush {
    fn from(value: Color) -> Self {
        PenikoBrush(peniko::Brush::Solid(PenikoColor::from(value).0))
    }
}

#[derive(Clone)]
pub struct FillStyle {
    pub style: peniko::Fill,
    pub brush: peniko::Brush,
}

impl FillStyle {
    pub fn from_brush(brush: impl Into<PenikoBrush>) -> Self {
        Self::default().with_brush(brush)
    }

    #[inline]
    pub fn with_style(mut self, style: peniko::Fill) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_brush(mut self, brush: impl Into<PenikoBrush>) -> Self {
        self.brush = brush.into().0;
        self
    }
}

impl Default for FillStyle {
    fn default() -> Self {
        Self {
            style: peniko::Fill::NonZero,
            brush: peniko::Brush::Solid(peniko::Color::WHITE_SMOKE),
        }
    }
}

impl Lerp<f32> for FillStyle {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        FillStyle {
            // Style cannot be interpolated
            style: self.style,
            brush: peniko::Brush::lerp(&self.brush, &other.brush, t),
        }
    }
}

impl From<Color> for FillStyle {
    fn from(value: Color) -> Self {
        Self {
            brush: peniko::Brush::Solid(peniko::Color::rgba(
                value.r() as f64,
                value.g() as f64,
                value.b() as f64,
                value.a() as f64,
            )),
            ..default()
        }
    }
}

#[derive(Clone)]
pub struct StrokeStyle {
    pub style: kurbo::Stroke,
    pub brush: peniko::Brush,
}

impl StrokeStyle {
    pub fn from_brush(brush: impl Into<peniko::Brush>) -> Self {
        Self::default().with_brush(brush)
    }

    #[inline]
    pub fn with_style(mut self, style: kurbo::Stroke) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_brush(mut self, brush: impl Into<peniko::Brush>) -> Self {
        self.brush = brush.into();
        self
    }
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            style: kurbo::Stroke::default(),
            brush: peniko::Brush::Solid(peniko::Color::WHITE_SMOKE),
        }
    }
}

impl Lerp<f32> for StrokeStyle {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        StrokeStyle {
            style: kurbo::Stroke::lerp(&self.style, &other.style, t),
            brush: peniko::Brush::lerp(&self.brush, &other.brush, t),
        }
    }
}
