use anyhow::Error;
use std::collections::HashMap;

pub trait Registerable: Send + Sync + 'static {
    fn get_id(&self) -> String;
    fn get_type(&self) -> String;
}

// Add a registry for structures within ModLoader
pub struct CallbackRegistry {
    structures: HashMap<String, Box<dyn Registerable>>,
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

    pub fn get(&self, id: &str) -> Option<&dyn Registerable> {
        self.structures.get(id).map(|s| s.as_ref())
    }

    pub fn get_all_of_type(&self, type_name: &str) -> Vec<&dyn Registerable> {
        self.structures
            .values()
            .filter(|s| s.get_type() == type_name)
            .map(|s| s.as_ref())
            .collect()
    }
}
