use common::ModInterface;
use std::collections::HashMap;

pub struct ModRegistry {
    mods: HashMap<String, Box<dyn ModInterface>>,
}

impl ModRegistry {
    pub fn new() -> Self {
        Self {
            mods: HashMap::new(),
        }
    }

    pub fn register_mod(&mut self, mod_id: &str, mod_instance: Box<dyn ModInterface>) {
        self.mods.insert(mod_id.to_string(), mod_instance);
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
}
