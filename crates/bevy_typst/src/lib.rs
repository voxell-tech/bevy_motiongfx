use std::path::PathBuf;

use bevy::prelude::*;
use bevy_vello::vello_svg::usvg;
use ecow::EcoVec;
use typst::{diag::SourceDiagnostic, eval::Tracer, layout::Abs};

use crate::world::TypstWorld;

pub mod world;

pub mod prelude {
    pub use crate::{world::TypstWorld, TypstCompiler, TypstCompilerPlugin};
}

mod download;
mod fonts;
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

/// A compiler for compiling Typst content.
///
/// This compiler can be accessed from the resource:
/// ```
/// use bevy::prelude::*;
/// use motiongfx_vello::prelude::*;
/// use motiongfx_typst::TypstCompiler;
///
/// pub fn compile(
///     mut commands: Commands,
///     mut typst_compiler: ResMut<TypstCompiler>,
///     mut scenes: ResMut<Assets<VelloScene>>,
/// ) {
///     let content: String = String::from(
///         r###"
///         = Introduction
///         + First element.
///         + Second element.
///         "###,
///     );
///
///     match typst_compiler.compile_flatten(&mut commands, &mut scenes, content) {
///         Ok(tree) => {
///             println!("{:#?}", tree.size);
///         }
///         Err(_) => todo!(),
///     }
/// }
/// ```
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

    // TODO: take a look at typst_ide for getting FrameItem to svg output relation
    pub fn compile_text(&mut self, text: String) -> Result<usvg::Tree, EcoVec<SourceDiagnostic>> {
        self.world.set_source(text);
        let document = typst::compile(&self.world, &mut self.tracer)?;

        let svg = typst_svg::svg_merged(&document, Abs::zero());

        // Svg string should not have any issue if compilation succeeded
        Ok(usvg::Tree::from_str(
            &svg,
            &usvg::Options::default(),
            &usvg::fontdb::Database::new(),
        )
        .unwrap())
    }
}