use anyhow::{Context, Result};
use common::{ModContext, ModInfo, ModInterface};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{debug, debug_span, error, info};
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

        let mod_info = ModInfo::default();

        // Create a mod wrapper that handles the WASM instance
        let mut mod_wrapper = WasmModWrapper {
            instance,
            store,
            info: mod_info.clone(),
        };

        match mod_wrapper.call_info() {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to call info function: {}", e);
            }
        }

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
        let host = Host_::new(&mut self.store, &mut self.instance)
            .map_err(|e| format!("Failed to create host binding: {}", e))?;
        host.call_init(&mut self.store)
            .map_err(|e| format!("Failed to call info function: {}", e))?;

        Ok(())
    }

    fn call_info(&mut self) -> Result<(), String> {
        let host = Host_::new(&mut self.store, &self.instance)
            .map_err(|e| format!("Failed to create host binding: {}", e))?;
        let result = host
            .call_info(&mut self.store)
            .map_err(|e| format!("Failed to call info function: {}", e))?;
        println!("Result: {:#?}", result);

        self.info = ModInfo {
            //id: result.id,
            id: String::from("test"),
            name: String::from("Test Mod"),
            version: "1.0".to_string(),
            author: "No name".to_string(),
            description: "A mod".to_string(),
            //name: result.name,
            //version: result.version,
            //author: result.author,
            //description: result.description,
        };

        Ok(())
    }

    fn get_info(&self) -> ModInfo {
        self.info.clone()
    }

    fn update(&mut self, _delta_time: f32) -> Result<(), String> {
        let host = Host_::new(&mut self.store, &self.instance)
            .map_err(|e| format!("Failed to create host binding: {}", e))?;
        host.call_update(&mut self.store)
            .map_err(|e| format!("Failed to call update function: {}", e))?;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        let host = Host_::new(&mut self.store, &self.instance)
            .map_err(|e| format!("Failed to create host binding: {}", e))?;
        host.call_shutdown(&mut self.store)
            .map_err(|e| format!("Failed to call shutdown function: {}", e))?;
        Ok(())
    }
}
