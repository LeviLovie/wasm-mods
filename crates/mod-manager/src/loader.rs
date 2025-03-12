use crate::registry::ModRegistry;
use anyhow::{Context, Result};
use common::{ModContext, ModInfo, ModInterface};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{debug, debug_span, error, info};
use wasm_component_layer::*;

pub struct ModLoader {
    //engine: Engine,
    //registry: Arc<Mutex<ModRegistry>>,
}

impl ModLoader {
    pub fn new(_registry: Arc<Mutex<ModRegistry>>) -> Result<Self> {
        //let engine = Engine::default();
        //Ok(Self { engine, registry })
        Ok(Self {})
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

        let bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let engine = Engine::new(wasmi_runtime_layer::Engine::default());
        let mut store = Store::new(&engine, ());
        let component = match Component::new(&engine, bytes.as_slice()) {
            Ok(component) => component,
            Err(e) => {
                error!("Failed to create component: {}", e);
                return Err(e);
            }
        };
        let mut linker = Linker::default();
        let host_interface = linker
            .define_instance("test:guest/log".try_into().unwrap())
            .unwrap();

        host_interface
            .define_func(
                "log",
                Func::new(
                    &mut store,
                    FuncType::new([ValueType::String], []),
                    move |_, params, _results| {
                        let params = match &params[0] {
                            Value::String(s) => s,
                            _ => panic!("Unexpected parameter type"),
                        };

                        println!("[HostLog] log");
                        println!(" └ {}", params.to_string());
                        Ok(())
                    },
                ),
            )
            .unwrap();

        let instance = linker.instantiate(&mut store, &component).unwrap();
        let interface = instance
            .exports()
            .instance(&"test:guest/foo".try_into().unwrap())
            .unwrap();

        let mut results = vec![Value::Bool(false)];
        let resource_constructor = interface.func("[constructor]bar").unwrap();
        resource_constructor
            .call(&mut store, &[Value::S32(42)], &mut results)
            .unwrap();
        let resource = match results[0] {
            Value::Own(ref resource) => resource.clone(),
            _ => panic!("Unexpected result type"),
        };
        let borrow_res = resource.borrow(store.as_context_mut()).unwrap();
        let arguments = vec![Value::Borrow(borrow_res)];

        let mut results = vec![Value::S32(0)];
        let method_bar_value = interface.func("[method]bar.value").unwrap();
        method_bar_value
            .call(&mut store, &arguments, &mut results)
            .unwrap();
        match results[0] {
            Value::S32(v) => {
                println!("[ResultLog]");
                println!(" └ bar.value() = {}", v);
                assert_eq!(v, 42);
            }
            _ => panic!("Unexpected result type"),
        }

        let mut results = vec![];
        let method_increment = interface.func("[method]bar.increment").unwrap();
        method_increment
            .call(&mut store, &arguments, &mut results)
            .unwrap();

        let mut results = vec![Value::S32(0)];
        let method_bar_value = interface.func("[method]bar.value").unwrap();
        method_bar_value
            .call(&mut store, &arguments, &mut results)
            .unwrap();
        match results[0] {
            Value::S32(v) => {
                println!("[ResultLog]");
                println!(" └ bar.value() = {}", v);
                assert_eq!(v, 43);
            }
            _ => panic!("Unexpected result type"),
        }

        let mod_info = ModInfo::default();

        // Create a mod wrapper that handles the WASM instance
        let mut mod_wrapper = WasmModWrapper {
            _instance: instance,
            //store,
            info: mod_info.clone(),
        };

        match mod_wrapper.call_info() {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to call info function: {}", e);
            }
        }

        // Register the mod
        //let mut registry = self.registry.lock().unwrap();
        //registry.register_mod(&mod_info.id, Box::new(mod_wrapper))?;

        info!("Loaded mod: {}", mod_info.name);
        Ok(mod_info)
    }

    pub fn unload_mod(&self, _mod_id: &str) -> Result<()> {
        //let mut registry = self.registry.lock().unwrap();
        //
        //// Get the mod and call shutdown before unregistering
        //if let Some(mod_instance) = registry.get_mut_mod(mod_id) {
        //    mod_instance.shutdown().expect("Failed to shutdown mod");
        //}
        //
        //registry.unregister_mod(mod_id);
        Ok(())
    }
}

// A wrapper to handle WASM module instances
struct WasmModWrapper {
    _instance: Instance,
    //store: Store<()>,
    info: ModInfo,
}

impl ModInterface for WasmModWrapper {
    fn init(&mut self, _context: ModContext) -> Result<(), String> {
        //let host = Host_::new(&mut self.store, &mut self.instance)
        //    .map_err(|e| format!("Failed to create host binding: {}", e))?;
        //host.call_init(&mut self.store)
        //    .map_err(|e| format!("Failed to call info function: {}", e))?;

        Ok(())
    }

    fn call_info(&mut self) -> Result<(), String> {
        //let host = Host_::new(&mut self.store, &self.instance)
        //    .map_err(|e| format!("Failed to create host binding: {}", e))?;
        //let result = host
        //    .call_info(&mut self.store)
        //    .map_err(|e| format!("Failed to call info function: {}", e))?;
        //println!("Result: {:#?}", result);

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
        //let host = Host_::new(&mut self.store, &self.instance)
        //    .map_err(|e| format!("Failed to create host binding: {}", e))?;
        //host.call_update(&mut self.store)
        //    .map_err(|e| format!("Failed to call update function: {}", e))?;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        //let host = Host_::new(&mut self.store, &self.instance)
        //    .map_err(|e| format!("Failed to create host binding: {}", e))?;
        //host.call_shutdown(&mut self.store)
        //    .map_err(|e| format!("Failed to call shutdown function: {}", e))?;
        Ok(())
    }
}
