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

pub fn get_fen_for_piece(piece: Piece) -> char {
    let piece_chars: [char; 2];
    match piece.piece_type {
        PieceType::Pawn => {
            piece_chars = ['p', 'P'];
        }
        PieceType::Knight => {
            piece_chars = ['n', 'N'];
        }
        PieceType::Bishop => {
            piece_chars = ['b', 'B'];
        }
        PieceType::Rook => {
            piece_chars = ['r', 'R'];
        }
        PieceType::Queen => {
            piece_chars = ['q', 'Q'];
        }
        PieceType::King => {
            piece_chars = ['k', 'K'];
        }
    }
    match piece.side {
        Side::White => {
            return piece_chars[1];
        }
        Side::Black => {
            return piece_chars[0];
        }
        _ => {
            panic!("Couldn't parse side");
        }
    }
}

pub fn fen_from_rank(rank: Vec<char>) -> String {
    let mut final_fen_chars: Vec<char> = Vec::new();
    let mut space_counter: u32 = 0;
    for fen_char in rank {
        if fen_char == 'X' {
            space_counter += 1;
        }
        else {
            if space_counter > 0 {
                final_fen_chars.push(char::from_digit(space_counter, 10).unwrap());
                space_counter = 0;
            }
            final_fen_chars.push(fen_char);
        }
    }
    if space_counter > 0 {
        final_fen_chars.push(char::from_digit(space_counter, 10).unwrap());
        space_counter = 0;
    }
    let fen_rank: String = final_fen_chars.iter().collect();
    return fen_rank;
}

pub fn rank_from_fen(fen_rank: &str) -> Vec<char> {
    let mut fen_rank_pieces: Vec<char> = Vec::new();
    for fen_char in fen_rank.chars() {
       if fen_char.is_digit(10) {
           let c_digit: u32 = fen_char.to_digit(10).unwrap();
           for i in 0..c_digit {
               fen_rank_pieces.push('X');
           }
       }
       else {
           fen_rank_pieces.push(fen_char);
       }
    }
    return fen_rank_pieces;
}

pub fn add_piece_to_fen(fen: &str, target: &str, piece: &str) -> String {
    //Get rank to update as vec of chars (X for empty)
    let piece_rs = parse_piece(target, piece);
    let mut fen_ranks: Vec<&str> = fen.split('/').collect();
    let fen_rank_to_change = fen_ranks[7-piece_rs.rank as usize];
    let mut fen_rank_pieces: Vec<char> = rank_from_fen(fen_rank_to_change);
    
    //Add piece in the correct spot
    fen_rank_pieces.remove(piece_rs.file as usize);
    let piece_char = get_fen_for_piece(piece_rs);
    fen_rank_pieces.insert(piece_rs.file as usize, piece_char);

    //Convert back to fen-formatted string and insert into fen
    let fen_rank = fen_from_rank(fen_rank_pieces);
    fen_ranks.remove(7-piece_rs.rank as usize);
    fen_ranks.insert(7-piece_rs.rank as usize, &fen_rank);
    let final_fen_ranks = fen_ranks.join("/");
    return final_fen_ranks;
}

pub fn remove_piece_from_fen(fen: &str, source: &str, piece: &str) -> String {
    //Get rank to update as vec of chars (X for empty)
    let piece_rs = parse_piece(source, piece);
    let mut fen_ranks: Vec<&str> = fen.split('/').collect();
    let fen_rank_to_change = fen_ranks[7-piece_rs.rank as usize];
    let mut fen_rank_pieces: Vec<char> = rank_from_fen(fen_rank_to_change);
    
    //Remove piece and add X for empty 
    fen_rank_pieces.remove(piece_rs.file as usize);
    fen_rank_pieces.insert(piece_rs.file as usize, 'X');

    //Convert back to fen-formatted string and insert into fen
    let fen_rank = fen_from_rank(fen_rank_pieces);
    fen_ranks.remove(7-piece_rs.rank as usize);
    fen_ranks.insert(7-piece_rs.rank as usize, &fen_rank);
    let final_fen_ranks = fen_ranks.join("/");
    return final_fen_ranks;
}

pub fn get_fen_for_move(old_fen: &str, source: &str, target: &str, piece: &str) -> String {

    let removed_fen = remove_piece_from_fen(old_fen, source, piece);
    let final_fen = add_piece_to_fen(&removed_fen, target, piece);
    //return "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R".to_string();
    return final_fen;
}

pub fn get_engine_move(fen: &str, engine_side: Side) -> String {
    //return "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R".to_string();
    return fen.to_string();
}


