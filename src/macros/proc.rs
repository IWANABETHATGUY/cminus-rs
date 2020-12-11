use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{self, Parser},
    parse_macro_input, Data, DeriveInput, ItemStruct,
};

#[proc_macro_attribute]
pub fn add_field(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input2 = input.clone();
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let mut input = parse_macro_input!(input2 as DeriveInput);
    let _ = parse_macro_input!(args as parse::Nothing);
    let name = &input.ident.to_string();
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    #[serde(default = #name)]
                    pub a: Option<String>
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}
