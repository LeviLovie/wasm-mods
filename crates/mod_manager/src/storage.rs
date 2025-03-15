#[derive(Debug)]
pub struct ScalStorage<T: Default> {
    value: T,
}

impl<T: Default> ScalStorage<T> {
    pub fn new() -> Self {
        Self {
            value: T::default(),
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn clear(&mut self) {
        self.value = T::default();
    }
}

#[derive(Debug)]
pub struct VecStorage<T> {
    values: Vec<T>,
}

impl<T> VecStorage<T> {
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

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.values.iter_mut()
    }
}

#[derive(Debug)]
pub struct Storages {
    pub textures: VecStorage<((u32, u32, u32, u32), (u8, u8, u8, u8))>,
    pub color: ScalStorage<(u8, u8, u8, u8)>,
    pub window_size: ScalStorage<(u32, u32)>,
}

impl Storages {
    pub fn new() -> Self {
        Self {
            textures: VecStorage::new(),
            color: ScalStorage::new(),
            window_size: ScalStorage::new(),
        }
    }

    pub fn clear(&mut self, window_size: (u32, u32)) {
        self.textures.clear();
        self.color = ScalStorage::new();
        self.window_size.set(window_size);
    }
}
