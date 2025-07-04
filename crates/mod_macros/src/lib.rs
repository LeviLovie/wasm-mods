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
        use crate::module::guest::structures::{register_structure, unregister_structure};
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

#[proc_macro]
pub fn create_data(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        use crate::module::guest::structures::{
            register_structure as wit_register_structure, unregister_structure as wit_unregister_structure,
        };
        use serde::{Deserialize, Serialize};

        pub struct StructureManager {
            mod_id: String,
        }

        impl StructureManager {
            pub fn new() -> Self {
                let mod_id = env!("CARGO_PKG_NAME").to_string();
                Self { mod_id }
            }

            pub fn register<T: Serialize>(&self, type_name: &str, data: &T) -> Result<String, String> {
                let id = format!("{}:{}-{}", self.mod_id, type_name, uuid::Uuid::new_v4());
                self.register_with_id(&id, type_name, data)
            }

            pub fn register_with_id<T: Serialize>(
                &self,
                id: &str,
                type_name: &str,
                data: &T,
            ) -> Result<String, String> {
                let data_str = match serde_json::to_string(data) {
                    Ok(s) => s,
                    Err(e) => return Err(format!("Serialization error: {}", e)),
                };

                let success = wit_register_structure(id, type_name, &data_str);
                if success {
                    Ok(id.to_string())
                } else {
                    Err(format!("Failed to register structure with ID: {}", id))
                }
            }

            pub fn unregister(&self, id: &str) -> bool {
                wit_unregister_structure(id)
            }

            pub fn update<T: Serialize>(&self, id: &str, data: &T) -> Result<(), String> {
                if !wit_unregister_structure(id) {
                    return Err(format!("Structure with ID {} not found", id));
                }

                let parts: Vec<&str> = id.split(':').collect();
                if parts.len() != 2 {
                    return Err("Invalid structure ID format".to_string());
                }

                let type_parts: Vec<&str> = parts[1].split('-').collect();
                if type_parts.is_empty() {
                    return Err("Invalid structure ID format".to_string());
                }

                let type_name = type_parts[0];
                match self.register_with_id(id, type_name, data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }

            pub fn generate_id(&self, name: &str) -> String {
                format!("{}:{}", self.mod_id, name)
            }
        }

        #[derive(Serialize, Deserialize)]
        pub struct GameEntity<T> {
            id: String,
            entity_type: String,
            data: T,
        }

        impl<T: Serialize> GameEntity<T> {
            pub fn new(id: &str, entity_type: &str, data: T) -> Self {
                Self {
                    id: id.to_string(),
                    entity_type: entity_type.to_string(),
                    data,
                }
            }

            pub fn register(&self, manager: &StructureManager) -> Result<(), String> {
                manager
                    .register_with_id(&self.id, &self.entity_type, &self.data)
                    .map(|_| ())
            }

            pub fn unregister(&self, manager: &StructureManager) -> bool {
                manager.unregister(&self.id)
            }

            pub fn update(&self, manager: &StructureManager) -> Result<(), String> {
                manager.update(&self.id, &self.data)
            }
        }
    };
    return expanded.into();
}
