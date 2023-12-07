use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use motiongfx_bevy::prelude::TransformMotion;
use motiongfx_core::prelude::*;

use crate::{
    fill_style::FillStyleMotion,
    stroke_style::StrokeStyleMotion,
    vello_vector::{
        text::{VelloTextSimple, VelloTextSimpleBundle},
        VelloBuilder,
    },
};

pub(crate) struct VelloTextSimpleMotionPlugin;

impl Plugin for VelloTextSimpleMotionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PostUpdate,
            (sequence_player_system::<VelloTextSimple, String, EmptyRes>,),
        );
    }
}

pub struct VelloTextSimpleBundleMotion {
    pub text: VelloTextSimpleMotion,
    pub fill: FillStyleMotion,
    pub stroke: StrokeStyleMotion,
    pub transform: TransformMotion,
}

impl VelloTextSimpleBundleMotion {
    pub fn new(target_id: Entity, bundle: VelloTextSimpleBundle) -> Self {
        Self {
            text: VelloTextSimpleMotion::new(target_id, bundle.text),
            fill: FillStyleMotion::new(target_id, bundle.fill),
            stroke: StrokeStyleMotion::new(target_id, bundle.stroke),
            transform: TransformMotion::new(target_id, bundle.fragment_bundle.transform.local),
        }
    }
}

// pub struct VelloTextSimpleBundleMotion

pub struct VelloTextSimpleMotion {
    target_id: Entity,
    vello_text: VelloTextSimple,
}

impl VelloTextSimpleMotion {
    pub fn new(target_id: Entity, vello_text: VelloTextSimple) -> Self {
        Self {
            target_id,
            vello_text,
        }
    }

    pub fn content_to(
        &mut self,
        new_content: impl Into<String>,
    ) -> Action<VelloTextSimple, String, EmptyRes> {
        let new_content: String = new_content.into();

        let action: Action<VelloTextSimple, String, EmptyRes> = Action::new(
            self.target_id,
            self.vello_text.content.clone(),
            new_content.clone(),
            Self::content_interp,
        );

        self.vello_text.content = new_content;

        action
    }

    fn content_interp(
        vello_text: &mut VelloTextSimple,
        begin: &String,
        end: &String,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        let begin_bytes: &[u8] = begin.as_bytes();
        let end_bytes: &[u8] = end.as_bytes();

        let begin_len: usize = begin_bytes.len();
        let end_len: usize = end_bytes.len();

        let max_len: usize = usize::max(begin_len, end_len);
        let lerp_len: usize = (max_len as f32 * t) as usize;

        // Refresh the String content
        vello_text.content.clear();

        // Replace with end charcters based on lerp value
        for c in 0..lerp_len {
            if c < end_len {
                vello_text.content.push(end_bytes[c] as char);
            } else {
                vello_text.content.push(' ');
            }
        }

        // Put back the begin characters
        for c in lerp_len..begin_len {
            vello_text.content.push(begin_bytes[c] as char);
        }

        vello_text.set_built(false);
    }
}
