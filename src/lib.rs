extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: String) -> String {
    name + "foo"
}

fn main() {
    println!("Hello, world!, {}", greet("bar".to_string()));
}