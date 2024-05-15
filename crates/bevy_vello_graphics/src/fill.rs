use bevy::prelude::*;
use bevy_vello_renderer::vello::peniko;

use crate::brush::Brush;

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
