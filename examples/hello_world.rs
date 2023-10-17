use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    },
    log::LogPlugin,
    pbr::ScreenSpaceAmbientOcclusionBundle,
    prelude::*,
};
use bevy_motiongfx::{action::Action, sequence::Sequence, timeline::Timeline, MotionGfx};

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
        .add_systems(Update, timeline_movemen_system)
        .run();
}

fn translate_action(target_id: Entity, begin: Vec3, end: Vec3) -> Action<Transform, Vec3> {
    Action::new(target_id, begin, end, translate_interp)
}

fn translate_interp(transform: &mut Transform, begin: &Vec3, end: &Vec3, t: f32) {
    transform.translation = Vec3::lerp(*begin, *end, t);
}

fn scale_action(target_id: Entity, begin: Vec3, end: Vec3) -> Action<Transform, Vec3> {
    Action::new(target_id, begin, end, scale_interp)
}

fn scale_interp(transform: &mut Transform, begin: &Vec3, end: &Vec3, t: f32) {
    transform.scale = Vec3::lerp(*begin, *end, t);
}

pub fn hello_world(
    mut commands: Commands,
    mut sequence: ResMut<Sequence>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let target_id: Entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .id();

    sequence
        .all(&mut commands)
        .add_action(
            translate_action(target_id, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)),
            1.0,
        )
        .add_action(scale_action(target_id, Vec3::ONE, Vec3::ONE * 0.5), 1.0)
        .build();
    sequence
        .all(&mut commands)
        .add_action(
            translate_action(
                target_id,
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 1.0),
            ),
            1.0,
        )
        .add_action(
            scale_action(target_id, Vec3::ONE * 0.5, Vec3::ONE * 2.0),
            1.0,
        )
        .build();
    sequence
        .all(&mut commands)
        .add_action(
            translate_action(
                target_id,
                Vec3::new(1.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 0.0),
            ),
            1.0,
        )
        .build();
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

fn timeline_movemen_system(
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
}
