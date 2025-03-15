use super::{ModContext, ModInfo, ModInterface};
use crate::{
    callback::{CallbackRegistry, ModStructure},
    registry::ModRegistry,
};

use anyhow::{Error, Result};
use std::{
    cell::RefCell,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tracing::{debug, debug_span, error_span, info, info_span};
use utils::logging::*;
use wasm_component_layer::*;
use wasmi_runtime_layer::Engine as WasmEngine;

#[derive(Clone)]
pub struct ModLoader {
    engine: Engine<WasmEngine>,
    registry: Arc<Mutex<ModRegistry>>,
    callbacks: Arc<Mutex<CallbackRegistry>>,
}

impl ModLoader {
    pub fn new(registry: Arc<Mutex<ModRegistry>>, callbacks: Arc<Mutex<CallbackRegistry>>) -> Self {
        let engine = Engine::new(WasmEngine::default());

        Self {
            engine,
            registry,
            callbacks,
        }
    }

    pub fn load_mod(&mut self, path: &Path, _context: &ModContext) -> Result<ModInfo, Error> {
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

        let bytes = std::fs::read(path).log_msg("Failed to read file")?;

        let mut store = Store::new(&self.engine, ());
        let component =
            Component::new(&self.engine, bytes.as_slice()).log_msg("Failed to create component")?;
        let mut linker = Linker::default();
        let host_interface = linker
            .define_instance("module:guest/log".try_into().unwrap())
            .log_msg("Failed to define instance")?;

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

                        let span = info_span!("mod_log");
                        let _guard = span.enter();

                        info!("{}", params);
                        Ok(())
                    },
                ),
            )
            .log()?;

        let host_struct_interface = linker
            .define_instance("module:guest/structures".try_into().unwrap())
            .log_msg("Failed to define structures instance")?;
        let callbacks_clone = self.callbacks.clone();
        host_struct_interface
            .define_func(
                "register-structure",
                Func::new(
                    &mut store,
                    FuncType::new(
                        [ValueType::String, ValueType::String, ValueType::String],
                        [ValueType::Bool],
                    ),
                    move |_, params, results| {
                        let id = match &params[0] {
                            Value::String(s) => s.to_string(),
                            _ => panic!("Unexpected parameter type for id"),
                        };

                        let type_name = match &params[1] {
                            Value::String(s) => s.to_string(),
                            _ => panic!("Unexpected parameter type for type_name"),
                        };

                        let data = match &params[2] {
                            Value::String(s) => s.to_string(),
                            _ => panic!("Unexpected parameter type for data"),
                        };

                        let span = debug_span!(
                            "register_structure",
                            id = id.clone(),
                            type_name = type_name.clone()
                        );
                        let _guard = span.enter();

                        // Create a ModStructure that implements Registerable
                        let structure = Box::new(ModStructure {
                            id: id.clone(),
                            type_name: type_name.clone(),
                            data,
                        });

                        let mut reg = callbacks_clone.lock().unwrap();
                        let success = reg.register(structure).is_ok();
                        results[0] = Value::Bool(success);

                        debug!(
                            "Structure registration: {}",
                            if success { "success" } else { "failed" }
                        );
                        Ok(())
                    },
                ),
            )
            .log()?;

        let structure_registry_unregister = self.callbacks.clone();
        host_struct_interface
            .define_func(
                "unregister-structure",
                Func::new(
                    &mut store,
                    FuncType::new([ValueType::String], [ValueType::Bool]),
                    move |_, params, results| {
                        let id = match &params[0] {
                            Value::String(s) => s.to_string(),
                            _ => panic!("Unexpected parameter type for id"),
                        };

                        let span = debug_span!("unregister_structure", id = id.clone());
                        let _guard = span.enter();

                        let mut reg = structure_registry_unregister.lock().unwrap();
                        let success = reg.unregister(&id).is_ok();
                        results[0] = Value::Bool(success);

                        debug!(
                            "Structure unregistration: {}",
                            if success { "success" } else { "failed" }
                        );
                        Ok(())
                    },
                ),
            )
            .log()?;

        let instance = linker.instantiate(&mut store, &component).log()?;
        let mod_info = ModInfo::default();
        let mut mod_wrapper =
            WasmModWrapper::new(store, instance, mod_info.clone(), self.callbacks.clone());
        mod_wrapper.call_info().log()?;
        let mut registry = self.registry.lock().unwrap();
        registry.register_mod(&mod_info.id, Box::new(mod_wrapper))?;

        Ok(mod_info)
    }

    pub fn unload_mod(&self, mod_id: &str) -> Result<(), Error> {
        let span = error_span!("unload_mod", mod_id = mod_id);
        let _guard = span.enter();

        let mut registry = self
            .registry
            .lock()
            .log_msg("Failed to lock registry")
            .unwrap();

        if let Some(mod_instance) = registry.get_mut_mod(mod_id) {
            mod_instance.shutdown().log_msg("Failed to shutdown mod")?;
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
    arguments: Vec<Value>,
    callbacks: Arc<Mutex<CallbackRegistry>>,
}

impl<'a> WasmModWrapper<'a> {
    fn new(
        store: Store<(), WasmEngine>,
        instance: Instance,
        info: ModInfo,
        callbacks: Arc<Mutex<CallbackRegistry>>,
    ) -> Self {
        let instance_rc = Rc::new(instance);

        Self {
            store,
            instance: instance_rc,
            interface_cache: RefCell::new(None),
            info,
            arguments: Vec::new(),
            callbacks,
        }
    }

    fn get_interface(&self) -> &ExportInstance {
        if self.interface_cache.borrow().is_none() {
            let interface = self
                .instance
                .exports()
                .instance(&"module:guest/general".try_into().unwrap())
                .expect("Interface not found");

            *self.interface_cache.borrow_mut() = Some(unsafe { std::mem::transmute(interface) });
        }

        self.interface_cache.borrow().expect("Interface not found")
    }
}

impl<'a> ModInterface for WasmModWrapper<'a> {
    fn init(&mut self, _context: ModContext) -> Result<(), Error> {
        let span = error_span!("init", mod_id = self.info.id.clone());
        let _guard = span.enter();

        let data_constructor = self
            .get_interface()
            .func("[constructor]main")
            .check_log("Unable to get \"main\" constructor from mod")?;
        let data_init = self
            .get_interface()
            .func("[method]main.init")
            .check_log("Unable to get \"main.init\" from mod")?;

        let mut results = vec![Value::Bool(false)];
        data_constructor
            .call(&mut self.store, &[], &mut results)
            .log()?;
        let resource = match results[0] {
            Value::Own(ref resource) => resource.clone(),
            _ => Err(Error::msg("Unexpected result type")).log()?,
        };
        let borrow_res = resource.borrow(self.store.as_context_mut()).log()?;
        let arguments = vec![Value::Borrow(borrow_res)];
        self.arguments = arguments;

        data_init
            .call(&mut self.store, &self.arguments, &mut [])
            .log()?;

        Ok(())
    }

    fn call_info(&mut self) -> Result<(), Error> {
        let span = error_span!("call_info", mod_id = self.info.id.clone());
        let _guard = span.enter();

        let method_info = self
            .get_interface()
            .func("info")
            .check_log("Unable to get \"info\" func from mod")?;
        let mut results = vec![Value::List(
            List::new(
                ListType::new(ValueType::String),
                vec![
                    Value::String("example_mod".into()),
                    Value::String("Example Mod".into()),
                    Value::String("0.1.0".into()),
                    Value::String("Example Author".into()),
                    Value::String("Example Description".into()),
                ],
            )
            .log()?,
        )];
        method_info.call(&mut self.store, &[], &mut results).log()?;
        let result = match &results[0] {
            Value::List(list) => {
                let mut result: Vec<String> = Vec::new();
                for value in list.iter() {
                    result.push(match value {
                        Value::String(str) => (*str).to_string(),
                        _ => panic!("Unexpected list element type"),
                    });
                }
                result
            }
            _ => Err(Error::msg("Unexpected result type")).log()?,
        };
        if result.len() != 5 {
            Err(Error::msg("Unexpected result length")).log()?;
        }

        self.info = ModInfo {
            id: result[0].clone(),
            name: result[1].clone(),
            version: result[2].clone(),
            author: result[3].clone(),
            description: result[4].clone(),
        };

        Ok(())
    }

    fn get_info(&self) -> ModInfo {
        self.info.clone()
    }

    fn update(&mut self, delta_time: f32) -> Result<(), Error> {
        let span = error_span!("update", mod_id = self.info.id.clone());
        let _guard = span.enter();

        let method_data_update = self
            .get_interface()
            .func("[method]main.update")
            .check_log("Unable to get \"main.update\" from mod")?;
        let mut arguments = self.arguments.clone();
        arguments.push(Value::F32(delta_time));
        method_data_update
            .call(&mut self.store, &arguments, &mut [])
            .log()?;

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        let span = error_span!("shutdown", mod_id = self.info.id.clone());
        let _guard = span.enter();

        let method_data_shutdown = self
            .get_interface()
            .func("[method]main.shutdown")
            .check_log("Unable to get \"main.shutdown\" from mod")?;
        method_data_shutdown
            .call(&mut self.store, &self.arguments, &mut [])
            .log()?;

        let mod_id = self.info.id.clone();
        let mut reg = self.callbacks.lock().unwrap();
        reg.cleanup(&mod_id)
            .log_msg("Failed to cleanup callbacks")?;

        Ok(())
    }
}
