wit_bindgen::generate!({
    path: "../../wit/host.wit",
    exports: {
        "test:guest/foo/bar": Component,
    },
});

use crate::exports::test::guest::foo::*;
use crate::test::guest::log::log;
use std::cell::RefCell;

pub struct Component {
    val: RefCell<i32>,
}

impl GuestBar for Component {
    fn new(val: i32) -> Self {
        log(&format!("Creating new Component with value: {}", val));
        Component {
            val: RefCell::new(val),
        }
    }

    fn increment(&self) {
        let mut val = self.val.borrow_mut();
        *val += 1;
        log(&format!("Incremented value to: {}", *val));
    }

    fn value(&self) -> i32 {
        *self.val.borrow()
    }
}
