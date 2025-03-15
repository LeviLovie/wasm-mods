mod_macros::create_mod!("../../wit/module.wit");

use std::cell::RefCell;

pub struct Main {
    position: RefCell<(f32, f32)>,
    reverse: RefCell<(bool, bool)>,
    size: (f32, f32),
}

impl GuestMain for Main {
    fn new() -> Self {
        Main {
            position: RefCell::new((0.0, 0.0)),
            reverse: RefCell::new((false, false)),
            size: (80.0, 60.0),
        }
    }

    fn init(&self) {}

    fn update(&self, _: f32) {
        let mut position = self.position.borrow_mut();
        let mut reverse = self.reverse.borrow_mut();
        let size = self.size;

        position.0 += if reverse.0 { -1.0 } else { 1.0 };
        position.1 += if reverse.1 { -1.0 } else { 1.0 };

        if position.0 >= 800.0 - size.0 {
            reverse.0 = true;
        } else if position.0 <= 0.0 {
            reverse.0 = false;
        }

        if position.1 >= 600.0 - size.1 {
            reverse.1 = true;
        } else if position.1 <= 0.0 {
            reverse.1 = false;
        }
    }

    fn draw(&self) {
        let position = self.position.borrow();
        let size = self.size;
        color(
            position.0 / 800.0,
            position.1 / 600.0,
            1.0 - position.0 / 800.0,
            1.0,
        );
        draw_rect(position.0, position.1, size.0, size.1);
    }

    fn shutdown(&self) {}
}
