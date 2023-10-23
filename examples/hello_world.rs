use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    },
    log::LogPlugin,
    pbr::{NotShadowCaster, ScreenSpaceAmbientOcclusionBundle},
    prelude::*,
};
use bevy_motiongfx::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins((
            DefaultPlugins.build().disable::<LogPlugin>(),
            TemporalAntiAliasPlugin,
        ))
        // Custom plugins
        .add_plugins(MotionGfx)
        .add_systems(Startup, (setup, hello_world))
        .add_systems(Update, timeline_movement_system)
        .run();
}

pub fn hello_world(
    mut commands: Commands,
    mut sequence: ResMut<Sequence>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    const CAPACITY: usize = WIDTH * HEIGHT;

    let mut cubes: Vec<Entity> = Vec::with_capacity(CAPACITY);
    // States
    let mut cube_translations: Vec<Translation> = Vec::with_capacity(CAPACITY);
    let mut cube_scales: Vec<Scale> = Vec::with_capacity(CAPACITY);
    let mut cube_rotations: Vec<Rotation> = Vec::with_capacity(CAPACITY);
    // Actions
    let mut cube_actions: Vec<ActionMetaGroup> = Vec::with_capacity(CAPACITY);

    // Create cube objects (Entity)
    let material: StandardMaterial = StandardMaterial {
        base_color: style::GREEN.into(),
        ..default()
    };

    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let transform: Transform = Transform::from_translation(Vec3::new(
                (w as f32) - (WIDTH as f32) * 0.5 - 1.0,
                (h as f32) - (HEIGHT as f32) * 0.5,
                0.0,
            ))
            .with_scale(Vec3::ZERO);

            let cube = commands
                .spawn(PbrBundle {
                    transform,
                    mesh: meshes.add(shape::Cube::default().into()),
                    material: materials.add(material.clone()),
                    ..default()
                })
                .insert(NotShadowCaster)
                .id();

            cube_translations.push(Translation::from_transform(cube, &transform));
            cube_scales.push(Scale::from_transform(cube, &transform));
            cube_rotations.push(Rotation::from_transform(cube, &transform));

            cubes.push(cube);
        }
    }

    let mut act: ActionBuilder = ActionBuilder::new(&mut commands);

    // Generate cube animations
    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let c = w * WIDTH + h;

            cube_actions.push(
                all(&[
                    act.play(cube_translations[c].translate(Vec3::X), 1.0),
                    act.play(cube_scales[c].scale_all_to(0.9), 1.0),
                    act.play(
                        cube_rotations[c].rotate_to(Quat::from_euler(
                            EulerRot::XYZ,
                            0.0,
                            f32::to_radians(90.0),
                            0.0,
                        )),
                        1.0,
                    ),
                ])
                .with_ease(ease::circ::ease_in_out),
            );
        }
    }

    let action_grp: ActionMetaGroup = flow(0.01, &cube_actions);

    sequence.play(action_grp);
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(-0.5, -0.5, 15.0),
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
            ..default()
        })
        .insert(BloomSettings::default())
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
        .insert(TemporalAntiAliasBundle::default());

    // Directional light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn timeline_movement_system(
    mut timeline: ResMut<Timeline>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
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
