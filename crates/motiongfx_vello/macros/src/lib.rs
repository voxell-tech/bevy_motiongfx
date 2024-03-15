use motiongfx_macro_utils::get_one_field_of_attribute;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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
