use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    },
    log::LogPlugin,
    pbr::ScreenSpaceAmbientOcclusionBundle,
    prelude::*,
    render::view::ViewTarget,
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
    let mut cubes: Vec<Entity> = Vec::with_capacity(10);
    let mut cube_translations: Vec<Translation> = Vec::with_capacity(10);
    let mut cube_scales: Vec<Scale> = Vec::with_capacity(10);
    let mut cube_colors: Vec<BaseColor> = Vec::with_capacity(10);
    let mut cube_actions: Vec<ActionMetaGroup> = Vec::with_capacity(10);

    // Create cube objects (Entity)
    let green_color: Color = Color::rgb(0.3, 0.5, 0.3);

    for c in 0..10 {
        let cube = commands
            .spawn(PbrBundle {
                transform: Transform::from_scale(Vec3::ONE * 0.2).with_translation(Vec3::new(
                    -1.0,
                    (c as f32) * 0.21 - 0.5,
                    0.0,
                )),
                mesh: meshes.add(shape::Cube::default().into()),
                material: materials.add(green_color.into()),
                ..default()
            })
            .id();
        cubes.push(cube);
    }

    let mut act: ActionBuilder = ActionBuilder::new(&mut commands);

    // Initialize translation
    for c in 0..10 {
        let id: Entity = cubes[c];
        cube_translations.push(Translation::new(
            id,
            Vec3::new(-1.0, (c as f32) * 0.21 - 0.5, 0.0),
        ));
        cube_scales.push(Scale::new(id, Vec3::ONE * 0.2));
        cube_colors.push(BaseColor::new(id, green_color.into()));
    }

    // Generate cube animations
    for c in 0..10 {
        cube_actions.push(
            all(&[
                act.play(cube_translations[c].translate(Vec3::X), 1.0),
                act.play(cube_scales[c].scale_all(0.5), 1.0),
                act.play(cube_colors[c].color_to(Color::WHITE.into()), 1.0),
            ])
            .with_ease(ease::quart::ease_in_out),
        );
    }
    for c in 0..10 {
        cube_actions.push(
            all(&[
                act.play(cube_translations[c].translate(-Vec3::X), 1.0),
                act.play(cube_scales[c].scale_all(2.0), 1.0),
                act.play(cube_colors[c].color_to(green_color.into()), 1.0),
            ])
            .with_ease(ease::circ::ease_in_out),
        );
    }

    let action_grp: ActionMetaGroup = flow(0.1, &cube_actions);

    sequence.play(action_grp);
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
            ..default()
        })
        .insert(BloomSettings::default())
        .insert(FogSettings {
            color: Color::rgba(0.05, 0.05, 0.05, 1.0),
            falloff: FogFalloff::Linear {
                start: 5.0,
                end: 20.0,
            },
            ..default()
        })
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
