use motiongfx_macro_utils::get_one_field_of_attribute;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GetId, attributes(id))]
pub fn get_id_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident.clone();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let id_ident = get_one_field_of_attribute(&input, "id");

    quote! {
        impl #impl_generics GetId for #ident #type_generics #where_clause {
            fn get_id(&self) -> ::bevy::ecs::entity::Entity {
                self.#id_ident
            }
        }
    }
    .into()
}

#[proc_macro_derive(TransformMotion, attributes(transform))]
pub fn transform_motion_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident.clone();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let transform_ident = get_one_field_of_attribute(&input, "transform");

    quote! {
        impl #impl_generics TransformMotion for #ident #type_generics #where_clause {
            fn get_transform(&mut self) -> &mut ::bevy::transform::components::Transform {
                &mut self.#transform_ident
            }
        }
    }
    .into()
}
