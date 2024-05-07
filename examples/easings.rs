use bevy::{core_pipeline::bloom::BloomSettings, pbr::NotShadowCaster, prelude::*};
use bevy_motiongfx::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins(MotionGfxPlugin)
        .add_systems(Startup, (setup, easings))
        .add_systems(Update, timeline_movement)
        .run();
}

fn easings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    const CAPACITY: usize = 10;

    // Color palette
    let palette = ColorPalette::default();

    let mut sphere_ids = Vec::with_capacity(CAPACITY);
    let mut transforms = Vec::with_capacity(CAPACITY);
    let mut materials = Vec::with_capacity(CAPACITY);

    // Create sphere objects (Entity)
    let material = StandardMaterial {
        base_color: Color::WHITE,
        emissive: palette.get(ColorKey::Blue) * 100.0,
        ..default()
    };

    for i in 0..CAPACITY {
        let transform =
            Transform::from_translation(Vec3::new(-5.0, (i as f32) - (CAPACITY as f32) * 0.5, 0.0))
                .with_scale(Vec3::ONE);

        let sphere = commands
            .spawn(PbrBundle {
                transform,
                mesh: meshes.add(Sphere::default()),
                material: material_assets.add(material.clone()),
                ..default()
            })
            .insert(NotShadowCaster)
            .id();

        sphere_ids.push(sphere);
        transforms.push(transform);
        materials.push(material.clone());
    }

    // Generate sequence
    let easings = [
        ease::linear,
        ease::sine::ease_in_out,
        ease::quad::ease_in_out,
        ease::cubic::ease_in_out,
        ease::quart::ease_in_out,
        ease::quint::ease_in_out,
        ease::expo::ease_in_out,
        ease::circ::ease_in_out,
        ease::back::ease_in_out,
        ease::elastic::ease_in_out,
    ];

    let easing_seqs: Vec<Sequence> = transforms
        .iter_mut()
        .zip(materials.iter_mut())
        .enumerate()
        .map(|(i, (t, m))| {
            play!(
                commands,
                act!(
                    (sphere_ids[i], Transform),
                    start = { t }.translation.x,
                    end = t.translation.x + 10.0,
                )
                .with_ease(easings[i])
                .animate(1.0),
                act!(
                    (sphere_ids[i], StandardMaterial),
                    start = { m }.emissive,
                    end = palette.get(ColorKey::Red) * 100.0,
                )
                .with_ease(easings[i])
                .animate(1.0),
            )
            .all()
        })
        .collect();

    let sequence = easing_seqs.chain();

    commands.spawn(SequencePlayerBundle {
        sequence,
        ..default()
    });
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 15.0),
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
            ..default()
        })
        .insert(BloomSettings::default());
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
