use bevy::{math::DVec2, prelude::*};
use bevy_motiongfx::prelude::*;

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
    let mut line = commands.build_stroke(
        Transform::from_xyz(0.0, -100.0, 0.0),
        VelloLine::new(DVec2::new(-300.0, 0.0), DVec2::new(300.0, 0.0)),
        Stroke::default().with_brush(Brush::from_color(palette.get(ColorKey::Base8))),
    );

    let mut rect = commands.build_fill_stroke(
        Transform::from_xyz(0.0, -100.0, 0.0),
        VelloRect::new(100.0, 100.0),
        Fill::new().with_color(palette.get(ColorKey::Blue)),
        Stroke::new(4.0).with_color(palette.get(ColorKey::Blue) * 1.5),
    );

    let mut circle = commands.build_fill_stroke(
        Transform::from_xyz(0.0, -100.0, 0.0),
        VelloCircle::new(50.0),
        Fill::new().with_color(palette.get(ColorKey::Purple)),
        Stroke::new(4.0).with_color(palette.get(ColorKey::Purple) * 1.5),
    );

    // Generate sequence
    let line_seq = [
        play!(
            commands,
            act!(
                (line.id, Transform),
                start = { line.transform }.translation.y,
                end = line.transform.translation.y - 100.0,
            )
            .animate(1.5),
            act!(
                (line.id, VelloLine),
                start = { line.vector },
                end = line.vector.extend(100.0),
            )
            .animate(1.0),
            act!(
                (line.id, Stroke),
                start = { line.stroke }.style.width,
                end = 10.0,
            )
            .animate(1.0),
        )
        .all(),
        play!(
            commands,
            act!(
                (line.id, Transform),
                start = { line.transform }.translation.y,
                end = line.transform.translation.y + 100.0,
            )
            .animate(1.5),
            act!(
                (line.id, VelloLine),
                start = { line.vector },
                end = line.vector.extend(-100.0),
            )
            .animate(1.0),
            act!(
                (line.id, Stroke),
                start = { line.stroke }.style.width,
                end = 1.0,
            )
            .animate(1.0),
        )
        .all(),
    ]
    .chain();

    let rect_seq = [
        play!(
            commands,
            act!(
                (rect.id, VelloRect),
                start = { rect.vector }.size,
                end = rect.vector.size + 50.0,
            )
            .animate(1.0),
            act!(
                (rect.id, Transform),
                start = { rect.transform }.rotation,
                end = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, std::f32::consts::PI),
            )
            .animate(1.0),
            act!(
                (rect.id, Stroke),
                start = { rect.stroke }.style.width,
                end = 20.0,
            )
            .animate(1.0),
        )
        .all(),
        play!(
            commands,
            act!(
                (rect.id, VelloRect),
                start = { rect.vector }.size,
                end = rect.vector.size - 50.0,
            )
            .animate(1.0),
            act!(
                (rect.id, Transform),
                start = { rect.transform }.rotation,
                end = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, std::f32::consts::TAU),
            )
            .animate(1.0),
            act!(
                (rect.id, Stroke),
                start = { rect.stroke }.style.width,
                end = 4.0,
            )
            .animate(1.0),
        )
        .all(),
    ]
    .chain();

    let cirlce_seq = [
        play!(
            commands,
            act!(
                (circle.id, VelloCircle),
                start = { circle.vector }.radius,
                end = circle.vector.radius + 50.0,
            )
            .animate(1.0),
            act!(
                (circle.id, Stroke),
                start = { circle.stroke }.style.width,
                end = 20.0,
            )
            .animate(1.0),
        )
        .all(),
        play!(
            commands,
            act!(
                (circle.id, VelloCircle),
                start = { circle.vector }.radius,
                end = circle.vector.radius - 50.0,
            )
            .animate(1.0),
            act!(
                (circle.id, Stroke),
                start = { circle.stroke }.style.width,
                end = 4.0,
            )
            .animate(1.0),
        )
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
