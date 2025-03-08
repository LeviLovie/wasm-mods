use anyhow::{Context, Result};
use common::{ModContext, ModInfo, ModInterface};
use std::path::Path;
use std::sync::{Arc, Mutex};
use wasmtime::{Engine, Instance, Module, Store};

use crate::registry::ModRegistry;

pub struct ModLoader {
    engine: Engine,
    registry: Arc<Mutex<ModRegistry>>,
}

impl ModLoader {
    pub fn new(registry: Arc<Mutex<ModRegistry>>) -> Result<Self> {
        let engine = Engine::default();
        Ok(Self { engine, registry })
    }

    pub fn load_mod(&self, path: &Path, _context: &ModContext) -> Result<ModInfo> {
        let wasm_bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read WASM file: {}", path.display()))?;

        let module = Module::new(&self.engine, wasm_bytes)
            .with_context(|| format!("Failed to compile WASM module: {}", path.display()))?;

        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, &module, &[])
            .with_context(|| format!("Failed to instantiate WASM module: {}", path.display()))?;

        // Extract the ModInterface implementation
        // This is a simplified version - in reality you'd need to use wasm-bindgen
        // to properly communicate between the host and WASM modules
        let get_info = instance.get_typed_func::<(), i32>(&mut store, "get_info")?;
        let ptr = get_info.call(&mut store, ())?;

        let memory = instance.get_memory(&mut store, "memory").unwrap();
        let data = memory.data(&store);
        let json_bytes = &data[ptr as usize..ptr as usize + 256]; // Assume max 256 bytes
        let mod_info: ModInfo = serde_json::from_slice(json_bytes).unwrap();

        // Initialize the mod
        let init = instance.get_typed_func::<(), ()>(&mut store, "init")?;
        init.call(&mut store, ())
            .with_context(|| format!("Failed to initialize mod: {}", mod_info.name))?;

        // Create a mod wrapper that handles the WASM instance
        let mod_wrapper = WasmModWrapper {
            instance,
            store,
            info: mod_info.clone(),
        };

        // Register the mod
        let mut registry = self.registry.lock().unwrap();
        registry.register_mod(&mod_info.id, Box::new(mod_wrapper));

        Ok(mod_info)
    }

    pub fn unload_mod(&self, mod_id: &str) -> Result<()> {
        let mut registry = self.registry.lock().unwrap();

        // Get the mod and call shutdown before unregistering
        if let Some(mod_instance) = registry.get_mut_mod(mod_id) {
            mod_instance.shutdown().expect("Failed to shutdown mod");
        }

        registry.unregister_mod(mod_id);
        Ok(())
    }
}

// A wrapper to handle WASM module instances
struct WasmModWrapper {
    instance: Instance,
    store: Store<()>,
    info: ModInfo,
}

impl ModInterface for WasmModWrapper {
    fn init(&mut self, _context: ModContext) -> Result<(), String> {
        let init = match self
            .instance
            .get_typed_func::<(), ()>(&mut self.store, "init")
        {
            Ok(init) => init,
            Err(e) => return Err(format!("Failed to get init function: {}", e)),
        };
        match init
            .call(&mut self.store, ())
            .with_context(|| format!("Failed to initialize mod: {}", self.info.name))
        {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to initialize mod: {}", e)),
        }

        Ok(())
    }

    fn get_info(&self) -> ModInfo {
        self.info.clone()
    }

    fn update(&mut self, delta_time: f32) -> Result<(), String> {
        let update = self
            .instance
            .get_typed_func::<f32, ()>(&mut self.store, "update")
            .map_err(|e| format!("Failed to get update function: {}", e))?;

        update
            .call(&mut self.store, delta_time)
            .map_err(|e| format!("Failed to update mod: {}", e))
    }

    fn shutdown(&mut self) -> Result<(), String> {
        let shutdown = self
            .instance
            .get_typed_func::<(), ()>(&mut self.store, "shutdown")
            .map_err(|e| format!("Failed to get shutdown function: {}", e))?;

        shutdown
            .call(&mut self.store, ())
            .map_err(|e| format!("Failed to shutdown mod: {}", e))
    }
}
