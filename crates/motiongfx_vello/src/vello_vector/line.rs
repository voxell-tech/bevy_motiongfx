use bevy::{math::DVec2, prelude::*};
use bevy_vello_renderer::{prelude::*, vello::kurbo};

use crate::vello_vector::VelloVector;

#[derive(Component, Default, Clone)]
pub struct VelloLine {
    pub p0: DVec2,
    pub p1: DVec2,
}

impl VelloLine {
    pub fn new(p0: DVec2, p1: DVec2) -> Self {
        Self::default().with_p0(p0).with_p1(p1)
    }

    pub fn with_p0(mut self, p0: DVec2) -> Self {
        self.p0 = p0;
        self
    }

    pub fn with_p1(mut self, p1: DVec2) -> Self {
        self.p1 = p1;
        self
    }

    pub fn build(self, commands: &mut Commands, scenes: &mut Assets<VelloScene>) -> Entity {
        commands
            .spawn((
                self.clone(),
                VelloSceneBundle {
                    scene: scenes.add(VelloScene::default()),
                    ..default()
                },
            ))
            .id()
    }
}

impl VelloVector for VelloLine {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::Line::new(
            kurbo::Point::new(self.p0.x, self.p0.y),
            kurbo::Point::new(self.p1.x, self.p1.y),
        )
    }
}
