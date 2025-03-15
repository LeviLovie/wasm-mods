mod_macros::create_mod!("../../wit/module.wit");

pub struct Main {}

impl GuestMain for Main {
    fn new() -> Self {
        Main {}
    }

    fn init(&self) {}

    fn update(&self, _: f32) {}

    fn shutdown(&self) {}
}
