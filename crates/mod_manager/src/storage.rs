#[derive(Debug)]
pub struct Storage<T> {
    values: Vec<T>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn add(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug)]
pub struct Storages {
    pub textures: Storage<u32>,
}

impl Storages {
    pub fn new() -> Self {
        Self {
            textures: Storage::new(),
        }
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }
}
