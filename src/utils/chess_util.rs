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

pub fn get_all_moves_knight(rank: u8, file: u8, enemies: Bitboard, allies: Bitboard) -> Vec<String> {
    let knight_d4: u64 = u64::from_str_radix(constants::knight_d4_bitstr, 2).unwrap();
    let x_shift: i8 = (file as i8-3).try_into().unwrap();
    let y_shift: i8 = (rank as i8-3).try_into().unwrap();
    let knight_moves = bitboard::shift(Bitboard{bitboard:knight_d4}, x_shift, y_shift);
    let knight_moves_no_allies = Bitboard{bitboard: knight_moves.bitboard & !allies.bitboard};
    return bitboard::to_squares(knight_moves_no_allies);
}

pub fn get_all_moves_pawn(rank: u8, file: u8, side: Side, enemies: Bitboard, allies: Bitboard) -> Vec<String> {
    let pawn_pos: Bitboard = bitboard::parse_from_square(rank, file);
    let forward: i8;
    let advance_two_rank: u64;
    let occupied: u64 = allies.bitboard | enemies.bitboard;
    match side {
        Side::White => {
            forward = 1;
            advance_two_rank = constants::fourth_rank;
        }
        Side::Black => {
            forward = -1;
            advance_two_rank = constants::fifth_rank;
        }
    }
    //TODO handle en passant
    let advance_one: u64 = bitboard::shift(pawn_pos, 0, forward).bitboard & !occupied;
    let advance_two: u64 = if advance_one!=0 {bitboard::shift(pawn_pos, 0, forward*2).bitboard & advance_two_rank & !occupied} else {0};
    let capture_right: u64 = bitboard::shift(pawn_pos, 1, forward).bitboard & enemies.bitboard;
    let capture_left: u64 = bitboard::shift(pawn_pos, -1, forward).bitboard & enemies.bitboard;
    return bitboard::to_squares(Bitboard{bitboard: advance_one | advance_two | capture_right | capture_left});
}


fn get_all_moves(piece: Piece, enemies: Bitboard, allies: Bitboard) -> Vec<String> {
    match piece.piece_type {
        PieceType::Knight => return get_all_moves_knight(piece.rank, piece.file, enemies, allies),
        PieceType::Pawn => return get_all_moves_pawn(piece.rank, piece.file, piece.side, enemies, allies),
        _ => return Vec::from(["e4".to_string(), "e5".to_string()]),
    }
}

pub fn get_legal_moves(fen: &str, source: &str, piece: &str, side: Side) -> Vec<String> {
    if piece.is_empty() {
        return Vec::new();
    }
    //let moves = Vec::from(["e3".to_string(), "e5".to_string()]);
    let piece_rs = parse_piece(source, piece);
    let allies = bitboard::parse_from_side(fen, piece_rs.side);
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
    //return "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R".to_string();
    return fen.to_string();
}


