use bevy_macro_utils::BevyManifest;
use motiongfx_macro_utils::get_one_field_of_attribute;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn bevy_ecs_path() -> syn::Path {
    BevyManifest::default().get_path("bevy_ecs")
}

pub(crate) fn bevy_transform_path() -> syn::Path {
    BevyManifest::default().get_path("bevy_transform")
}

#[proc_macro_derive(GetId, attributes(id))]
pub fn get_id_derive_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let bevy_ecs_path = bevy_ecs_path();

    let ident = ast.ident.clone();
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let id_ident = get_one_field_of_attribute(&ast, "id");

    quote! {
        impl #impl_generics GetId for #ident #type_generics #where_clause {
            fn get_id(&self) -> #bevy_ecs_path::entity::Entity {
                self.#id_ident
            }
        }
    }
    .into()
}

#[proc_macro_derive(TransformMotion, attributes(transform))]
pub fn transform_motion_derive_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let bevy_transform_path = bevy_transform_path();

    let ident = ast.ident.clone();
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let transform_ident = get_one_field_of_attribute(&ast, "transform");

    quote! {
        impl #impl_generics TransformMotion for #ident #type_generics #where_clause {
            fn get_transform(&mut self) -> &mut #bevy_transform_path::components::Transform {
                &mut self.#transform_ident
            }
        }
    }
    .into()
}
