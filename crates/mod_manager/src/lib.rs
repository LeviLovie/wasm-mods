mod loader;
mod mod_context;
mod registry;
pub use mod_context::{ModContext, ModInfo, ModInterface};

use anyhow::{Context, Error, Result};
use loader::ModLoader;
use registry::ModRegistry;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use tracing::{debug, debug_span, error_span, info, warn};
use utils::logging::*;

pub struct ModManager {
    registry: Arc<Mutex<ModRegistry>>,
    loader: ModLoader,
    mods_dir: String,
    context: ModContext,
}

impl ModManager {
    pub fn new(mods_dir: &str, context: ModContext) -> Result<Self, Error> {
        let registry = Arc::new(Mutex::new(ModRegistry::new()));
        let loader = ModLoader::new(Arc::clone(&registry));

        Ok(Self {
            registry,
            loader,
            mods_dir: mods_dir.to_string(),
            context,
        })
    }

    pub fn load_all_mods(&mut self) -> Result<()> {
        let span = debug_span!("load_all_mods");
        let _guard = span.enter();
        let start_instant = std::time::Instant::now();

        let mods_path = std::env::current_exe()
            .with_context(|| "Failed to get current executable path")?
            .parent()
            .with_context(|| "Failed to get parent directory of executable")?
            .join(self.mods_dir.clone());
        debug!("WASM directory: {}", mods_path.display());
        if !mods_path.exists() {
            warn!("WASM directory doesn't exist: {}", self.mods_dir);
            return Ok(());
        }

        for entry in std::fs::read_dir(mods_path).log()? {
            let path = entry.log()?.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "wasm") {
                self.load_mod(&path)?;
            }
        }

        info!(
            "Loaded {} mods in {}ms",
            self.get_mod_count(),
            (start_instant.elapsed().as_micros() / 100) as f32 / 10.0
        );
        Ok(())
    }

    pub fn load_mod(&mut self, path: &Path) -> Result<ModInfo, Error> {
        let span = error_span!("load_mod", file = path.display().to_string());
        let _guard = span.enter();

        self.loader
            .load_mod(path, &self.context)
            .log_msg("Failed to load mod")
    }

    pub fn unload_mod(&mut self, mod_id: &str) -> Result<()> {
        self.loader.unload_mod(mod_id)?;

        Ok(())
    }

    pub fn update_all_mods(&mut self, delta_time: f32) -> Result<()> {
        let span = error_span!("update_all_mods");
        let _guard = span.enter();

        let mut registry = self.registry.lock().unwrap();
        for (_id, mod_instance) in registry.get_all_mods_mut() {
            mod_instance.update(delta_time).log()?;
        }

        Ok(())
    }

    pub fn get_mod_info(&self, mod_id: &str) -> Result<ModInfo, Error> {
        let span = error_span!("get_mod_info", mod_id = mod_id);
        let _guard = span.enter();

        let registry = self.registry.lock().unwrap();
        let mod_instance: &Box<dyn ModInterface> =
            registry.get_mod(mod_id).check_log("Mod not found")?;
        Ok(mod_instance.as_ref().get_info())
    }

    pub fn get_all_mod_info(&mut self) -> Vec<ModInfo> {
        let mut registry = self.registry.lock().unwrap();
        let mut mod_infos = Vec::new();
        for (_, mod_instance) in registry.mods_mut_iter() {
            let info = mod_instance.get_info();
            mod_infos.push(info);
        }
        mod_infos
    }

    pub fn get_mod_count(&self) -> usize {
        let registry = self.registry.lock().unwrap();
        registry.get_all_mods().len()
    }

    pub fn call_init(&mut self) -> Result<()> {
        let span = error_span!("call_init");
        let _guard = span.enter();

        let mut registry = self.registry.lock().unwrap();
        for (_, mod_instance) in registry.mods_mut_iter() {
            mod_instance
                .init(self.context.clone())
                .log_msg("Failed to init mod")?;
        }
        Ok(())
    }
}
