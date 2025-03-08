use anyhow::Error;
use common::ModInterface;
use std::collections::HashMap;
use tracing::{warn, warn_span};

pub struct ModRegistry {
    mods: HashMap<String, Box<dyn ModInterface>>,
}

impl ModRegistry {
    pub fn new() -> Self {
        Self {
            mods: HashMap::new(),
        }
    }

    pub fn register_mod(
        &mut self,
        mod_id: &str,
        mod_instance: Box<dyn ModInterface>,
    ) -> Result<(), Error> {
        let mut current_mod_id = mod_id.to_string();
        if self.mods.contains_key(mod_id) {
            while self.mods.contains_key(&current_mod_id) {
                current_mod_id = format!("{}_{}", mod_id, self.mods.len());
            }

            let span = warn_span!("register_mod", id = mod_id);
            let _guard = span.enter();

            warn!(
                "Mod id {} already exists, new id: {}",
                mod_id, current_mod_id
            );
        }

        self.mods.insert(current_mod_id, mod_instance);
        Ok(())
    }

    pub fn unregister_mod(&mut self, mod_id: &str) -> Option<Box<dyn ModInterface>> {
        self.mods.remove(mod_id)
    }

    pub fn get_mod(&self, mod_id: &str) -> Option<&Box<dyn ModInterface>> {
        self.mods.get(mod_id)
    }

    pub fn get_mut_mod(&mut self, mod_id: &str) -> Option<&mut Box<dyn ModInterface>> {
        self.mods.get_mut(mod_id)
    }

    pub fn get_all_mods(&self) -> &HashMap<String, Box<dyn ModInterface>> {
        &self.mods
    }

    pub fn get_all_mods_mut(&mut self) -> &mut HashMap<String, Box<dyn ModInterface>> {
        &mut self.mods
    }

    pub fn mods_mut_iter(&mut self) -> impl Iterator<Item = (&String, &mut Box<dyn ModInterface>)> {
        self.mods.iter_mut()
    }
}
