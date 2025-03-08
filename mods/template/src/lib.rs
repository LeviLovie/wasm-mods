#![no_std]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {}

#[wasm_bindgen]
pub fn update() {}

#[wasm_bindgen]
pub fn shutdown() {}

#[wasm_bindgen]
pub fn get_info() -> i32 {
    return 2;
}
