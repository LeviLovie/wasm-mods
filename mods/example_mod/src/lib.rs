mod_macros::create_mod!("../../wit/module.wit");

pub struct Data {
    val: RefCell<i32>,
}

use std::cell::RefCell;

impl GuestData for Data {
    fn new() -> Self {
        let val = 42;
        log("Creating new Data");

        Data {
            val: RefCell::new(val),
        }
    }

    fn value(&self) -> i32 {
        log("Updating Data");
        *self.val.borrow()
    }
}
