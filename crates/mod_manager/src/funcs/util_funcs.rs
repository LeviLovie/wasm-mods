use super::super::Storages;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use tracing::{error, error_span, info, info_span};
use utils::logging::*;
use wasm_component_layer::{Func, FuncType, Linker, Store, Value, ValueType};
use wasmi_runtime_layer::Engine as WasmEngine;

pub fn register(
    linker: &mut Linker,
    store: &mut Store<(), WasmEngine>,
    _storages: Arc<Mutex<Storages>>,
) -> Result<()> {
    let interface = linker
        .define_instance("module:guest/utils".try_into().unwrap())
        .log_msg("Failed to define instance")?;

    interface
        .define_func(
            "log",
            Func::new(
                &mut *store,
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

    interface
        .define_func(
            "fatal",
            Func::new(
                &mut *store,
                FuncType::new([ValueType::String], []),
                move |_, params, _results| {
                    let params = match &params[0] {
                        Value::String(s) => s,
                        _ => panic!("Unexpected parameter type"),
                    };

                    let span = error_span!("mod_log");
                    let _guard = span.enter();
                    error!("{}", params);

                    Ok(())
                },
            ),
        )
        .log()?;

    Ok(())
}
