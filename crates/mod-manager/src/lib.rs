mod loader;
mod mod_context;
mod registry;

pub use mod_context::{ModContext, ModInfo, ModInterface};

use anyhow::{Context, Result};
use loader::ModLoader;
use registry::ModRegistry;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{debug, debug_span, error, error_span, info, warn};

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

        for entry in std::fs::read_dir(mods_path)
            .with_context(|| format!("Failed to read mods directory: {}", self.mods_dir))?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "wasm") {
                self.load_mod(&path);
            }
        }

        info!(
            "Loaded {} mods in {}ms",
            self.get_mod_count(),
            (start_instant.elapsed().as_micros() / 100) as f32 / 10.0
        );
        Ok(())
    }

    pub fn load_mod(&mut self, path: &Path) -> ModInfo {
        match self.loader.load_mod(path, &self.context) {
            Ok(info) => info,
            Err(err) => {
                error!("Error loading mod: {}", err);
                panic!();
            }
        }
    }

    pub fn unload_mod(&mut self, mod_id: &str) -> Result<()> {
        self.loader.unload_mod(mod_id)?;
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

    pub fn get_mod_info(&self, mod_id: &str) -> ModInfo {
        let registry = self.registry.lock().unwrap();
        let mod_instance: &Box<dyn ModInterface> = registry
            .get_mod(mod_id)
            .ok_or("Mod not found")
            .expect("Mod not found");
        mod_instance.as_ref().get_info()
    }

    pub fn get_all_mod_info(&mut self) -> Vec<ModInfo> {
        let span = error_span!("get_all_mod_info");
        let _guard = span.enter();

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
        let mut registry = self.registry.lock().unwrap();
        for (_, mod_instance) in registry.mods_mut_iter() {
            match mod_instance.init(self.context.clone()) {
                Ok(_) => {}
                Err(err) => {
                    error!("Error initializing mod: {}", err);
                }
            }
        }
        Ok(())
    }
}
