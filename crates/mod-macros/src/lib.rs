use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, Expr};

#[proc_macro]
pub fn create_mod(input: TokenStream) -> TokenStream {
    // Parse input as a list of comma-separated string literals
    let args = parse_macro_input!(input with Punctuated::<Expr, syn::Token![,]>::parse_terminated);

    let parts: Vec<String> = args
        .iter()
        .map(|expr| {
            if let Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = expr
            {
                lit_str.value()
            } else {
                panic!("Expected a string literal");
            }
        })
        .collect();

    // Extract parts safely with defaults
    let mod_id = parts.get(0).map_or("my_mod", String::as_str);
    let mut mod_name = mod_id.to_string();
    if let Some(name) = parts.get(1) {
        mod_name = name.to_string();
    }
    let mut mod_description = "A custom mod".to_string();
    if let Some(desc) = parts.get(2) {
        mod_description = desc.to_string();
    }
    let mut mod_author = "Unknown".to_string();
    if let Some(author) = parts.get(3) {
        mod_author = author.to_string();
    }
    let mut mod_version = "1.0.0".to_string();
    if let Some(version) = parts.get(4) {
        mod_version = version.to_string();
    }

    // Generate a valid Rust identifier
    let struct_name = format_ident!("{}", {
        let cleaned_name = mod_name.replace([' ', '"'], "");
        if cleaned_name.ends_with("Mod") {
            cleaned_name
        } else {
            format!("{}Mod", cleaned_name)
        }
    });

    let expanded = quote! {
        use common::{ModInterface, ModInfo, ModContext, ModResult};
        use wasm_bindgen::prelude::*;

        #[derive(Default, ModInterface)]
        //#[wasm_bindgen]
        pub struct #struct_name {
            // Add your mod state here
        }

        // Custom implementation - will be extended by the ModImpl derive macro
        impl #struct_name {
            // Initialize a new instance of the mod
            pub fn new() -> Self {
                Self::default()
            }

            // Override these methods in your implementation
            pub fn on_init(&self, context: &ModContext) -> ModResult<()> {
                // Called when the mod is initialized
                println!("Initializing mod: {}", #mod_name);
                Ok(())
            }

            pub fn on_update(&self, delta_time: f32) -> ModResult<()> {
                // Called every frame/tick
                Ok(())
            }

            pub fn on_shutdown(&self) -> ModResult<()> {
                // Called when the mod is being unloaded
                println!("Shutting down mod: {}", #mod_name);
                Ok(())
            }
        }

        //// Implement the ModInterface trait
        //// This will be done by the derive macro, but we add custom wrappers here
        impl ModInterface for #struct_name {
            fn init(&mut self, context: ModContext) -> ModResult<()> {
                self.on_init(&context)
            }

            fn update(&mut self, delta_time: f32) -> ModResult<()> {
                self.on_update(delta_time)
            }

            fn shutdown(&mut self) -> ModResult<()> {
                self.on_shutdown()
            }

            fn get_info(&self) -> ModInfo {
                self.info
            }
        }

        // Export the mod interface for WASM
        //#[wasm_bindgen]
        pub fn get_info() -> ModInfo {
            let instance = #struct_name::default();
            instance.get_info()
        }

        //#[wasm_bindgen]
        pub fn init(context: ModContext) -> Result<(), String> {
            let instance = #struct_name::default();
            instance.init(context)
        }

        //#[wasm_bindgen]
        pub fn update(delta_time: f32) -> Result<(), String> {
            let instance = #struct_name::default();
            instance.update(delta_time)
        }

        #[wasm_bindgen]
        pub fn shutdown() -> Result<(), String> {
            let instance = #struct_name::default();
            instance.shutdown()
        }
    };

    TokenStream::from(expanded)
}

