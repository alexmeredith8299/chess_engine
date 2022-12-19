mod utils;
mod chess_util;

use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Array;
use crate::chess_util::Side;
use crate::chess_util::Outcome;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ChessGame {
    current_turn: Side,
    player_side: Side,
    engine_side: Side,
    game_over: bool,
    outcome: Option<Outcome>,
    current_position: String,
    white_in_check: bool,
    black_in_check: bool
}


#[wasm_bindgen]
impl ChessGame {
    pub fn make_move(&mut self, fen: &str) -> String {
        let fen = chess_util::get_engine_move(fen, self.engine_side);
        self.current_position = fen.clone();
        return fen.clone();
    }

    pub fn check_if_legal(&self, old_fen: &str, source: &str, target: &str, piece: &str) -> bool {
        return chess_util::check_if_legal(old_fen, source, target, piece, self.player_side); 
    }

    pub fn update(&mut self, old_fen: &str, source: &str, target: &str, piece: &str) -> String {
        let fen = chess_util::get_fen_for_move(old_fen, source, target, piece);
        self.current_position = fen.clone();
        return fen.clone();
    }

    pub fn get_legal_moves_for_highlighting(&self, fen: &str, source: &str, piece: &str) -> js_sys::Array {
        let rust_array = chess_util::get_legal_moves(fen, source, piece, self.player_side);
        return rust_array.into_iter().map(JsValue::from).collect() // Works as expected! 
    }

    pub fn new() -> ChessGame {
        let first_turn = Side::White;
        let player_side = Side::White;
        let engine_side = Side::Black;
        let game_over = false;
        let outcome = None;
        let white_in_check = false;
        let black_in_check = false;
        let current_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();

        ChessGame {
            current_turn: first_turn,
            player_side: player_side,
            engine_side: engine_side,
            game_over: game_over,
            outcome: outcome,
            white_in_check: white_in_check,
            black_in_check: black_in_check,
            current_position: current_position
        }
    }
}
