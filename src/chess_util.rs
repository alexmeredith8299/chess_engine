use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    WhiteWon = 0,
    Draw = 1,
    BlackWon = 2
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    White = 0,
    Black = 1
}

pub fn get_legal_moves(fen: &str, source: &str, piece: &str, side: Side) -> Vec<String> {
    if piece.is_empty() {
        return Vec::new();
    }
    let moves = Vec::from(["e4".to_string(), "e5".to_string()]);
    return moves;
}

pub fn check_if_legal(fen: &str, source: &str, target: &str, piece: &str, side: Side) -> bool {
    let legal_moves = get_legal_moves(fen, source, piece, side);
    for legal_target in legal_moves {
        if legal_target == target {
            return true
        }
    }
    return true
}

pub fn get_fen_for_move(old_fen: &str, source: &str, target: &str, piece: &str) -> String {
    return "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R".to_string();
}

pub fn get_engine_move(fen: &str, engine_side: Side) -> String {
    return "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R".to_string();
}


