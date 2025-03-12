mod_macros::create_mod!("../../wit/module.wit");

pub struct Data {
    val: RefCell<i32>,
}

use std::cell::RefCell;

impl GuestData for Data {
    fn new() -> Self {
        let val = 42;
        log(&format!("Creating new Component with value: {}", val));

        Data {
            val: RefCell::new(val),
        }
    }

    fn value(&self) -> i32 {
        *self.val.borrow()
    }
}
