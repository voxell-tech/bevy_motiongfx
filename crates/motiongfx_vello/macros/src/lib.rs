use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(VelloBuilder)]
pub fn reflective_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

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
