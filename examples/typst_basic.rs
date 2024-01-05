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
        .add_systems(Update, timeline_movement_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn typst_basic(
    mut commands: Commands,
    mut sequence: ResMut<Sequence>,
    mut typst_compiler: ResMut<TypstCompiler>,
    mut fragment_assets: ResMut<Assets<VelloFragment>>,
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
        fn main() {
            println!("Hello from Typst!");
        }
        ```

        = Introduction
        + First element.
        + Second element.
        "###,
    );

    match typst_compiler.compile_flatten(&mut commands, &mut fragment_assets, content) {
        Ok((root_entity, svg_path_bundles)) => {
            commands
                .entity(root_entity)
                .insert(Transform::from_xyz(-600.0, 600.0, 0.0));

            // Motion
            let mut transform_motions: Vec<TransformMotion> = svg_path_bundles
                .iter()
                .map(|bundle| TransformMotion::new(bundle.entity, bundle.transform))
                .collect();

            // Actions
            let mut act: ActionBuilder = ActionBuilder::new(&mut commands);

            let transform_actions: Vec<ActionMetaGroup> = transform_motions
                .iter_mut()
                .map(|motion| act.play(motion.translate_add(Vec3::Y * 50.0), 1.0))
                .collect();

            sequence.play(flow(0.1, &transform_actions).with_ease(ease::expo::ease_in_out));
        }
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

fn timeline_movement_system(
    mut timeline: ResMut<Timeline>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    println!("{:?}", time.elapsed());
    if keys.pressed(KeyCode::D) {
        timeline.target_time += time.delta_seconds();
    }

    if keys.pressed(KeyCode::A) {
        timeline.target_time -= time.delta_seconds();
    }

    if keys.pressed(KeyCode::Space) && keys.pressed(KeyCode::ShiftLeft) {
        timeline.time_scale = -1.0;
        timeline.is_playing = true;
    } else if keys.pressed(KeyCode::Space) {
        timeline.time_scale = 1.0;
        timeline.is_playing = true;
    }
}
