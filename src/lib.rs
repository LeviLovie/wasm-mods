use anyhow::{Error, Result};
use common::ModContext;
use mod_manager::ModManager;
use tracing::info;

pub fn run() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let load_instant = std::time::Instant::now();
    let context = ModContext {
        game_version: "1.0".to_string(),
        api_version: "1.0".to_string(),
    };
    let mut manager = ModManager::new("wasm", context)?;
    manager.load_all_mods()?;
    info!(
        "Loaded in {:?}ms",
        (load_instant.elapsed().as_micros() / 100) as f32 / 10.0
    );
    Ok(())
}
