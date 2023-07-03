pub mod utils;
pub mod board;

use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Array;
use crate::utils::chess_struct::Side;
use crate::utils::chess_struct::Outcome;
use crate::board::constants;//TODO remove, debug only

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
    black_in_check: bool,
    white_can_castle_kingside: bool,
    black_can_castle_kingside: bool,
    white_can_castle_queenside: bool,
    black_can_castle_queenside: bool,
    en_passant_square: Option<u64>
}


#[wasm_bindgen]
impl ChessGame {
    pub fn make_move(&mut self, old_fen: &str) -> String {
        let fen = utils::chess_util::get_engine_move(old_fen, self.engine_side);
        self.current_position = fen.clone();
        let (white_king_castle, black_king_castle, white_queen_castle, black_queen_castle) = utils::chess_util::check_castling(old_fen, &fen);
        self.white_can_castle_kingside = white_king_castle;
        self.black_can_castle_kingside = black_king_castle;
        self.white_can_castle_queenside = white_queen_castle;
        self.black_can_castle_queenside = black_queen_castle;
        //TODO uncomment once we are actually moving stuff
        //self.en_passant_square = utils::chess_util::get_en_passant_square(old_fen, &fen);
        return fen.clone();
    }

    pub fn check_if_legal(&self, old_fen: &str, source: &str, target: &str, piece: &str) -> bool {
        //Snap piece back if it's not on the board
        if target.eq("offboard") {
            return false;
        }
        //Otherwise check if it's a legal move
        return utils::chess_util::check_if_legal(old_fen, source, target, piece, self.player_side, self.white_can_castle_kingside, self.black_can_castle_kingside, self.white_can_castle_queenside, self.black_can_castle_queenside, self.en_passant_square); 
    }

    pub fn check_if_promotion(&self, old_fen: &str, source: &str, target: &str, piece: &str) -> bool {
        return utils::chess_util::check_if_promotion(old_fen, source, target, piece, self.player_side);
    }


    pub fn update(&mut self, old_fen: &str, source: &str, target: &str, piece: &str) -> String {
        let fen = utils::chess_util::get_fen_for_move(old_fen, source, target, piece, self.en_passant_square);
        self.current_position = fen.clone();
        let (white_king_castle, black_king_castle, white_queen_castle, black_queen_castle) = utils::chess_util::check_castling(old_fen, &fen);
        self.white_can_castle_kingside = white_king_castle;
        self.black_can_castle_kingside = black_king_castle;
        self.white_can_castle_queenside = white_queen_castle;
        self.black_can_castle_queenside = black_queen_castle;
        self.en_passant_square = utils::chess_util::get_en_passant_square(old_fen, &fen);
        return fen.clone();
    }

    pub fn get_legal_moves_for_highlighting(&self, fen: &str, source: &str, piece: &str) -> js_sys::Array {
        let rust_array = utils::chess_util::get_legal_moves(fen, source, piece, self.player_side, self.white_can_castle_kingside, self.black_can_castle_kingside, self.white_can_castle_queenside, self.black_can_castle_queenside, self.en_passant_square);
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
            current_position: current_position,
            white_can_castle_kingside: true,
            black_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_queenside: true,
            en_passant_square: None
        }
    }
}
