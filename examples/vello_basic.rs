use bevy::{math::DVec2, prelude::*};
use bevy_motiongfx::prelude::*;
use motiongfx_vello::bevy_vello_renderer::vello::kurbo;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins((MotionGfxPlugin, MotionGfxVelloPlugin))
        .add_systems(Startup, (setup, vello_basic))
        .add_systems(Update, timeline_movement)
        .run();
}

fn vello_basic(mut commands: Commands, mut scenes: ResMut<Assets<VelloScene>>) {
    // Color palette
    let palette = ColorPalette::default();

    // Spawning entities
    let circ_bundle = VelloCircleBundle {
        circle: VelloCircle::from_radius(50.0),
        fill: FillStyle::from_brush(*palette.get_or_default(&ColorKey::Purple)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::Purple) * 1.5)
            .with_style(4.0),
        scene_bundle: VelloSceneBundle {
            scene: scenes.add(VelloScene::default()),
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
    };

    let line_bundle = VelloLineBundle {
        line: VelloLine::from_points(DVec2::new(-300.0, 0.0), DVec2::new(300.0, 0.0)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::Base8)),
        scene_bundle: VelloSceneBundle {
            scene: scenes.add(VelloScene::default()),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
    };

    let circ_id = commands.spawn(circ_bundle.clone()).id();
    let line_id = commands.spawn(line_bundle.clone()).id();

    // Motions
    // TODO: Set transform
    // transform: Transform::from_xyz(-200.0, 0.0, 0.0),
    let mut rect_motion = VelloRect::new(100.0, 100.0)
        .with_anchor(0.5, 0.5)
        .with_fill_color(*palette.get_or_default(&ColorKey::Blue))
        .with_stroke(kurbo::Stroke::new(4.0))
        .with_stroke_color(*palette.get_or_default(&ColorKey::Blue) * 1.5)
        .build(&mut commands, &mut scenes);

    let mut circ_motion = VelloCircleBundleMotion::new(circ_id, circ_bundle);
    let mut line_motion = VelloLineBundleMotion::new(line_id, line_bundle);

    // Sequence
    let sequence = flow(
        0.5,
        // Line animation
        &[
            chain(&[
                all(&[
                    commands.play(
                        line_motion
                            .transform
                            .translate_add(Vec3::new(0.0, -100.0, 0.0)),
                        1.5,
                    ),
                    commands.play(line_motion.line.extend(100.0), 1.0),
                    commands.play(line_motion.stroke.style_to(10.0), 1.0),
                ]),
                all(&[
                    commands.play(
                        line_motion
                            .transform
                            .translate_add(Vec3::new(0.0, 100.0, 0.0)),
                        1.5,
                    ),
                    commands.play(line_motion.line.extend(-100.0), 1.0),
                    commands.play(line_motion.stroke.style_to(1.0), 1.0),
                ]),
            ]),
            // Rect animation
            chain(&[
                all(&[
                    commands.play(rect_motion.add_size(50.0, 50.0), 1.0),
                    commands.play(
                        rect_motion.rotate_to(Quat::from_euler(
                            EulerRot::XYZ,
                            0.0,
                            0.0,
                            std::f32::consts::PI,
                        )),
                        1.0,
                    ),
                    commands.play(rect_motion.to_stroke_width(20.0), 1.0),
                ]),
                all(&[
                    commands.play(rect_motion.add_size(-50.0, -50.0), 1.0),
                    commands.play(
                        rect_motion.rotate_to(Quat::from_euler(
                            EulerRot::XYZ,
                            0.0,
                            0.0,
                            std::f32::consts::TAU,
                        )),
                        1.0,
                    ),
                    commands.play(rect_motion.to_stroke_width(4.0), 1.0),
                ]),
            ]),
            // Circle animation
            chain(&[
                all(&[
                    commands.play(circ_motion.circle.inflate(50.0), 1.0),
                    commands.play(circ_motion.stroke.style_to(20.0), 1.0),
                ]),
                all(&[
                    commands.play(circ_motion.circle.inflate(-50.0), 1.0),
                    commands.play(circ_motion.stroke.style_to(4.0), 1.0),
                ]),
            ]),
        ],
    )
    .with_ease(ease::cubic::ease_in_out);

    commands.spawn(SequencePlayerBundle {
        sequence,
        ..default()
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
