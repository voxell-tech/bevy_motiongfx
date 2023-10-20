use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    },
    log::LogPlugin,
    pbr::ScreenSpaceAmbientOcclusionBundle,
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

    let mut action_builder: ActionBuilder = ActionBuilder::new(&mut commands);

    let action_grp: ActionMetaGroup = chain(&[
        action_builder
            .play(
                translate_action(target_id, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)),
                1.0,
            )
            .with_ease(ease::cubic::ease_in_out),
        action_builder
            .play(
                translate_action(
                    target_id,
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                ),
                1.0,
            )
            .with_ease(ease::cubic::ease_in_out),
        all(&[
            action_builder
                .play(
                    translate_action(target_id, Vec3::new(1.0, 1.0, 0.0), Vec3::ZERO),
                    1.0,
                )
                .with_ease(ease::expo::ease_in_out),
            action_builder
                .play(scale_action(target_id, Vec3::ONE, Vec3::ONE * 0.5), 1.0)
                .with_ease(ease::expo::ease_in_out),
        ]),
    ]);

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
