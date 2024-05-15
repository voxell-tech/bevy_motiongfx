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

    quote! {
        impl #impl_generics FillMotion for #ident #type_generics #where_clause {
            fn get_fill(&mut self) -> &mut ::bevy_vello_graphics::fill::Fill {
                &mut self.#fill_ident
            }
        }
    }
    .into()
}

#[proc_macro_derive(StrokeMotion, attributes(stroke))]
pub fn stroke_motion_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident.clone();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let stroke_ident = get_one_field_of_attribute(&input, "stroke");

    quote! {
        impl #impl_generics StrokeMotion for #ident #type_generics #where_clause {
            fn get_stroke(&mut self) -> &mut ::bevy_vello_graphics::stroke::Stroke {
                &mut self.#stroke_ident
            }
        }
    }
    .into()
}
