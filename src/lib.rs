extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(name: String) -> String {
    name + "foo"
}