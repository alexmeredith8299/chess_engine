use super::super::utils::chess_struct::Side;
use super::super::utils::chess_struct::PieceType;
use super::super::board::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard {
    pub bitboard: u64
}

pub fn parse_from_square(rank: u8, file: u8) -> Bitboard {
    let bitboard: u64 = 1 << (rank * 8 + (7-file));
    return Bitboard{bitboard: bitboard};
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
    let bitboard: u64 = u64::from_str_radix(&bitstring, 2).unwrap();
    return Bitboard{bitboard: bitboard};
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
    let bitboard: u64 = u64::from_str_radix(&bitstring, 2).unwrap();
    return Bitboard{bitboard: bitboard};
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
    let bitboard: u64 = u64::from_str_radix(&bitstring, 2).unwrap();
    return Bitboard{bitboard: bitboard};
}

pub fn shift_left(bitboard: Bitboard) -> Bitboard {
    let new_bitboard = (bitboard.bitboard & !constants::A_file) << 1;
    return Bitboard{bitboard: new_bitboard};
}

pub fn shift_right(bitboard: Bitboard) -> Bitboard {
    let new_bitboard = (bitboard.bitboard & !constants::H_file) >> 1;
    return Bitboard{bitboard: new_bitboard};
}

pub fn shift_up(bitboard: Bitboard) -> Bitboard {
    let new_bitboard = (bitboard.bitboard & !constants::eighth_rank) << 8;
    return Bitboard{bitboard: new_bitboard};
}

pub fn shift_down(bitboard: Bitboard) -> Bitboard {
    let new_bitboard = (bitboard.bitboard & !constants::first_rank) >> 8;
    return Bitboard{bitboard: new_bitboard};
}

pub fn shift(bitboard: Bitboard, x: i8, y: i8) -> Bitboard {
    let mut new_bitboard: u64 = bitboard.bitboard;
    for i in 0..x {
        new_bitboard = shift_right(Bitboard{bitboard: new_bitboard}).bitboard;
    }
    for i in 0..-x {
        new_bitboard = shift_left(Bitboard{bitboard: new_bitboard}).bitboard;
    }
    for i in 0..y {
        new_bitboard = shift_up(Bitboard{bitboard: new_bitboard}).bitboard;
    }
    for i in 0..-y {
        new_bitboard = shift_down(Bitboard{bitboard: new_bitboard}).bitboard;
    }
    return Bitboard{bitboard: new_bitboard};
}

pub fn to_squares(bitboard: Bitboard) -> Vec<String> {
    let occupied = (0..64).rev().map(|n| (bitboard.bitboard >> n) & 1); //Most to least significant
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
