mod loader;
mod registry;

use anyhow::{Context, Result};
use common::{ModContext, ModInfo};
use loader::ModLoader;
use registry::ModRegistry;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{error, info, warn};

pub struct ModManager {
    registry: Arc<Mutex<ModRegistry>>,
    loader: ModLoader,
    mods_dir: String,
    context: ModContext,
}

impl ModManager {
    pub fn new(mods_dir: &str, context: ModContext) -> Result<Self> {
        let registry = Arc::new(Mutex::new(ModRegistry::new()));
        let loader = ModLoader::new(Arc::clone(&registry))?;

        Ok(Self {
            registry,
            loader,
            mods_dir: mods_dir.to_string(),
            context,
        })
    }

    pub fn load_all_mods(&mut self) -> Result<()> {
        let mods_path = std::env::current_exe()
            .with_context(|| "Failed to get current executable path")?
            .parent()
            .with_context(|| "Failed to get parent directory of executable")?
            .join(self.mods_dir.clone());
        if !mods_path.exists() {
            warn!("Mods directory doesn't exist: {}", self.mods_dir);
            return Ok(());
        }

        for entry in std::fs::read_dir(mods_path)
            .with_context(|| format!("Failed to read mods directory: {}", self.mods_dir))?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "wasm") {
                self.load_mod(&path)?;
            }
        }

        info!("Loaded {} mods", self.get_mod_count());
        Ok(())
    }

    pub fn load_mod(&mut self, path: &Path) -> Result<ModInfo> {
        let mod_info = self.loader.load_mod(path, &self.context)?;
        info!("Loaded mod: {}", mod_info.name);
        Ok(mod_info)
    }

    pub fn unload_mod(&mut self, mod_id: &str) -> Result<()> {
        self.loader.unload_mod(mod_id)?;
        info!("Unloaded mod: {}", mod_id);
        Ok(())
    }

    pub fn update_all_mods(&mut self, delta_time: f32) -> Result<()> {
        let mut registry = self.registry.lock().unwrap();
        for (mod_id, mod_instance) in registry.get_all_mods_mut() {
            if let Err(err) = mod_instance.update(delta_time) {
                error!("Error updating mod {}: {}", mod_id, err);
            }
        }
        Ok(())
    }

    pub fn get_mod_info(&self, mod_id: &str) -> Option<ModInfo> {
        let registry = self.registry.lock().unwrap();
        registry.get_mod(mod_id).map(|m| m.get_info())
    }

    pub fn get_all_mod_info(&self) -> Vec<ModInfo> {
        let registry = self.registry.lock().unwrap();
        registry
            .get_all_mods()
            .iter()
            .map(|(_, m)| m.get_info())
            .collect()
    }

    pub fn get_mod_count(&self) -> usize {
        let registry = self.registry.lock().unwrap();
        registry.get_all_mods().len()
    }

    pub fn setup_hot_reload(&self) -> Result<()> {
        // Set up file watcher for hot-reloading mods
        // This is a simple implementation - the build.rs script handles the actual rebuilding
        Ok(())
    }
}
