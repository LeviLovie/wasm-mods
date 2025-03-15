pub mod storage;
pub mod util_funcs;

use super::Storages;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use utils::logging::*;
use wasm_component_layer::{Linker, Store};
use wasmi_runtime_layer::Engine as WasmEngine;

pub fn register(
    linker: &mut Linker,
    store: &mut Store<(), WasmEngine>,
    storages: Arc<Mutex<Storages>>,
) -> Result<()> {
    storage::register(linker, store, storages.clone())
        .log_msg("Failed to register storage funcs")?;
    util_funcs::register(linker, store, storages.clone())
        .log_msg("Failed to register utils funcs")?;
    Ok(())
}
