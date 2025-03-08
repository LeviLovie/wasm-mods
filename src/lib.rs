use anyhow::{Error, Result};
use common::ModContext;
use mod_manager::ModManager;

pub fn run() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let context = ModContext {
        game_version: "1.0".to_string(),
        api_version: "1.0".to_string(),
    };
    let mut manager = ModManager::new("wasm", context)?;
    manager.load_all_mods()?;
    println!("Loaded {} mods", manager.get_mod_count());
    //let info = manager.get_all_mod_info();
    //for mod_info in info {
    //    println!("Loaded mod: {}", mod_info.name);
    //}
    Ok(())
}
