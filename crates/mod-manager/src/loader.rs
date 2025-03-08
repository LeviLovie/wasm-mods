use anyhow::{Context, Result};
use common::{ModContext, ModInfo, ModInterface};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{info, info_span};
use wasmtime::{Engine, Instance, Store};
use wasmtime_wasi::{add_to_linker_sync, IoView, ResourceTable, WasiCtx, WasiView};

use crate::registry::ModRegistry;

pub struct ComponentRunStates {
    // These two are required basically as a standard way to enable the impl of IoView and
    // WasiView.
    // impl of WasiView is required by [`wasmtime_wasi::add_to_linker_sync`]
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
    // You can add other custom host states if needed
}

impl IoView for ComponentRunStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

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

        let module = wasmtime::Module::new(&self.engine, wasm_bytes)
            .with_context(|| format!("Failed to compile WASM module: {}", path.display()))?;

        let wasi_ctx = wasmtime_wasi::WasiCtxBuilder::new().inherit_stdio().build();
        let resource_table = ResourceTable::new();
        let state = ComponentRunStates {
            wasi_ctx,
            resource_table,
        };

        let mut store = wasmtime::Store::new(&self.engine, state);

        let mut linker = wasmtime::Linker::new(&self.engine);
        add_to_linker_sync(&mut linker).context("Failed to add WASI to linker")?;

        // Instantiate the module
        let instance = linker
            .instantiate(&mut store, &module)
            .with_context(|| format!("Failed to instantiate WASM module: {}", path.display()))?;

        // Rest of your code...
        let mod_info = ModInfo {
            id: "test".to_string(),
            name: "Test Mod".to_string(),
            version: "1.0".to_string(),
            author: "No name".to_string(),
            description: "A mod".to_string(),
        };

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
    store: Store<ComponentRunStates>,
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
