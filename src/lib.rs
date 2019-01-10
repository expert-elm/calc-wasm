use wasm_bindgen::prelude::*;

mod lexer;
mod parser;

use self::parser::Parser;

#[wasm_bindgen]
pub fn calc(name: String) -> String {
    let mut p = Parser::new(&name);
    format!("{}", p.parse())
}