use anyhow::{Error, Result};
use mod_manager::{ModContext, ModManager};
use tracing::{error, info};

fn run() -> Result<(), Error> {
    let context = ModContext {
        game_version: "1.0".to_string(),
        api_version: "1.0".to_string(),
    };
    let mut manager = ModManager::new("wasm", context)?;
    manager.load_all_mods()?;

    let init_instant = std::time::Instant::now();
    manager.call_init()?;
    info!("Initialized in {}us", init_instant.elapsed().as_micros());

    for _ in 0..3 {
        let update_instant = std::time::Instant::now();
        manager.update_all_mods(1000.0 / 16.0)?;
        info!("Updated in {}us", update_instant.elapsed().as_micros());

        let draw_instant = std::time::Instant::now();
        manager.call_draw()?;
        info!("Drawn in {}us", draw_instant.elapsed().as_micros());

        {
            let storages_ref = manager.storages(); // Bind to a variable
            let mut storages = storages_ref.lock().unwrap();
            println!("{:#?}", storages);
            storages.clear();
        }
    }

    let unload_instant = std::time::Instant::now();
    manager.unload_all_mods()?;
    info!("Unloaded in {}us", unload_instant.elapsed().as_micros());

    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();

    match run() {
        Ok(_) => {}
        Err(e) => {
            error!("Error occured: {:?}", e);
        }
    }
}
