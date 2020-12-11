use quote::quote;
use syn::{
    parse::{self, Parser},
    parse_macro_input, DeriveInput, ItemStruct,
};

#[proc_macro_attribute]
pub fn add_type(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    pub r#type: String
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}


#[proc_macro_derive(DefaultType)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let name_string = name.to_string();
    let expand = quote! {
        impl Default for #name {
            fn default() -> Self {
                Self {
                    r#type: String::from(#name_string),
                    ..Default::default()
                }
            }
        }
    };
    expand.into()
}
