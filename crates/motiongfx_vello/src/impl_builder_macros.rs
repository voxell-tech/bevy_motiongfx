macro_rules! impl_brush_builder {
    ($fn_name:tt, $struct_name:ident, $brush:ident) => {
        ::paste::paste! {
            impl $struct_name {
                pub fn [< with_ $fn_name _color >](mut self, color: ::bevy::render::color::Color) -> Self {
                    self.$brush = ::bevy_vello_renderer::vello::peniko::Brush::Solid(
                        ::bevy_vello_renderer::vello::peniko::Color::rgba(
                            color.r() as f64,
                            color.g() as f64,
                            color.b() as f64,
                            color.a() as f64,
                        ),
                    );
                    self
                }
            }
        }
    };
}

macro_rules! impl_stroke_builder {
    ($struct_name:ident, $stroke:ident) => {
        impl $struct_name {
            pub fn with_stroke(
                mut self,
                stroke: ::bevy_vello_renderer::vello::kurbo::Stroke,
            ) -> Self {
                self.$stroke = stroke;
                self
            }
        }
    };
}

macro_rules! impl_optional_stroke_builder {
    ($struct_name:ident, $stroke:ident) => {
        impl $struct_name {
            pub fn with_stroke(
                mut self,
                stroke: ::bevy_vello_renderer::vello::kurbo::Stroke,
            ) -> Self {
                self.$stroke = Some(stroke);
                self
            }
        }
    };
}

macro_rules! impl_transform_motion {
    ($struct_name:ident, $transform:ident, $target_id:ident) => {
        impl $struct_name {
            pub fn transform_to(
                &mut self,
                transform: ::bevy::transform::components::Transform,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::transform::components::Transform,
                $crate::EmptyRes,
            > {
                let action = $crate::Action::new(
                    self.$target_id,
                    self.$transform,
                    transform,
                    |transform: &mut ::bevy::transform::components::Transform, begin, end, t, _| {
                        transform.translation =
                            ::bevy::math::Vec3::lerp(begin.translation, end.translation, t);
                        transform.rotation =
                            ::bevy::math::Quat::slerp(begin.rotation, end.rotation, t);
                        transform.scale = ::bevy::math::Vec3::lerp(begin.scale, end.scale, t);
                    },
                );

                self.$transform = transform;
                action
            }

            pub fn translate_to(
                &mut self,
                translation: ::bevy::math::Vec3,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::math::Vec3,
                $crate::EmptyRes,
            > {
                let action = self.create_translation_action(translation);
                self.$transform.translation = translation;
                action
            }

            pub fn translate_add(
                &mut self,
                translation: ::bevy::math::Vec3,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::math::Vec3,
                $crate::EmptyRes,
            > {
                let translation = self.$transform.translation + translation;

                let action = self.create_translation_action(translation);
                self.$transform.translation = translation;
                action
            }

            pub fn rotate_to(
                &mut self,
                rotation: ::bevy::math::Quat,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::math::Quat,
                $crate::EmptyRes,
            > {
                let action = self.create_rotation_action(rotation);
                self.$transform.rotation = rotation;
                action
            }

            pub fn scale_to(
                &mut self,
                scale: ::bevy::math::Vec3,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::math::Vec3,
                $crate::EmptyRes,
            > {
                let action = self.create_scale_action(scale);
                self.$transform.scale = scale;
                action
            }

            pub fn scale_add(
                &mut self,
                scale: ::bevy::math::Vec3,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::math::Vec3,
                $crate::EmptyRes,
            > {
                let scale = self.$transform.scale + scale;
                let action = self.create_scale_action(scale);
                self.$transform.scale = scale;
                action
            }

            fn create_translation_action(
                &self,
                end: ::bevy::math::Vec3,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::math::Vec3,
                $crate::EmptyRes,
            > {
                $crate::Action::new(
                    self.$target_id,
                    self.$transform.translation,
                    end,
                    |transform: &mut ::bevy::transform::components::Transform, begin, end, t, _| {
                        transform.translation = ::bevy::math::Vec3::lerp(*begin, *end, t);
                    },
                )
            }

            fn create_rotation_action(
                &self,
                end: ::bevy::math::Quat,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::math::Quat,
                $crate::EmptyRes,
            > {
                $crate::Action::new(
                    self.$target_id,
                    self.$transform.rotation,
                    end,
                    |transform: &mut ::bevy::transform::components::Transform, begin, end, t, _| {
                        transform.rotation = ::bevy::math::Quat::slerp(*begin, *end, t);
                    },
                )
            }

            fn create_scale_action(
                &self,
                end: ::bevy::math::Vec3,
            ) -> $crate::Action<
                ::bevy::transform::components::Transform,
                ::bevy::math::Vec3,
                $crate::EmptyRes,
            > {
                $crate::Action::new(
                    self.$target_id,
                    self.$transform.scale,
                    end,
                    |transform: &mut ::bevy::transform::components::Transform, begin, end, t, _| {
                        transform.scale = ::bevy::math::Vec3::lerp(*begin, *end, t);
                    },
                )
            }
        }
    };
}

pub(crate) use impl_brush_builder;
pub(crate) use impl_optional_stroke_builder;
pub(crate) use impl_stroke_builder;
pub(crate) use impl_transform_motion;
