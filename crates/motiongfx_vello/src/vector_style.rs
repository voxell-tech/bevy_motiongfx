use bevy_vello_renderer::vello::{kurbo, peniko};

#[derive(Clone)]
pub struct FillStyle {
    pub style: peniko::Fill,
    pub brush: peniko::Brush,
}

impl FillStyle {
    pub fn from_brush(brush: impl Into<peniko::Brush>) -> Self {
        Self::default().with_brush(brush)
    }

    #[inline]
    pub fn with_style(mut self, style: peniko::Fill) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_brush(mut self, brush: impl Into<peniko::Brush>) -> Self {
        self.brush = brush.into();
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
