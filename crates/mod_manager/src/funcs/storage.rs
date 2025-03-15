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
    Ok(())
}
