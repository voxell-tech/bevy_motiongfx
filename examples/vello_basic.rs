use bevy::math::{DVec2, DVec4};
use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use motiongfx_vello::bevy_vello_renderer::prelude::*;
use motiongfx_vello::bevy_vello_renderer::vello::SceneFragment;
use motiongfx_vello::vello_vector::rect::{VelloRect, VelloRectBundle, VelloRectMotion};

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins((MotionGfx, MotionGfxBevy, MotionGfxVello))
        .add_systems(Startup, (setup, vello_basic))
        .add_systems(Update, timeline_movement_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn vello_basic(
    mut commands: Commands,
    mut fragments: ResMut<Assets<VelloFragment>>,
    mut sequence: ResMut<Sequence>,
) {
    const RECT_COUNT: usize = 14;
    const RECT_SIZE: f32 = 40.0;
    const SPACING: f32 = 5.0;

    let mut rect_motions: Vec<VelloRectMotion> = Vec::with_capacity(RECT_COUNT);

    let start_y: f32 = (RECT_COUNT as f32) * 0.5 * (RECT_SIZE + SPACING);

    for r in 0..RECT_COUNT {
        let rect: VelloRect = VelloRect::anchor_center(DVec2::new(0.0, 0.0), DVec4::splat(10.0));

        let rect_bundle: VelloRectBundle =
            VelloRectBundle {
                rect: rect.clone(),
                fragment_bundle: VelloFragmentBundle {
                    fragment: fragments.add(VelloFragment {
                        fragment: SceneFragment::new().into(),
                    }),
                    transform: TransformBundle::from_transform(Transform::from_translation(
                        Vec3::new(-500.0, start_y - (r as f32) * (RECT_SIZE + SPACING), 0.0),
                    )),
                    ..default()
                },
            };

        let fragment_id: Entity = commands.spawn(rect_bundle).id();

        rect_motions.push(VelloRectMotion::new(fragment_id, rect));
    }

    let mut act: ActionBuilder = ActionBuilder::new(&mut commands);

    let mut inflate_actions: Vec<ActionMetaGroup> = Vec::with_capacity(RECT_COUNT);
    let mut expand_actions: Vec<ActionMetaGroup> = Vec::with_capacity(RECT_COUNT);

    for r in 0..RECT_COUNT {
        inflate_actions.push(
            act.play(rect_motions[r].inflate(Vec2::splat(RECT_SIZE * 0.5)), 1.0)
                .with_ease(ease::expo::ease_in_out),
        );
        expand_actions.push(
            act.play(
                rect_motions[r].expand_right(900.0 * (r as f64) / (RECT_COUNT as f64) + 100.0),
                1.0,
            )
            .with_ease(ease::expo::ease_in_out),
        );
    }

    sequence.play(flow(
        1.0,
        &[flow(0.1, &inflate_actions), flow(0.1, &expand_actions)],
    ));
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
