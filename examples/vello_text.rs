use bevy::{
    math::{DVec2, DVec4},
    prelude::*,
};
use bevy_motiongfx::prelude::*;
use motiongfx_vello::prelude::*;

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

    // Spawning entities
    let text_bundle: VelloTextSimpleBundle = VelloTextSimpleBundle {
        text: VelloTextSimple::from_content("Hello world!").with_size(16.0),
        fill: FillStyle::from_brush(*palette.get_or_default(&ColorKey::Blue)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::default()))
            .with_style(0.4),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                -500.0, 0.0, 0.0,
            ))),
            ..default()
        },
    };

    let text_id: Entity = commands.spawn(text_bundle.clone()).id();

    let mut text_motion: VelloTextSimpleBundleMotion =
        VelloTextSimpleBundleMotion::new(text_id, text_bundle);

    let mut act: ActionBuilder = ActionBuilder::new(&mut commands);

    sequence.play(act.play(text_motion.text.content_to("Transformed text!"), 1.0));
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
