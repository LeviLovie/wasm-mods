mod_macros::create_mod!("../../wit/module.wit");

use std::cell::RefCell;

pub struct Main {
    updates: RefCell<u32>,
}

impl GuestMain for Main {
    fn new() -> Self {
        Main {
            updates: RefCell::new(0),
        }
    }

    fn init(&self) {}

    fn update(&self, _: f32) {
        *self.updates.borrow_mut() += 1;
    }

    fn draw(&self) {
        for _ in 0..*self.updates.borrow() {
            draw_debug();
        }
    }

    fn shutdown(&self) {}
}
