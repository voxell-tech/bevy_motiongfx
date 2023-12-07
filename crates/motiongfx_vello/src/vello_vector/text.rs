use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_utils::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{
        self,
        fello::{
            self,
            meta::{charmap::Charmap, metrics::Metrics, variations::axis::Axes},
            raw::{types::F2Dot14, FontRef},
            MetadataProvider,
        },
        glyph::Glyph,
        peniko,
    },
};

use crate::{
    fill_style::FillStyle,
    prelude::StrokeStyle,
    vello_vector::{VelloBuilder, VelloVector},
};

const CONSOLAS_FONT: &[u8] = include_bytes!("../../fonts/consola.ttf");

#[derive(Bundle, Clone, Default)]
pub struct VelloTextSimpleBundle {
    pub text: VelloTextSimple,
    pub fill: FillStyle,
    pub stroke: StrokeStyle,
    pub fragment_bundle: VelloFragmentBundle,
}

#[derive(Component, Clone)]
pub struct VelloTextSimple {
    pub(crate) font: peniko::Font,
    pub(crate) font_size: f32,
    pub(crate) variations: &'static [(&'static str, f32)],
    pub(crate) content: String,
    built: bool,
}

impl VelloTextSimple {
    pub fn new(font: peniko::Font, content: impl Into<String>) -> Self {
        Self {
            font,
            content: content.into(),
            ..default()
        }
    }

    pub fn from_content(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..default()
        }
    }

    pub fn with_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn with_variations(mut self, variataions: &'static [(&'static str, f32)]) -> Self {
        self.variations = variataions;
        self
    }
}

impl Default for VelloTextSimple {
    fn default() -> Self {
        Self {
            font: peniko::Font::new(peniko::Blob::new(Arc::new(CONSOLAS_FONT)), 0),
            font_size: 24.0,
            variations: &[],
            content: "".into(),
            built: false,
        }
    }
}

impl VelloVector for VelloTextSimple {
    fn build_fill(&self, fill: &FillStyle, builder: &mut vello::SceneBuilder) {
        let Some(font_ref) = to_font_ref(&self.font) else {
            return;
        };

        let axes: Axes = font_ref.axes();
        let fello_size: fello::Size = fello::Size::new(self.font_size);

        let coords: Vec<F2Dot14> = axes.normalize(self.variations.iter().copied()).collect();

        let charmap: Charmap = font_ref.charmap();
        let metrics: Metrics = font_ref.metrics(fello_size, coords.as_slice().into());

        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font_ref.glyph_metrics(fello_size, coords.as_slice().into());

        let mut pen_x: f32 = 0.0;
        let mut pen_y: f32 = 0.0;

        let glyph_builder = builder
            .draw_glyphs(&self.font)
            .font_size(self.font_size)
            .brush(&fill.brush);

        glyph_builder.draw(
            fill.style,
            self.content.chars().filter_map(|ch| {
                if ch == '\n' {
                    pen_y += line_height;
                    pen_x = 0.0;
                    return None;
                }

                let gid = charmap.map(ch).unwrap_or_default();
                let advance = glyph_metrics.advance_width(gid).unwrap_or_default();

                let x = pen_x;
                pen_x += advance;

                Some(Glyph {
                    id: gid.to_u16() as u32,
                    x,
                    y: pen_y,
                })
            }),
        );
    }

    fn build_stroke(&self, stroke: &StrokeStyle, builder: &mut vello::SceneBuilder) {
        let Some(font_ref) = to_font_ref(&self.font) else {
            return;
        };

        let axes: Axes = font_ref.axes();
        let fello_size: fello::Size = fello::Size::new(self.font_size);

        let coords: Vec<F2Dot14> = axes.normalize(self.variations.iter().copied()).collect();

        let charmap: Charmap = font_ref.charmap();
        let metrics: Metrics = font_ref.metrics(fello_size, coords.as_slice().into());

        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font_ref.glyph_metrics(fello_size, coords.as_slice().into());

        let mut pen_x: f32 = 0.0;
        let mut pen_y: f32 = 0.0;

        let glyph_builder: vello::DrawGlyphs = builder
            .draw_glyphs(&self.font)
            .font_size(self.font_size)
            .brush(&stroke.brush);

        glyph_builder.draw(
            &stroke.style,
            self.content.chars().filter_map(|ch| {
                if ch == '\n' {
                    pen_y += line_height;
                    pen_x = 0.0;
                    return None;
                }

                let gid = charmap.map(ch).unwrap_or_default();
                let advance = glyph_metrics.advance_width(gid).unwrap_or_default();

                let x = pen_x;
                pen_x += advance;

                Some(Glyph {
                    id: gid.to_u16() as u32,
                    x,
                    y: pen_y,
                })
            }),
        );
    }
}

impl VelloBuilder for VelloTextSimple {
    #[inline]
    fn is_built(&self) -> bool {
        self.built
    }

    #[inline]
    fn set_built(&mut self, built: bool) {
        self.built = built;
    }
}

fn to_font_ref(font: &peniko::Font) -> Option<FontRef> {
    use vello::fello::raw::FileRef;

    let file_ref = FileRef::new(font.data.as_ref()).ok()?;

    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}
