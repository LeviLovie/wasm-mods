use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, LitStr, Result};

struct WitBindgenArgs {
    path: LitStr,
}

impl Parse for WitBindgenArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let path = input.parse::<LitStr>()?;
        Ok(WitBindgenArgs { path })
    }
}

#[proc_macro]
pub fn create_mod(input: TokenStream) -> TokenStream {
    // Parse the path argument
    let args = parse_macro_input!(input as WitBindgenArgs);
    let path_str = args.path.value();

    // Generate the code
    let expanded = quote! {
        wit_bindgen::generate!({
            path: #path_str,
            exports: {
                "module:guest/events": Events,
                "module:guest/events/data": Data,
            },
        });
        use crate::exports::module::guest::events::*;
        use crate::module::guest::log::log;

        pub struct Events {}

        impl Guest for Events {
            fn info() -> Vec<String> {
                let version = env!("CARGO_PKG_VERSION").to_string();
                let id = env!("CARGO_PKG_NAME").to_string();
                let authors = env!("CARGO_PKG_AUTHORS");
                let author = authors.split(':').collect::<Vec<&str>>().join(", ");
                let description = env!("CARGO_PKG_DESCRIPTION").to_string();
                let description_parts = description.split(": ").collect::<Vec<&str>>();
                let name = description_parts[0].to_string();
                let description = description_parts[1..].join(": ");
                return vec![id, name, version, author, description];
            }
        }
    };

    expanded.into()
}
