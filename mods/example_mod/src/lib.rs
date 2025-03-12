wit_bindgen::generate!({
    path: "../../wit/module.wit",
    exports: {
        "module:guest/events": Events,
        "module:guest/events/data": Data,
    },
});

use crate::exports::module::guest::events::*;
use crate::module::guest::log::log;

pub struct Events {}

impl Guest for Events {
    fn info() -> Vec<String> {
        let version = env!("CARGO_PKG_VERSION").to_string();
        let id = env!("CARGO_PKG_NAME").to_string();
        let authors = env!("CARGO_PKG_AUTHORS");
        let author = authors.split(':').collect::<Vec<&str>>().join(", ");
        let description = env!("CARGO_PKG_DESCRIPTION").to_string();
        let description_parts = description.split(": ").collect::<Vec<&str>>();
        let name = description_parts[0].to_string();
        let description = description_parts[1..].join(": ");

        return vec![id, name, version, author, description];
    }
}

pub struct Data {
    val: RefCell<i32>,
}

use std::cell::RefCell;

impl GuestData for Data {
    fn new() -> Self {
        let val = 42;
        log(&format!("Creating new Component with value: {}", val,));

        Data {
            val: RefCell::new(val),
        }
    }

    fn value(&self) -> i32 {
        let val = self.val.borrow();
        *self.val.borrow()
    }
}
