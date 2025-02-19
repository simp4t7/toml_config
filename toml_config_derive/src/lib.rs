use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TomlConfig)]
pub fn toml_config(input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let struct_name = &input_struct.ident;

    let expanded = quote!(
        impl TomlConfigTrait for #struct_name {}
    );

    expanded.into()
}
