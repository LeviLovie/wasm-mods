use anyhow::Error;

#[derive(Debug, Clone)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

impl Default for ModInfo {
    fn default() -> Self {
        ModInfo {
            id: "".to_string(),
            name: "".to_string(),
            version: "".to_string(),
            author: "".to_string(),
            description: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModContext {
    pub game_version: String,
    pub api_version: String,
}

pub trait ModInterface {
    fn call_info(&mut self) -> Result<(), Error>;
    fn get_info(&self) -> ModInfo;
    fn init(&mut self, context: ModContext) -> Result<(), Error>;
    fn update(&mut self, delta_time: f32) -> Result<(), Error>;
    fn draw(&mut self) -> Result<(), Error>;
    fn shutdown(&mut self) -> Result<(), Error>;
}
