use super::{ModContext, ModInfo, ModInterface};
use crate::registry::ModRegistry;
use anyhow::{Context, Result};
use std::{
    cell::RefCell,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tracing::{debug, debug_span, error};
use wasm_component_layer::*;
use wasmi_runtime_layer::Engine as WasmEngine;

pub struct ModLoader {
    engine: Engine<WasmEngine>,
    registry: Arc<Mutex<ModRegistry>>,
}

impl ModLoader {
    pub fn new(registry: Arc<Mutex<ModRegistry>>) -> Result<Self> {
        let engine = Engine::new(WasmEngine::default());
        Ok(Self { engine, registry })
    }

    pub fn load_mod(&mut self, path: &Path, _context: &ModContext) -> Result<ModInfo> {
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

        let mut store = Store::new(&self.engine, ());
        let component = match Component::new(&self.engine, bytes.as_slice()) {
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
        let mod_info = ModInfo::default();
        let mod_wrapper = WasmModWrapper::new(store, instance, mod_info.clone());
        let mut registry = self.registry.lock().unwrap();
        registry.register_mod(&mod_info.id, Box::new(mod_wrapper))?;

        Ok(mod_info)
    }

    pub fn unload_mod(&self, mod_id: &str) -> Result<()> {
        let mut registry = self.registry.lock().unwrap();

        if let Some(mod_instance) = registry.get_mut_mod(mod_id) {
            mod_instance.shutdown().expect("Failed to shutdown mod");
        }
        registry.unregister_mod(mod_id);

        Ok(())
    }
}

struct WasmModWrapper<'a> {
    store: Store<(), WasmEngine>,
    instance: Rc<Instance>,
    interface_cache: RefCell<Option<&'a ExportInstance>>,
    info: ModInfo,
}

impl<'a> WasmModWrapper<'a> {
    fn new(store: Store<(), WasmEngine>, instance: Instance, info: ModInfo) -> Self {
        let instance_rc = Rc::new(instance);

        Self {
            store,
            instance: instance_rc,
            interface_cache: RefCell::new(None),
            info,
        }
    }

    fn get_interface(&self) -> &ExportInstance {
        if self.interface_cache.borrow().is_none() {
            let interface = self
                .instance
                .exports()
                .instance(&"test:guest/foo".try_into().unwrap())
                .unwrap();

            *self.interface_cache.borrow_mut() = Some(unsafe { std::mem::transmute(interface) });
        }

        self.interface_cache.borrow().unwrap()
    }
}

impl<'a> ModInterface for WasmModWrapper<'a> {
    fn init(&mut self, _context: ModContext) -> Result<(), String> {
        let resource_constructor = self.get_interface().func("[constructor]bar").unwrap();
        let method_bar_value = self.get_interface().func("[method]bar.value").unwrap();
        let method_increment = self.get_interface().func("[method]bar.increment").unwrap();

        let mut results = vec![Value::Bool(false)];
        resource_constructor
            .call(&mut self.store, &[Value::S32(42)], &mut results)
            .unwrap();
        let resource = match results[0] {
            Value::Own(ref resource) => resource.clone(),
            _ => panic!("Unexpected result type"),
        };
        let borrow_res = resource.borrow(self.store.as_context_mut()).unwrap();
        let arguments = vec![Value::Borrow(borrow_res)];

        let mut results = vec![Value::S32(0)];
        method_bar_value
            .call(&mut self.store, &arguments, &mut results)
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
        method_increment
            .call(&mut self.store, &arguments, &mut results)
            .unwrap();

        let mut results = vec![Value::S32(0)];
        method_bar_value
            .call(&mut self.store, &arguments, &mut results)
            .unwrap();
        match results[0] {
            Value::S32(v) => {
                println!("[ResultLog]");
                println!(" └ bar.value() = {}", v);
                assert_eq!(v, 43);
            }
            _ => panic!("Unexpected result type"),
        }

        Ok(())
    }

    fn call_info(&mut self) -> Result<(), String> {
        self.info = ModInfo {
            id: String::from("test"),
            name: String::from("Test Mod"),
            version: "1.0".to_string(),
            author: "No name".to_string(),
            description: "A mod".to_string(),
        };

        Ok(())
    }

    fn get_info(&self) -> ModInfo {
        self.info.clone()
    }

    fn update(&mut self, _delta_time: f32) -> Result<(), String> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }
}
