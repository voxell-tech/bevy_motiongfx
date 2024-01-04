use std::path::PathBuf;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello_svg::usvg::{self, TreeParsing},
};
use ecow::EcoVec;
use motiongfx_vello::svg;
use typst::{diag::SourceDiagnostic, eval::Tracer, layout::Abs, model::Document};

use crate::world::TypstWorld;

mod download;
mod fonts;
mod package;
mod world;

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

#[derive(Resource)]
pub struct TypstCompiler {
    world: TypstWorld,
    tracer: Tracer,
}

impl TypstCompiler {
    pub fn new(font_paths: &[PathBuf]) -> Self {
        Self {
            world: TypstWorld::new(PathBuf::from(".\\assets"), font_paths).unwrap(),
            tracer: Tracer::new(),
        }
    }

    pub fn compile(
        &mut self,
        commands: &mut Commands,
        fragment_assets: &mut ResMut<Assets<VelloFragment>>,
        text: String,
    ) -> Result<Entity, EcoVec<SourceDiagnostic>> {
        self.world.set_source(text);
        let document: Document = typst::compile(&self.world, &mut self.tracer)?;

        let svg: String = typst_svg::svg_merged(&document.pages, Abs::zero());

        // Svg string should not have any issue if compilation succeeded
        let tree: usvg::Tree = usvg::Tree::from_str(&svg, &usvg::Options::default()).unwrap();

        Ok(svg::spawn_tree(commands, fragment_assets, &tree))
        // vello_svg::render_tree(&mut builder, &tree);

        // Ok(VelloFragment {
        //     fragment: Arc::new(fragment),
        // })
    }
}
