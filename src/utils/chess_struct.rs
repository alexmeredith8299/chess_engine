use wasm_bindgen::prelude::*;
use std::convert::TryInto;

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

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5
}

#[wasm_bindgen]
pub struct Piece {
    pub piece_type: PieceType,
    pub side: Side,
    pub rank: u8, //TODO: restrict to 1...8
    pub file: u8 
}


