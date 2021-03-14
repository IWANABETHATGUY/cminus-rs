use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CodeSpan)]
pub fn code_span(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl Codespan for #name {
            fn start(&self) -> usize {
                self.start
            }

            fn end(&self) -> usize {
                self.end
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
