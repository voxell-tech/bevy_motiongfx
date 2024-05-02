use bevy::{core_pipeline::tonemapping::Tonemapping, pbr::NotShadowCaster, prelude::*};
use bevy_motiongfx::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins(MotionGfxPlugin)
        .add_systems(Startup, (setup, hello_world))
        .add_systems(Update, timeline_movement)
        .run();
}

fn hello_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    const CAPACITY: usize = WIDTH * HEIGHT;

    // Color palette
    let palette = ColorPalette::default();

    let mut cube_ids = Vec::with_capacity(CAPACITY);
    let mut transforms = Vec::with_capacity(CAPACITY);

    // Create cube objects (Entity)
    let material = StandardMaterial {
        base_color: palette.get(ColorKey::Green),
        ..default()
    };

    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let transform = Transform::from_translation(Vec3::new(
                (w as f32) - (WIDTH as f32) * 0.5 - 1.0,
                (h as f32) - (HEIGHT as f32) * 0.5,
                0.0,
            ))
            .with_scale(Vec3::ZERO);

            let cube_id = commands
                .spawn(PbrBundle {
                    transform,
                    mesh: meshes.add(Cuboid::default()),
                    material: materials.add(material.clone()),
                    ..default()
                })
                .insert(NotShadowCaster)
                .id();

            cube_ids.push(cube_id);
            transforms.push(transform);
        }
    }

    // Generate cube animations
    let mut cube_seqs = Vec::with_capacity(CAPACITY);

    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let c = w * WIDTH + h;

            let cube_motion = [
                play!(
                    (commands, cube_ids[c], Transform),
                    from = { transforms[c] }.scale,
                    to = Vec3::splat(0.9),
                    duration = 1.0,
                    ease = ease::circ::ease_in_out,
                ),
                play!(
                    (commands, cube_ids[c], Transform),
                    from = { transforms[c] }.translation.x,
                    to = transforms[c].translation.x + 1.0,
                    duration = 1.0,
                    ease = ease::circ::ease_in_out,
                ),
                play!(
                    (commands, cube_ids[c], Transform),
                    from = { transforms[c] }.rotation,
                    to = Quat::from_euler(EulerRot::XYZ, 0.0, f32::to_radians(90.0), 0.0,),
                    duration = 1.0,
                    ease = ease::circ::ease_in_out,
                ),
            ]
            .all();

            cube_seqs.push(cube_motion);
        }
    }

    let sequence = cube_seqs.flow(0.01);

    commands.spawn(SequencePlayerBundle {
        sequence,
        ..default()
    });
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(-0.5, -0.5, 15.0),
        tonemapping: Tonemapping::AcesFitted,
        ..default()
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn timeline_movement(
    mut q_timelines: Query<(&mut SequencePlayer, &mut SequenceController)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut sequence_player, mut sequence_time) in q_timelines.iter_mut() {
        if keys.pressed(KeyCode::KeyD) {
            sequence_time.target_time += time.delta_seconds();
        }

        if keys.pressed(KeyCode::KeyA) {
            sequence_time.target_time -= time.delta_seconds();
        }

        if keys.just_pressed(KeyCode::Space) {
            if keys.pressed(KeyCode::ShiftLeft) {
                sequence_player.time_scale = -1.0;
            } else {
                sequence_player.time_scale = 1.0;
            }
        }

        if keys.just_pressed(KeyCode::Escape) {
            sequence_player.time_scale = 0.0;
        }
    }
}
