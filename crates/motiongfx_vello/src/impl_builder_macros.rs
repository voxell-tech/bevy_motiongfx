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

macro_rules! impl_transform_builder {
    ($struct_name:ident, $transform:ident) => {
        impl $struct_name {
            pub fn with_transform(
                mut self,
                transform: ::bevy::transform::components::Transform,
            ) -> Self {
                self.$transform = transform;

                self
            }
        }
    };
}

pub(crate) use impl_brush_builder;
pub(crate) use impl_optional_stroke_builder;
pub(crate) use impl_stroke_builder;
pub(crate) use impl_transform_builder;
