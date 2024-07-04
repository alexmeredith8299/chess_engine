use std::convert::TryInto;
use super::super::utils::chess_struct::Side;
use super::super::utils::chess_struct::PieceType;
use super::super::board::constants;

pub type Bitboard = u64;

pub fn parse_from_square(rank: u8, file: u8) -> Bitboard {
    return 1 << (rank * 8 + (7-file));
}

pub fn get_rank(rank: u8) -> Bitboard {
    return constants::ranks[rank as usize];
}

pub fn get_file(file: u8) -> Bitboard {
    return constants::files[file as usize];
}


pub fn parse_all_pieces(fen: &str) -> Bitboard {
    //Returns Bitboard where all squares occupied by any piece are 1's
    let chars = fen.chars();
    let mut bits: Vec<char> = Vec::new();
    for c in chars {
        if c !='/' {
            if c.is_digit(10) { //Empty space
                let c_digit: u32 = c.to_digit(10).unwrap();
                for i in 0..c_digit {
                    bits.push('0');
                }
            }
            else {
                bits.push('1');
            }
        }
    }
    let bitstring: String = bits.iter().collect::<String>();
    return u64::from_str_radix(&bitstring, 2).unwrap();
}

pub fn parse_from_side(fen: &str, side: Side) -> Bitboard {
    //Returns Bitboard where all squares occupied by side are 1's
    let chars = fen.chars();
    let mut bits: Vec<char> = Vec::new();
    for c in chars {
        if c !='/' {
            if c.is_digit(10) { //Empty space
                let c_digit: u32 = c.to_digit(10).unwrap();
                for i in 0..c_digit {
                    bits.push('0');
                }
            }
            else {
                match side {
                    Side::White => {
                        if c.is_uppercase() {
                            bits.push('1');
                        } else {
                            bits.push('0');
                        }
                    }
                    Side::Black => {
                        if c.is_uppercase() {
                            bits.push('0');
                        } else {
                            bits.push('1');
                        }
                    }
                }
            }
        }
    }
    let bitstring: String = bits.iter().collect::<String>();
    return u64::from_str_radix(&bitstring, 2).unwrap();
}

pub fn parse_from_piece_type(fen: &str, piece_type: PieceType) -> Bitboard {
    //Returns Bitboard where all squares occupied by piece type are 1's
    let chars = fen.chars();
    let piece_chars: [char; 2];
    match piece_type {
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
    let mut bits: Vec<char> = Vec::new();
    for c in chars {
        if c !='/' {
            if c.is_digit(10) { //Empty space
                let c_digit: u32 = c.to_digit(10).unwrap();
                for i in 0..c_digit {
                    bits.push('0');
                }
            }
            else {
                if c == piece_chars[0] || c == piece_chars[1] {
                    bits.push('1');
                } else {
                    bits.push('0');
                }
            }
        }
    }
    let bitstring: String = bits.iter().collect::<String>();
    return u64::from_str_radix(&bitstring, 2).unwrap();
}

pub fn shift_left(bitboard: Bitboard) -> Bitboard {
    return (bitboard & !constants::A_file) << 1;
}

pub fn shift_right(bitboard: Bitboard) -> Bitboard {
    return (bitboard & !constants::H_file) >> 1;
}

pub fn shift_up(bitboard: Bitboard) -> Bitboard {
    return (bitboard & !constants::eighth_rank) << 8;
}

pub fn shift_down(bitboard: Bitboard) -> Bitboard {
    return (bitboard & !constants::first_rank) >> 8;
}

pub fn shift(bitboard: Bitboard, x: i8, y: i8) -> Bitboard {
    let mut new_bitboard: u64 = bitboard;//bitboard.bitboard;
    for i in 0..x {
        new_bitboard = shift_right(new_bitboard);
    }
    for i in 0..-x {
        new_bitboard = shift_left(new_bitboard);
    }
    for i in 0..y {
        new_bitboard = shift_up(new_bitboard);
    }
    for i in 0..-y {
        new_bitboard = shift_down(new_bitboard);
    }
    return new_bitboard;
}

pub fn to_squares(bitboard: Bitboard) -> Vec<String> {
    let occupied = (0..64).rev().map(|n| (bitboard >> n) & 1); //Most to least significant
    let mut squares: Vec<String> = Vec::new();
    let mut i = 0;
    for occ in occupied {
        if occ == 1 {
            squares.push(constants::square_names[i].to_string());
        }
        i +=1;
    }
    return squares;
}

pub fn to_ranks_and_files(bitboard: Bitboard) -> (Vec<u64>, Vec<u64>) {
    let occupied = (0..64).rev().map(|n| (bitboard >> n) & 1); //Most to least significant
    let mut ranks: Vec<u64> = Vec::new();
    let mut files: Vec<u64> = Vec::new();
    let mut i = 0;
    for occ in occupied {
        if occ == 1 {
            let square = constants::squares[i];
            ranks.push(square/8);//.try_into().unwrap());
            files.push(square%8);//.try_into().unwrap());
        }
        i +=1;
    }
    return (ranks, files);
}

pub fn get_squares_above(rank: u8) -> Bitboard {
    let squares: u64 = constants::all_squares;
    return shift(squares, 0, (rank+1).try_into().unwrap());
}
pub fn get_squares_below(rank: u8) -> Bitboard {
    let squares: u64 = constants::all_squares;
    let rank_i8: i8 = (rank+1).try_into().unwrap();
    return shift(squares, 0, -rank_i8);
}
