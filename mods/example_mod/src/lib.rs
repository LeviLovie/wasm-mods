mod_macros::create_mod!("../../wit/module.wit");

pub struct Data {}

impl GuestData for Data {
    fn new() -> Self {
        log("Constructing");
        Data {}
    }

    fn init(&self) {
        log("Initializing");

        let mod_id = env!("CARGO_PKG_NAME");
        let structure_id = format!("{}:my_structure", mod_id);
        let structure_type = "game_entity";
        let structure_data = r#"{"position": {"x": 100, "y": 200}, "name": "Player"}"#;

        let success = register_structure(&structure_id, structure_type, structure_data);
        log(&format!(
            "Structure registration: {}",
            if success { "success" } else { "failed" }
        ));
    }

    fn update(&self, delta: f32) {
        log(&format!("Updating: {}ms", delta));
    }

    fn shutdown(&self) {
        log("Shutting down");

        let mod_id = env!("CARGO_PKG_NAME");
        let structure_id = format!("{}:my_structure", mod_id);
        let success = unregister_structure(&structure_id);
        log(&format!(
            "Structure unregistration: {}",
            if success { "success" } else { "failed" }
        ));
    }
}
