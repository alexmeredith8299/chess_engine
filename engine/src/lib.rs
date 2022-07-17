use wasm_bindgen::prelude::*;

//Make alert in JS callable from Rust
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

//Make greet in Rust callable from JS
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello from Rust, {}!", name));
}
