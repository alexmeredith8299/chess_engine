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
    piece_type: PieceType,
    side: Side,
    rank: u8, //TODO: restrict to 1...8
    file: u8 
}

fn parse_piece(source: &str, piece: &str) -> Piece {
    let side_str = piece.chars().nth(0).unwrap();
    let piece_str = piece.chars().nth(1).unwrap();
    let side: Side;
    match side_str {
        'w' => side = Side::White,
        'b' => side = Side::Black,
        _ => panic!("Couldn't parse side"),
    }
    let piece_type: PieceType;
    match piece_str {
        'P' => piece_type = PieceType::Pawn,
        'B' => piece_type = PieceType::Bishop,
        'R' => piece_type = PieceType::Rook,
        'Q' => piece_type = PieceType::Queen,
        'K' => piece_type = PieceType::King,
        _ => panic!("Couldn't parse piece type"),
    }
    let (file, rank) = parse_square(source);
    return Piece{piece_type: piece_type, side: side, rank: rank, file: file}
}

fn parse_square(source: &str) -> (u8, u8) {
    let file = source.chars().nth(0).unwrap();
    let rank = source.chars().nth(1).unwrap();
    let files: Vec<char> = "abcdefgh".chars().collect();
    let file_index = files.iter().position(|&r| r == file).unwrap();
    let ranks: Vec<char> = "12345678".chars().collect();
    let rank_index = ranks.iter().position(|&r| r == rank).unwrap();

    return (file_index.try_into().unwrap(), rank_index.try_into().unwrap());
}

pub fn get_legal_moves(fen: &str, source: &str, piece: &str, side: Side) -> Vec<String> {
    if piece.is_empty() {
        return Vec::new();
    }
    let moves = Vec::from(["e4".to_string(), "e5".to_string()]);
    let piece_rs = parse_piece(source, piece);
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


