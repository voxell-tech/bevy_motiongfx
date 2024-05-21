use bevy::prelude::*;
use bevy_vello::prelude::*;

use crate::brush::Brush;

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
