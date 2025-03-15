use super::super::Storages;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use utils::logging::*;
use wasm_component_layer::{Func, FuncType, Linker, Store, Value, ValueType};
use wasmi_runtime_layer::Engine as WasmEngine;

pub fn register(
    linker: &mut Linker,
    store: &mut Store<(), WasmEngine>,
    storages: Arc<Mutex<Storages>>,
) -> Result<()> {
    let interface = linker
        .define_instance("module:guest/graphics".try_into().unwrap())
        .log_msg("Failed to define instance")?;

    interface
        .define_func(
            "draw-rect",
            Func::new(
                &mut *store,
                FuncType::new(
                    [
                        ValueType::F32,
                        ValueType::F32,
                        ValueType::F32,
                        ValueType::F32,
                    ],
                    [],
                ),
                move |_, params, _results| {
                    let x = match params[0] {
                        Value::F32(x) => x as u32,
                        _ => panic!("Unexpected parameter type"),
                    };
                    let y = match params[1] {
                        Value::F32(y) => y as u32,
                        _ => panic!("Unexpected parameter type"),
                    };
                    let w = match params[2] {
                        Value::F32(w) => w as u32,
                        _ => panic!("Unexpected parameter type"),
                    };
                    let h = match params[3] {
                        Value::F32(h) => h as u32,
                        _ => panic!("Unexpected parameter type"),
                    };

                    {
                        let mut storages = storages.lock().unwrap();
                        let textures = &mut storages.textures;
                        textures.add((x, y, w, h));
                    }

                    Ok(())
                },
            ),
        )
        .log()?;

    Ok(())
}
