use bevy_render::prelude::*;
use bevy_vello_renderer::vello::{kurbo, peniko};

pub struct PenikoBrush(pub peniko::Brush);
pub struct PenikoColor(pub peniko::Color);

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
