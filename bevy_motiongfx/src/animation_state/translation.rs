use crate::action::Action;
use bevy::prelude::*;

pub struct Translation {
    target_id: Entity,
    translation: Vec3,
}

impl Translation {
    pub fn new(target_id: Entity, translation: Vec3) -> Self {
        Self {
            target_id,
            translation,
        }
    }

    pub fn translate(&mut self, translation: Vec3) -> Action<Transform, Vec3> {
        let new_translation: Vec3 = self.translation + translation;

        let action: Action<Transform, Vec3> = Action::new(
            self.target_id,
            self.translation,
            new_translation,
            Self::interp,
        );

        self.translation = new_translation;

        action
    }

    pub fn translate_to(&mut self, translation: Vec3) -> Action<Transform, Vec3> {
        let action: Action<Transform, Vec3> =
            Action::new(self.target_id, self.translation, translation, Self::interp);

        self.translation = translation;

        action
    }

    fn interp(transform: &mut Transform, begin: &Vec3, end: &Vec3, t: f32) {
        transform.translation = Vec3::lerp(*begin, *end, t);
    }
}
