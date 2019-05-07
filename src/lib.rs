extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]               //'attribute'wasm_bindgen knows how to find extern
// whenever y want to call new js functions put them here under the wasm_bingen umbrella
// "calling javascript alert! calling javascript alert!"
extern {
    pub fn alert(s: &str);   // alert is a function provided by js
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
//in this case wasm_bindgen is not modifying an extern block but a Rust fn 'greet'
//this means we want this function to be able to be called by js
//"greet, javascript is calling you, go! go! go!"