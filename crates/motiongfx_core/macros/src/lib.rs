use proc_macro::TokenStream;
use quote::quote;
// use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn rect__(input: TokenStream) -> TokenStream {
    // let inputs = input.into_iter().collect::<Vec<_>>();

    for token in input {
        println!("{:?}\n", token);
        // match token {
        //     proc_macro::TokenTree::Literal(literal) => {
        //         println!("{:?}", literal);
        //     }
        //     proc_macro::TokenTree::Group(group) => {
        //         println!("{:?}", group)
        //     }
        //     proc_macro::TokenTree::Ident(ident) => {
        //         println!("{:?}", ident)
        //     }
        //     proc_macro::TokenTree::Punct(punct) => {
        //         println!("{:?}", punct)
        //     }
        // }
    }
    // let input = parse_macro_input!(input as DeriveInput);
    // let ident = input.ident;

    quote! {}.into()
}
