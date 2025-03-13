use anyhow::Error;
use std::collections::HashMap;
use tracing::{debug, debug_span, info};
use utils::logging::*;

pub trait Registerable: Send + Sync + 'static {
    fn get_id(&self) -> String;
    fn get_type(&self) -> String;
    #[allow(dead_code)]
    fn get_data(&self) -> String;
}

pub struct CallbackRegistry {
    pub structures: HashMap<String, Box<dyn Registerable>>,
}

impl CallbackRegistry {
    pub fn new() -> Self {
        Self {
            structures: HashMap::new(),
        }
    }

    pub fn register(&mut self, structure: Box<dyn Registerable>) -> Result<(), Error> {
        let id = structure.get_id();
        if self.structures.contains_key(&id) {
            return Err(Error::msg(format!(
                "Structure with id {} already exists",
                id
            )));
        }
        self.structures.insert(id, structure);
        Ok(())
    }

    pub fn unregister(&mut self, id: &str) -> Result<Box<dyn Registerable>, Error> {
        match self.structures.remove(id) {
            Some(structure) => Ok(structure),
            None => Err(Error::msg(format!("Structure with id {} not found", id))),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, id: &str) -> Option<&dyn Registerable> {
        self.structures.get(id).map(|s| s.as_ref())
    }

    #[allow(dead_code)]
    pub fn get_all_of_type(&self, type_name: &str) -> Vec<&dyn Registerable> {
        self.structures
            .values()
            .filter(|s| s.get_type() == type_name)
            .map(|s| s.as_ref())
            .collect()
    }

    pub fn cleanup(&mut self, mod_id: &str) -> Result<(), Error> {
        let span = debug_span!("cleanup", mod_id = mod_id);
        let _guard = span.enter();

        let structures_to_remove: Vec<String> = self
            .structures
            .keys()
            .filter(|k| k.starts_with(&format!("{}:", mod_id)))
            .cloned()
            .collect();

        for id in &structures_to_remove {
            match self.unregister(&id) {
                Ok(_) => debug!("Removed structure: {}", id),
                Err(e) => Err(e).log()?,
            }
        }

        if structures_to_remove.len() > 0 {
            info!(
                "{} structures automatically unloaded",
                structures_to_remove.len()
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ModStructure {
    pub id: String,
    pub type_name: String,
    pub data: String,
}

impl Registerable for ModStructure {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_type(&self) -> String {
        self.type_name.clone()
    }

    fn get_data(&self) -> String {
        self.data.clone()
    }
}
