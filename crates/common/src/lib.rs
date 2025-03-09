use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Define common types that will be shared between the main app and mods
pub mod types {
    use serde_wasm_bindgen::to_value;

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModInfo {
        pub id: String,
        pub name: String,
        pub version: String,
        pub author: String,
        pub description: String,
    }

    impl Default for ModInfo {
        fn default() -> Self {
            ModInfo {
                id: "".to_string(),
                name: "".to_string(),
                version: "".to_string(),
                author: "".to_string(),
                description: "".to_string(),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModContext {
        pub game_version: String,
        pub api_version: String,
    }

    // Define the trait that all mods must implement
    pub trait ModInterface {
        fn call_info(&mut self) -> Result<(), String>;
        fn get_info(&self) -> ModInfo;
        fn init(&mut self, context: ModContext) -> Result<(), String>;
        fn update(&mut self, delta_time: f32) -> Result<(), String>;
        fn shutdown(&mut self) -> Result<(), String>;
    }

    // Define a concrete struct that implements the trait
    #[wasm_bindgen]
    pub struct WasmMod {
        info: ModInfo,
    }

    #[wasm_bindgen]
    impl WasmMod {
        #[wasm_bindgen(constructor)]
        pub fn new(
            id: String,
            name: String,
            version: String,
            author: String,
            description: String,
        ) -> WasmMod {
            WasmMod {
                info: ModInfo {
                    id,
                    name,
                    version,
                    author,
                    description,
                },
            }
        }

        #[wasm_bindgen]
        pub fn get_info(&self) -> JsValue {
            to_value(&self.info).unwrap()
        }

        #[wasm_bindgen]
        pub fn init(&self, _context: JsValue) -> Result<(), JsValue> {
            Ok(())
        }

        #[wasm_bindgen]
        pub fn update(&self, _delta_time: f32) -> Result<(), JsValue> {
            Ok(())
        }

        #[wasm_bindgen]
        pub fn shutdown(&self) -> Result<(), JsValue> {
            Ok(())
        }
    }
}

// Re-export for convenience
pub use types::*;

// Define a mod result type for error handling
pub type ModResult<T> = Result<T, String>;
