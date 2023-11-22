use bevy::{
    math::{DVec2, DVec4},
    prelude::*,
};
use bevy_motiongfx::prelude::*;
use motiongfx_vello::{
    bevy_vello_renderer::prelude::*,
    fill_style::FillStyle,
    stroke_style::StrokeStyle,
    vello_motion::rect_motion::VelloRectBundleMotion,
    vello_vector::rect::{VelloRect, VelloRectBundle},
};

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
    // Color palette
    let palette: ColorPalette<ColorKey> = ColorPalette::default();

    let rect_bundle: VelloRectBundle = VelloRectBundle {
        rect: VelloRect::anchor_center(DVec2::new(100.0, 100.0), DVec4::splat(10.0)),
        fill: FillStyle::from_brush(*palette.get_or_default(&ColorKey::Blue)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::Base8)).with_style(4.0),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(Transform::from_xyz(-100.0, 0.0, 0.0)),
            ..default()
        },
    };

    let fragment_id: Entity = commands.spawn(rect_bundle.clone()).id();

    // Actions
    let mut act: ActionBuilder = ActionBuilder::new(&mut commands);

    // sequence.play();
}

fn timeline_movement_system(
    mut timeline: ResMut<Timeline>,
    sequence: Res<Sequence>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if keys.pressed(KeyCode::D) {
        timeline.target_time += time.delta_seconds();
    }

    if keys.pressed(KeyCode::A) {
        timeline.target_time -= time.delta_seconds();
    }

    // Ping pong animation while playing
    if timeline.is_playing
        && (timeline.target_time <= 0.0 || timeline.target_time >= sequence.duration())
    {
        timeline.time_scale *= -1.0;
    }

    if keys.pressed(KeyCode::Space) && keys.pressed(KeyCode::ShiftLeft) {
        timeline.time_scale = -1.0;
        timeline.is_playing = true;
    } else if keys.pressed(KeyCode::Space) {
        timeline.time_scale = 1.0;
        timeline.is_playing = true;
    }
}
