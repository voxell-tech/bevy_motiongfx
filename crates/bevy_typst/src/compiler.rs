use std::path::PathBuf;

use bevy::prelude::*;
use typst::{diag::SourceResult, eval::Tracer, model::Document};

use world::TypstWorld;

pub mod fonts;
pub mod world;

mod download;
mod package;

#[derive(Default)]
pub struct TypstCompilerPlugin {
    font_paths: Vec<PathBuf>,
}

impl TypstCompilerPlugin {
    pub fn new(font_paths: Vec<PathBuf>) -> Self {
        Self { font_paths }
    }
}

impl Plugin for TypstCompilerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TypstCompiler::new(&self.font_paths));
    }
}

/// A resource compiler for compiling Typst content.
#[derive(Resource)]
pub struct TypstCompiler {
    world: TypstWorld,
    tracer: Tracer,
}

impl TypstCompiler {
    pub fn new(font_paths: &[PathBuf]) -> Self {
        let mut assets = PathBuf::from(".");
        assets.push("assets");
        Self {
            world: TypstWorld::new(assets, font_paths).unwrap(),
            tracer: Tracer::new(),
        }
    }

    /// Compile a raw Typst string into a document.
    pub fn compile(&mut self, text: &str) -> SourceResult<Document> {
        let source = self.world.get_main_source_mut();
        source.replace(text);
        typst::compile(&self.world, &mut self.tracer)
    }
}
