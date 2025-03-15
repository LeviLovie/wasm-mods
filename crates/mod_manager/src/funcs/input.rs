use super::super::Storages;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use utils::logging::*;
use wasm_component_layer::{Func, FuncType, Linker, Store, Tuple, TupleType, Value, ValueType};
use wasmi_runtime_layer::Engine as WasmEngine;

pub fn register(
    linker: &mut Linker,
    store: &mut Store<(), WasmEngine>,
    storages: Arc<Mutex<Storages>>,
) -> Result<()> {
    let interface = linker
        .define_instance("module:guest/input".try_into().unwrap())
        .log_msg("Failed to define instance")?;

    let storages_clone = storages.clone();
    interface
        .define_func(
            "get-window-size",
            Func::new(
                &mut *store,
                FuncType::new(
                    [],
                    [ValueType::Tuple(TupleType::new(
                        None,
                        vec![ValueType::F32, ValueType::F32],
                    ))],
                ),
                move |_, _params, results| {
                    let window_size = {
                        let mut storages = storages_clone.lock().unwrap();
                        let window_size = &mut storages.window_size;
                        *window_size.get()
                    };

                    results[0] = Value::Tuple(
                        Tuple::new(
                            TupleType::new(None, vec![ValueType::F32, ValueType::F32]),
                            vec![
                                Value::F32(window_size.0 as f32),
                                Value::F32(window_size.1 as f32),
                            ],
                        )
                        .expect("Failed to create tuple"),
                    );

                    Ok(())
                },
            ),
        )
        .log()?;

    Ok(())
}
