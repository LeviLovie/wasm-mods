wit_bindgen::generate!({
    path: "../../wit/host.wit",
    exports: {
        "test:guest/foo/bar": Component,
    },
});

use crate::exports::test::guest::foo::*;
use crate::test::guest::log::log;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct CustomData {
    value: i32,
}

pub struct Component {
    val: RefCell<i32>,
    data: RefCell<CustomData>,
}

impl GuestBar for Component {
    fn new(val: i32) -> Self {
        let data = CustomData { value: 5 };
        log(&format!(
            "Creating new Component with value: {}, data: {:?}",
            val,
            data.clone()
        ));

        Component {
            val: RefCell::new(val),
            data: RefCell::new(data),
        }
    }

    fn increment(&self) {
        let mut val = self.val.borrow_mut();
        *val += 1;
        let mut data = self.data.borrow_mut();
        data.value -= 1;
        log(&format!(
            "Incremented value to: {}, data: {:?}",
            *val,
            data.clone()
        ));
    }

    fn value(&self) -> i32 {
        let val = self.val.borrow();
        let data = self.data.borrow();
        log(&format!(
            "Returning value: {}, data: {:?}",
            *val,
            data.clone()
        ));
        *self.val.borrow()
    }
}
