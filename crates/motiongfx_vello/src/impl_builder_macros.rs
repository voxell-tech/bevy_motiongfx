macro_rules! impl_fill_brush_builder {
    ($struct_name:ident, $fill:ident) => {
        impl $struct_name {
            pub fn test(&self) {
                println!("{:#?}", self.$fill);
            }
        }
    };
}

macro_rules! impl_stroke_builder {
    ($struct_name:ident, $stroke:ident) => {
        impl $struct_name {
            pub fn test(&self) {
                println!("{:#?}", self.$stroke);
            }
        }
    };
}

pub(crate) use impl_fill_brush_builder;
pub(crate) use impl_stroke_builder;
