use parser::Parser;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

extern crate log;

pub mod parser;
pub mod symbol;
pub mod lexer;
pub mod query;

use crate::lexer::Scanner;


#[wasm_bindgen]
pub fn lexer(message: &str) -> Result<JsValue,JsValue> {
    // let message = "select beautiful_girls from Computer_engineering where sem = 1;";
    let mut s = Scanner::new(message);
    let tokens = s.scan_tokens().unwrap();

    Ok(serde_wasm_bindgen::to_value(&tokens)?)

} 

#[wasm_bindgen]
pub fn parser(message: &str) -> Result<JsValue, JsValue> {
    let parsed_values = Parser::new(message).unwrap();

    Ok(serde_wasm_bindgen::to_value(&parsed_values.parse().unwrap())?)
}

