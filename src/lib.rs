mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Array;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn get_legal_moves(fen:&str) -> Vec<String> {
    let moves = Vec::from(["e4".to_string(), "e5".to_string()]);
    return moves;
}

#[wasm_bindgen]
pub fn make_move(fen: &str) -> String {
    return "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R".to_string();
}

#[wasm_bindgen]
pub fn check_if_legal(old_fen: &str, source: &str, target: &str, piece: &str) -> bool {
    return true
}

#[wasm_bindgen]
pub fn update(old_fen: &str, source: &str, target: &str, piece: &str) -> String {
    return "r1bkqbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R".to_string();
}

#[wasm_bindgen]
pub fn get_legal_moves_for_highlighting(fen: &str) -> js_sys::Array {
    let rust_array = get_legal_moves(fen);
    return rust_array.into_iter().map(JsValue::from).collect() // Works as expected! 
    //return js_sys::Array::from(&rust_array[..]);
}
