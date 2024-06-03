use bevy_macro_utils::BevyManifest;
use motiongfx_macro_utils::{get_one_field_of_attribute, MotionGfxManifest};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GetId, attributes(id))]
pub fn get_id_derive_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let bevy_ecs_path = BevyManifest::get_path_direct("bevy_ecs");

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
    let motiongfx_core_path = MotionGfxManifest::get_path_direct("motiongfx_core");
    let transform_motion_builder_path =
        quote!(#motiongfx_core_path::motion::transform_motion::TransformMotionBuilder);

    let ident = ast.ident.clone();
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let transform_ident = get_one_field_of_attribute(&ast, "transform");

    quote! {
        impl #impl_generics TransformMotion for #ident #type_generics #where_clause {
            fn transform(&mut self) -> #transform_motion_builder_path {
                #transform_motion_builder_path::new(self.get_id(), &mut self.#transform_ident)
            }
        }
    }
    .into()
}
#[proc_macro_derive(StandardMaterialMotion, attributes(standard_material))]
pub fn standard_material_motion_derive_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let motiongfx_core_path = MotionGfxManifest::get_path_direct("motiongfx_core");
    let material_motion_builder_path = quote!(#motiongfx_core_path::motion::standard_material_motion::StandardMaterialMotionBuilder);

    let ident = ast.ident.clone();
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let material_ident = get_one_field_of_attribute(&ast, "standard_material");

    quote! {
        impl #impl_generics StandardMaterialMotion for #ident #type_generics #where_clause {
            fn std_material(&mut self) -> #material_motion_builder_path {
                #material_motion_builder_path::new(self.get_id(), &mut self.#material_ident)
            }
        }
    }
    .into()
}
