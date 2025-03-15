use super::super::Storages;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use utils::logging::*;
use wasm_component_layer::{Func, FuncType, Linker, Store};
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
            "draw-debug",
            Func::new(
                &mut *store,
                FuncType::new([], []),
                move |_, _params, _results| {
                    let mut storages = storages.lock().unwrap();
                    let textures = &mut storages.textures;
                    textures.add(textures.len() as u32);

                    Ok(())
                },
            ),
        )
        .log()?;

    Ok(())
}
