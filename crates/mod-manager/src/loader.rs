use anyhow::{Context, Result};
use common::{ModContext, ModInfo, ModInterface};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{debug, debug_span, info};
use wasmtime::{
    component::{Component, Instance, Linker},
    Engine, Store,
};
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiView};

use crate::registry::ModRegistry;

pub struct ComponentRunStates {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
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

wasmtime::component::bindgen!("host" in "../../wit/host.wit");

impl Host_Imports for ComponentRunStates {
    fn print(&mut self, msg: String) -> () {
        println!("{}", msg);
        ()
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
        let span = debug_span!(
            "load_mod",
            file = path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
        );
        let _guard = span.enter();
        debug!("Loading mod: {}", path.display());

        let mut store = Store::new(
            &self.engine,
            ComponentRunStates {
                wasi_ctx: WasiCtx::builder().build(),
                resource_table: ResourceTable::new(),
            },
        );

        let wasm_bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read WASM file: {}", path.display()))?;
        let main_component = Component::new(&self.engine, &wasm_bytes).with_context(|| {
            format!(
                "Failed to create component from WASM file: {}",
                path.display()
            )
        })?;

        let mut linker = Linker::<ComponentRunStates>::new(&self.engine);
        Host_::add_to_linker(&mut linker, |state| state)?;

        let instance = linker
            .instantiate(&mut store, &main_component)
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
        registry.register_mod(&mod_info.id, Box::new(mod_wrapper))?;

        info!("Loaded mod: {}", mod_info.name);
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
        // Call MyHost::run from exported function
        let run = self
            .instance
            .get_typed_func::<(), (u32,)>(&mut self.store, "run")
            .with_context(|| format!("Failed to get run function: {}", self.info.name))
            .map_err(|e| format!("Failed to get run function: {}", e))?;
        let result = run
            .call(&mut self.store, ())
            .with_context(|| format!("Failed to run mod: {}", self.info.name))
            .map_err(|e| format!("Failed to run mod: {}", e))?;
        println!("Result: {}", result.0);

        Ok(())
    }

    fn get_info(&self) -> ModInfo {
        self.info.clone()
    }

    fn update(&mut self, _delta_time: f32) -> Result<(), String> {
        //let update = self
        //    .instance
        //    .get_typed_func::<f32, ()>(&mut self.store, "update")
        //    .map_err(|e| format!("Failed to get update function: {}", e))?;
        //
        //update
        //    .call(&mut self.store, delta_time)
        //    .map_err(|e| format!("Failed to update mod: {}", e))

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        //let shutdown = self
        //    .instance
        //    .get_typed_func::<(), ()>(&mut self.store, "shutdown")
        //    .map_err(|e| format!("Failed to get shutdown function: {}", e))?;
        //
        //shutdown
        //    .call(&mut self.store, ())
        //    .map_err(|e| format!("Failed to shutdown mod: {}", e))

        Ok(())
    }
}
