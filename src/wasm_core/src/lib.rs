use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-pack!");
}

#[wasm_bindgen]
pub fn add(x: f64, y: f64) -> f64 {
    x + y
}
