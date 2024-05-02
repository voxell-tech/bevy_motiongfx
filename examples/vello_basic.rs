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
            play!(
                (commands, line.id, Transform),
                from = { line.transform }.translation.y,
                to = line.transform.translation.y - 100.0,
                duration = 1.5,
            ),
            play!(
                (commands, line.id, VelloLine),
                from = { line.vector },
                to = line.vector.extend(100.0),
                duration = 1.0,
            ),
            play!(
                (commands, line.id, Stroke),
                from = { line.stroke }.style.width,
                to = 10.0,
                duration = 1.0,
            ),
        ]
        .all(),
        [
            play!(
                (commands, line.id, Transform),
                from = { line.transform }.translation.y,
                to = line.transform.translation.y + 100.0,
                duration = 1.5,
            ),
            play!(
                (commands, line.id, VelloLine),
                from = { line.vector },
                to = line.vector.extend(-100.0),
                duration = 1.0,
            ),
            play!(
                (commands, line.id, Stroke),
                from = { line.stroke }.style.width,
                to = 1.0,
                duration = 1.0,
            ),
        ]
        .all(),
    ]
    .chain();

    let rect_seq = [
        [
            play!(
                (commands, rect.id, VelloRect),
                from = { rect.vector }.size,
                to = rect.vector.size + 50.0,
                duration = 1.0,
            ),
            play!(
                (commands, rect.id, Transform),
                from = { rect.transform }.rotation,
                to = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, std::f32::consts::PI),
                duration = 1.0,
            ),
            play!(
                (commands, rect.id, Stroke),
                from = { rect.stroke }.style.width,
                to = 20.0,
                duration = 1.0,
            ),
        ]
        .all(),
        [
            play!(
                (commands, rect.id, VelloRect),
                from = { rect.vector }.size,
                to = rect.vector.size - 50.0,
                duration = 1.0,
            ),
            play!(
                (commands, rect.id, Transform),
                from = { rect.transform }.rotation,
                to = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, std::f32::consts::TAU),
                duration = 1.0,
            ),
            play!(
                (commands, rect.id, Stroke),
                from = { rect.stroke }.style.width,
                to = 4.0,
                duration = 1.0,
            ),
        ]
        .all(),
    ]
    .chain();

    let cirlce_seq = [
        [
            play!(
                (commands, circle.id, VelloCircle),
                from = { circle.vector }.radius,
                to = circle.vector.radius + 50.0,
                duration = 1.0,
            ),
            play!(
                (commands, circle.id, Stroke),
                from = { circle.stroke }.style.width,
                to = 20.0,
                duration = 1.0,
            ),
        ]
        .all(),
        [
            play!(
                (commands, circle.id, VelloCircle),
                from = { circle.vector }.radius,
                to = circle.vector.radius - 50.0,
                duration = 1.0,
            ),
            play!(
                (commands, circle.id, Stroke),
                from = { circle.stroke }.style.width,
                to = 4.0,
                duration = 1.0,
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
