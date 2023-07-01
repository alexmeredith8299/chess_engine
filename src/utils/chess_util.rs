use wasm_bindgen::prelude::*;
use std::convert::TryInto;
use super::super::board::bitboard::Bitboard;
use super::super::board::bitboard;
use super::super::board::constants;
use super::super::utils::chess_struct::Outcome;
use super::super::utils::chess_struct::Side;
use super::super::utils::chess_struct::PieceType;
use super::super::utils::chess_struct::Piece;

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
        'N' => piece_type = PieceType::Knight,
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

pub fn get_all_moves_knight(rank: u8, file: u8) -> Vec<String> {
    let knight_d4: u64 = u64::from_str_radix(constants::knight_d4_bitstr, 2).unwrap();
    return bitboard::to_squares(Bitboard{bitboard:knight_d4});
    //return Vec::from(["e1".to_string(), "e5".to_string()])
}

fn get_all_moves(piece: Piece, enemies: Bitboard, allies: Bitboard) -> Vec<String> {
    match piece.piece_type {
        PieceType::Knight => return get_all_moves_knight(piece.rank, piece.file),
        _ => return Vec::from(["e4".to_string(), "e5".to_string()]),
    }
}

pub fn get_legal_moves(fen: &str, source: &str, piece: &str, side: Side) -> Vec<String> {
    if piece.is_empty() {
        return Vec::new();
    }
    //let moves = Vec::from(["e3".to_string(), "e5".to_string()]);
    let piece_rs = parse_piece(source, piece);
    let allies = bitboard::parse_from_side(fen, side);
    let pieces = bitboard::parse_all_pieces(fen);
    let enemies = Bitboard{bitboard: pieces.bitboard ^ allies.bitboard};
    let moves = get_all_moves(piece_rs, enemies, allies);
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


