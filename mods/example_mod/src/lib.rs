mod_macros::create_mod!("../../wit/module.wit");

pub struct Data {}

impl GuestData for Data {
    fn new() -> Self {
        log("Constructing");
        Data {}
    }

    fn init(&self) {
        log("Inititalizing");
    }

    fn update(&self, delta: f32) {
        log(&format!("Updating: {}ms", delta));
    }

    fn shutdown(&self) {
        log("Shutting down");
    }
}
