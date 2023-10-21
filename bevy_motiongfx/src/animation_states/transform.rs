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

pub struct Scale {
    target_id: Entity,
    scale: Vec3,
}

impl Scale {
    pub fn new(target_id: Entity, scale: Vec3) -> Self {
        Self { target_id, scale }
    }

    pub fn scale(&mut self, scale: Vec3) -> Action<Transform, Vec3> {
        let new_scale: Vec3 = self.scale * scale;

        let action: Action<Transform, Vec3> =
            Action::new(self.target_id, self.scale, new_scale, Self::interp);

        self.scale = new_scale;

        action
    }

    pub fn scale_all(&mut self, scale: f32) -> Action<Transform, Vec3> {
        let new_scale: Vec3 = self.scale * scale;

        let action: Action<Transform, Vec3> =
            Action::new(self.target_id, self.scale, new_scale, Self::interp);

        self.scale = new_scale;

        action
    }

    pub fn scale_to(&mut self, scale: Vec3) -> Action<Transform, Vec3> {
        let action: Action<Transform, Vec3> =
            Action::new(self.target_id, self.scale, scale, Self::interp);

        self.scale = scale;

        action
    }

    pub fn scale_all_to(&mut self, scale: f32) -> Action<Transform, Vec3> {
        let new_scale: Vec3 = Vec3::ONE * scale;

        let action: Action<Transform, Vec3> =
            Action::new(self.target_id, self.scale, new_scale, Self::interp);

        self.scale = new_scale;

        action
    }

    fn interp(transform: &mut Transform, begin: &Vec3, end: &Vec3, t: f32) {
        transform.scale = Vec3::lerp(*begin, *end, t);
    }
}
