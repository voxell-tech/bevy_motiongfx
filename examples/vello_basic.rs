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

fn vello_basic(mut commands: Commands) {
    // Color palette
    let palette = ColorPalette::default();

    // Spawning entities
    let mut line = VelloLine::new(DVec2::new(-300.0, 0.0), DVec2::new(300.0, 0.0));
    let mut line_stroke =
        Stroke::default().with_brush(Brush::from_color(*palette.get(ColorKey::Base8)));
    let mut line_transform = Transform::from_xyz(0.0, -100.0, 0.0);

    let line_id = commands
        .spawn((line, line_stroke.clone(), line_transform))
        .add_vello_handle()
        .id();

    let mut rect = VelloRect::new(100.0, 100.0);
    let rect_fill = Fill::new().with_color(*palette.get(ColorKey::Blue));
    let mut rect_stroke = Stroke::new(4.0).with_color(*palette.get(ColorKey::Blue) * 1.5);
    let mut rect_transform = Transform::from_xyz(-200.0, 0.0, 0.0);

    let rect_id = commands
        .spawn((rect, rect_fill, rect_stroke.clone(), rect_transform))
        .add_vello_handle()
        .id();

    let mut circle = VelloCircle::new(50.0);
    let circle_fill = Fill::new().with_color(*palette.get(ColorKey::Purple));
    let mut circle_stroke = Stroke::new(4.0).with_color(*palette.get(ColorKey::Purple) * 1.5);
    let circle_transform = Transform::from_xyz(200.0, 0.0, 0.0);

    let circle_id = commands
        .spawn((circle, circle_fill, circle_stroke.clone(), circle_transform))
        .add_vello_handle()
        .id();

    // Sequence
    let sequence = flow(
        0.5,
        // Line animation
        &[
            chain(&[
                all(&[
                    commands.play(
                        act!(
                            line_id,
                            Transform = line_transform,
                            translation.y,
                            line_transform.translation.y - 100.0
                        ),
                        1.5,
                    ),
                    commands.play(act!(line_id, VelloLine = line, line.extend(100.0)), 1.0),
                    commands.play(
                        act!(
                            line_id,
                            Stroke = line_stroke,
                            style,
                            kurbo::Stroke::new(10.0)
                        ),
                        1.0,
                    ),
                ]),
                all(&[
                    commands.play(
                        act!(
                            line_id,
                            Transform = line_transform,
                            translation.y,
                            line_transform.translation.y + 100.0
                        ),
                        1.5,
                    ),
                    commands.play(
                        act!(line_id, VelloLine = line, line.clone().extend(-100.0)),
                        1.0,
                    ),
                    commands.play(
                        act!(
                            line_id,
                            Stroke = line_stroke,
                            style,
                            kurbo::Stroke::new(1.0)
                        ),
                        1.0,
                    ),
                ]),
            ]),
            // Rect animation
            chain(&[
                all(&[
                    commands.play(act!(rect_id, VelloRect = rect, size, rect.size + 50.0), 1.0),
                    commands.play(
                        act!(
                            rect_id,
                            Transform = rect_transform,
                            rotation,
                            Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, std::f32::consts::PI)
                        ),
                        1.0,
                    ),
                    commands.play(
                        act!(
                            rect_id,
                            Stroke = rect_stroke,
                            style,
                            kurbo::Stroke::new(20.0)
                        ),
                        1.0,
                    ),
                ]),
                all(&[
                    commands.play(act!(rect_id, VelloRect = rect, size, rect.size - 50.0), 1.0),
                    commands.play(
                        act!(
                            rect_id,
                            Transform = rect_transform,
                            rotation,
                            Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, std::f32::consts::TAU)
                        ),
                        1.0,
                    ),
                    commands.play(
                        act!(
                            rect_id,
                            Stroke = rect_stroke,
                            style,
                            kurbo::Stroke::new(4.0)
                        ),
                        1.0,
                    ),
                ]),
            ]),
            // Circle animation
            chain(&[
                all(&[
                    commands.play(
                        act!(
                            circle_id,
                            VelloCircle = circle,
                            radius,
                            circle.radius + 50.0
                        ),
                        1.0,
                    ),
                    commands.play(
                        act!(
                            circle_id,
                            Stroke = circle_stroke,
                            style,
                            kurbo::Stroke::new(20.0)
                        ),
                        1.0,
                    ),
                ]),
                all(&[
                    commands.play(
                        act!(
                            circle_id,
                            VelloCircle = circle,
                            radius,
                            circle.radius - 50.0
                        ),
                        1.0,
                    ),
                    commands.play(
                        act!(
                            circle_id,
                            Stroke = circle_stroke,
                            style,
                            kurbo::Stroke::new(4.0)
                        ),
                        1.0,
                    ),
                ]),
            ]),
        ],
    );

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
