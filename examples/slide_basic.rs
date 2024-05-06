use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    },
    pbr::{NotShadowCaster, ScreenSpaceAmbientOcclusionBundle},
    prelude::*,
};
use bevy_motiongfx::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins((DefaultPlugins, TemporalAntiAliasPlugin))
        .insert_resource(Msaa::Off)
        // Custom plugins
        .add_plugins(MotionGfxPlugin)
        .add_systems(Startup, (setup, slide_basic))
        .add_systems(Update, slide_movement)
        .run();
}

fn slide_basic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Color palette
    let palette = ColorPalette::default();

    // Materials
    let mut cube_material = StandardMaterial {
        base_color: palette.get(ColorKey::Green),
        ..default()
    };
    let sphere_material = StandardMaterial {
        base_color: palette.get(ColorKey::Blue),
        ..default()
    };

    let x_offset = 2.0;
    // Cube
    let mut cube_pbr = PbrBundle {
        transform: Transform::default().with_scale(Vec3::splat(0.0)),
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(cube_material.clone()),
        ..default()
    };
    let cube_id = commands
        .spawn(cube_pbr.clone())
        .insert(NotShadowCaster)
        .id();

    // Sphere
    let mut sphere_pbr = PbrBundle {
        transform: Transform::default()
            .with_translation(Vec3::X * x_offset)
            .with_scale(Vec3::splat(0.0)),
        mesh: meshes.add(Sphere::default()),
        material: materials.add(sphere_material),
        ..default()
    };
    let sphere_id = commands
        .spawn(sphere_pbr.clone())
        .insert(NotShadowCaster)
        .id();

    // Create slides
    let slide0 = play!(
        (commands, cube_id, Transform),
        start = { cube_pbr.transform }.scale,
        end = Vec3::ONE,
        duration = 1.0,
    );

    let slide1 = [
        [
            play!(
                (commands, cube_id, Transform),
                start = { cube_pbr.transform }.translation.x,
                end = -x_offset,
                duration = 1.0,
            ),
            play!(
                (commands, cube_id, StandardMaterial),
                start = { cube_material }.base_color,
                end = palette.get(ColorKey::Base0),
                duration = 1.0,
            ),
        ]
        .all(),
        play!(
            (commands, sphere_id, Transform),
            start = { sphere_pbr.transform }.scale,
            end = Vec3::ONE,
            duration = 1.0,
        ),
    ]
    .flow(0.1);

    commands.spawn(create_slide(vec![slide0, slide1]));
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
        .insert(BloomSettings::default())
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
        .insert(TemporalAntiAliasBundle::default());

    // Directional light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn slide_movement(mut q_slides: Query<&mut SlideController>, keys: Res<ButtonInput<KeyCode>>) {
    for mut slide in q_slides.iter_mut() {
        if keys.just_pressed(KeyCode::Space) {
            slide.set_time_scale(1.0);

            if keys.pressed(KeyCode::ShiftLeft) {
                slide.prev();
            } else {
                slide.next();
            }
        }

        if keys.just_pressed(KeyCode::Escape) {
            slide.set_time_scale(0.0);
        }
    }
}