//// Helper macro to create a new mod
//#[proc_macro]
//pub fn create_mod(input: TokenStream) -> TokenStream {
//    let input_str = input.to_string();
//    let parts: Vec<&str> = input_str.split(',').map(|s| s.trim()).collect();
//
//    let mod_id = if parts.is_empty() { "my_mod" } else { parts[0] };
//    let mod_name = if parts.len() < 2 { mod_id } else { parts[1] };
//    let mod_description = if parts.len() < 3 {
//        "A custom mod"
//    } else {
//        parts[2]
//    };
//    let mod_author = if parts.len() < 4 { "Unknown" } else { parts[3] };
//    let mod_version = if parts.len() < 5 { "1.0.0" } else { parts[4] };
//
//    let struct_name = format_ident!("{}", {
//        if mod_name.is_empty() {
//            "MyMod".to_string()
//        } else {
//            if mod_name.ends_with("Mod") {
//                mod_name.to_string() // No need to add "Mod" if it already ends with "Mod"
//            } else {
//                format!("{}Mod", mod_name).to_string() // Append "Mod" if it doesn't
//            }
//        }
//    });
//
//    // Generate the mod struct with required implementations
//    let expanded = quote! {
//        use common::{ModInterface, ModInfo, ModContext, ModResult};
//        use mod_macros::ModImpl;
//        use wasm_bindgen::prelude::*;
//
//        //#[derive(Default)]
//        //#[wasm_bindgen]
//        //#[mod_info(
//        //    id = #mod_id,
//        //    name = #mod_name,
//        //    description = #mod_description,
//        //    author = #mod_author,
//        //    version = #mod_version
//        //)]
//        pub struct #struct_name {
//            // Add your mod state here
//        }
//
//        // Custom implementation - will be extended by the ModImpl derive macro
//        //impl #struct_name {
//        //    // Initialize a new instance of the mod
//        //    pub fn new() -> Self {
//        //        Self::default()
//        //    }
//        //
//        //    // Override these methods in your implementation
//        //    pub fn on_init(&self, context: &ModContext) -> ModResult<()> {
//        //        // Called when the mod is initialized
//        //        println!("Initializing mod: {}", #mod_name);
//        //        Ok(())
//        //    }
//        //
//        //    pub fn on_update(&self, delta_time: f32) -> ModResult<()> {
//        //        // Called every frame/tick
//        //        Ok(())
//        //    }
//        //
//        //    pub fn on_shutdown(&self) -> ModResult<()> {
//        //        // Called when the mod is being unloaded
//        //        println!("Shutting down mod: {}", #mod_name);
//        //        Ok(())
//        //    }
//        //}
//        //
//        //// Implement the ModInterface trait
//        //// This will be done by the derive macro, but we add custom wrappers here
//        //impl ModInterface for #struct_name {
//        //    fn init(&self, context: ModContext) -> ModResult<()> {
//        //        self.on_init(&context)
//        //    }
//        //
//        //    fn update(&self, delta_time: f32) -> ModResult<()> {
//        //        self.on_update(delta_time)
//        //    }
//        //
//        //    fn shutdown(&self) -> ModResult<()> {
//        //        self.on_shutdown()
//        //    }
//        //
//        //    fn get_info(&self) -> ModInfo {
//        //        let info = ModInfo {
//        //            id: "mod-123".to_string(),
//        //            name: "Cool Mod".to_string(),
//        //            version: "1.0.0".to_string(),
//        //            author: "Dev".to_string(),
//        //            description: "A test mod".to_string(),
//        //        };
//        //        let json = serde_json::to_vec(&info).unwrap();
//        //        let ptr = allocate(json.len());
//        //        unsafe { std::ptr::copy_nonoverlapping(json.as_ptr(), ptr as *mut u8, json.len()) };
//        //        ptr as i32
//        //    }
//        //}
//        //
//        //// Export the mod interface for WASM
//        //#[wasm_bindgen]
//        //pub fn get_info() -> ModInfo {
//        //    let instance = #struct_name::default();
//        //    instance.get_info()
//        //}
//        //
//        //#[wasm_bindgen]
//        //pub fn init(context: ModContext) -> Result<(), String> {
//        //    let instance = #struct_name::default();
//        //    instance.init(context)
//        //}
//        //
//        //#[wasm_bindgen]
//        //pub fn update(delta_time: f32) -> Result<(), String> {
//        //    let instance = #struct_name::default();
//        //    instance.update(delta_time)
//        //}
//        //
//        //#[wasm_bindgen]
//        //pub fn shutdown() -> Result<(), String> {
//        //    let instance = #struct_name::default();
//        //    instance.shutdown()
//        //}
//    };
//
//    TokenStream::from(expanded)
//}
