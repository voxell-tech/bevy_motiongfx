use motiongfx_macro_utils::get_one_field_of_attribute;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FillMotion, attributes(fill))]
pub fn fill_motion_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident.clone();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let fill_ident = get_one_field_of_attribute(&input, "fill");
    let peniko_brush_ident = quote!(crate::convert::PenikoBrush);

    quote! {
        impl #impl_generics FillMotion for #ident #type_generics #where_clause {
            fn brush_to(
                &mut self,
                new_brush: impl Into<#peniko_brush_ident>,
            ) -> Action<FillStyle, peniko::Brush, EmptyRes> {
                let new_brush: peniko::Brush = new_brush.into().0;

                let action: Action<FillStyle, peniko::Brush, EmptyRes> = Action::new(
                    self.target_id,
                    self.#fill_ident.brush.clone(),
                    new_brush.clone(),
                    brush_interp,
                );

                self.#fill_ident.brush = new_brush;

                action
            }

            fn alpha_to(&mut self, new_alpha: f32) -> Action<FillStyle, f32, EmptyRes> {
                let mut alpha = 0.0;

                match &mut self.#fill_ident.brush {
                    peniko::Brush::Solid(color) => {
                        alpha = (color.a / 255) as f32;
                        color.a = (new_alpha * 255.0) as u8;
                    }
                    peniko::Brush::Gradient(grad) => {
                        if grad.stops.len() > 0 {
                            alpha = (grad.stops[0].color.a / 255) as f32;
                        }
                        for stop in &mut grad.stops {
                            stop.color.a = (new_alpha * 255.0) as u8;
                        }
                    }
                    peniko::Brush::Image(_) => {}
                }

                Action::new(self.target_id, alpha, new_alpha, alpha_interp)
            }

        }
    }
    .into()
}

#[proc_macro_derive(VelloBuilder)]
pub fn vello_builder_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    quote! {
        impl VelloBuilder for #ident {
            #[inline]
            fn is_built(&self) -> bool {
                self.built
            }

            #[inline]
            fn set_built(&mut self, built: bool) {
                self.built = built;
            }
        }
    }
    .into()
}

#[proc_macro_derive(VelloVector, attributes(shape))]
pub fn vello_vector_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident.clone();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let shape_ident = get_one_field_of_attribute(&input, "shape");

    quote!(
        impl #impl_generics VelloVector for #ident #type_generics #where_clause {
            #[inline]
            fn shape(&self) -> &impl bevy_vello_renderer::vello::kurbo::Shape {
                &self.#shape_ident
            }
        }
    )
    .into()
}
