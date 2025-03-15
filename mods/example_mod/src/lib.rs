mod_macros::create_mod!("../../wit/module.wit");

use types::Position;

pub struct Main {}

impl GuestMain for Main {
    fn new() -> Self {
        let _ = Position {
            x: 0.0,
            y: 0.0,
            z: 0,
        };

        Main {}
    }

    fn init(&self) {}

    fn update(&self, _: f32) {}

    fn shutdown(&self) {}
}
