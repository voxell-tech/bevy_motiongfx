use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use motiongfx_typst::{TypstCompiler, TypstCompilerPlugin};

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins((
            MotionGfx,
            MotionGfxBevy,
            MotionGfxVello,
            TypstCompilerPlugin::new(Vec::new()),
        ))
        .add_systems(Startup, (setup, typst_basic))
        .add_systems(Update, test_movement)
        // .add_systems(Update, timeline_movement_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn typst_basic(
    mut commands: Commands,
    mut fragment_assets: ResMut<Assets<VelloFragment>>,
    mut typst_compiler: ResMut<TypstCompiler>,
) {
    let content: String = String::from(
        r###"
        #set page(width: 1120pt, margin: 8pt)
        #set raw(theme: "themes/Monokai Pro.tmTheme")
        #set text(size: 36pt, font: "consolas", fill: rgb("#fcfcfa"))

        #show raw: set text(font: "Consolas")

        Hello World!

        Test line 2.

        $ frac(a^2, 2) $
        $ vec(1, 2, delim: "[") $
        $ mat(1, 2; 3, 4) $
        $ lim_x =
            op("lim", limits: #true)_x $

        ```rs
        match typst_compiler.compile(content) {
            Ok(fragment) => {
                commands.spawn(VelloFragmentBundle {
                    fragment: fragment_assets.add(fragment),
                    transform: TransformBundle {
                        local: Transform::from_xyz(-500.0, 500.0, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
            Err(err) => println!("{:#?}", err),
        };
        ```

        = Introduction
        + First element.
        + Second element.
        "###,
    );

    match typst_compiler.compile_flatten(&mut commands, &mut fragment_assets, content) {
        Ok(ids) => {}
        Err(err) => {
            println!("{:#?}", err);
        }
    }
    // match typst_compiler.compile(&mut commands, &mut fragment_assets, content) {
    //     Ok(id) => {
    //         commands
    //             .entity(id)
    //             .insert(Transform::from_xyz(-500.0, 600.0, 0.0));
    //     }
    //     Err(err) => {
    //         println!("{:#?}", err);
    //     }
    // }
}

fn test_movement(mut q_paths: Query<&Transform, With<Handle<VelloFragment>>>) {}
