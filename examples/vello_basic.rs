use bevy::{math::DVec2, prelude::*};
use bevy_motiongfx::prelude::*;
use motiongfx_core::sequence::MultiSequenceOrdering;

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

    // Create vello graphics
    let mut line = build_vector!(
        commands,
        vector = VelloLine::new(DVec2::new(-300.0, 0.0), DVec2::new(300.0, 0.0)),
        stroke = Stroke::default().with_brush(Brush::from_color(palette.get(ColorKey::Base8))),
        transform = Transform::from_xyz(0.0, -100.0, 0.0)
    );

    let mut rect = build_vector!(
        commands,
        vector = VelloRect::new(100.0, 100.0),
        fill = Fill::new().with_color(palette.get(ColorKey::Blue)),
        stroke = Stroke::new(4.0).with_color(palette.get(ColorKey::Blue) * 1.5),
        transform = Transform::from_xyz(-200.0, 0.0, 0.0)
    );

    let mut circle = build_vector!(
        commands,
        vector = VelloCircle::new(50.0),
        fill = Fill::new().with_color(palette.get(ColorKey::Purple)),
        stroke = Stroke::new(4.0).with_color(palette.get(ColorKey::Purple) * 1.5),
        transform = Transform::from_xyz(200.0, 0.0, 0.0)
    );

    // Generate sequence
    let line_seq = [
        [
            commands.play(
                act!(
                    line.id,
                    Transform = line.transform,
                    translation.y,
                    line.transform.translation.y - 100.0
                ),
                1.5,
            ),
            commands.play(
                act!(line.id, VelloLine = line.vector, line.vector.extend(100.0)),
                1.0,
            ),
            commands.play(act!(line.id, Stroke = line.stroke, style.width, 10.0), 1.0),
        ]
        .all(),
        [
            commands.play(
                act!(
                    line.id,
                    Transform = line.transform,
                    translation.y,
                    line.transform.translation.y + 100.0
                ),
                1.5,
            ),
            commands.play(
                act!(line.id, VelloLine = line.vector, line.vector.extend(-100.0)),
                1.0,
            ),
            commands.play(act!(line.id, Stroke = line.stroke, style.width, 1.0), 1.0),
        ]
        .all(),
    ]
    .chain();

    let rect_seq = [
        [
            commands.play(
                act!(
                    rect.id,
                    VelloRect = rect.vector,
                    size,
                    rect.vector.size + 50.0
                ),
                1.0,
            ),
            commands.play(
                act!(
                    rect.id,
                    Transform = rect.transform,
                    rotation,
                    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, std::f32::consts::PI)
                ),
                1.0,
            ),
            commands.play(act!(rect.id, Stroke = rect.stroke, style.width, 20.0), 1.0),
        ]
        .all(),
        [
            commands.play(
                act!(
                    rect.id,
                    VelloRect = rect.vector,
                    size,
                    rect.vector.size - 50.0
                ),
                1.0,
            ),
            commands.play(
                act!(
                    rect.id,
                    Transform = rect.transform,
                    rotation,
                    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, std::f32::consts::TAU)
                ),
                1.0,
            ),
            commands.play(act!(rect.id, Stroke = rect.stroke, style.width, 4.0), 1.0),
        ]
        .all(),
    ]
    .chain();

    let cirlce_seq = [
        [
            commands.play(
                act!(
                    circle.id,
                    VelloCircle = circle.vector,
                    radius,
                    circle.vector.radius + 50.0
                ),
                1.0,
            ),
            commands.play(
                act!(circle.id, Stroke = circle.stroke, style.width, 20.0),
                1.0,
            ),
        ]
        .all(),
        [
            commands.play(
                act!(
                    circle.id,
                    VelloCircle = circle.vector,
                    radius,
                    circle.vector.radius - 50.0
                ),
                1.0,
            ),
            commands.play(
                act!(circle.id, Stroke = circle.stroke, style.width, 4.0),
                1.0,
            ),
        ]
        .all(),
    ]
    .chain();

    let sequence = [line_seq, rect_seq, cirlce_seq].flow(0.5);

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
