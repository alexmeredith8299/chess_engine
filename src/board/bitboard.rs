use super::super::utils::chess_struct::Side;
use super::super::utils::chess_struct::PieceType;
use super::super::board::constants;

pub struct Bitboard {
    pub bitboard: u64
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

pub fn to_squares(bitboard: Bitboard) -> Vec<String> {
    let occupied = (0..64).rev().map(|n| (bitboard.bitboard >> n) & 1); //Most to least significant
                                                                      let square_names = ["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
                                                                                          "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
                                                                                          "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
                                                                                          "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
                                                                                          "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
                                                                                          "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
                                                                                          "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
                                                                                          "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"];
                                                                      let mut squares: Vec<String> = Vec::new();
                                                                      let mut i = 0;
    for occ in occupied {
        if occ == 1 {
            squares.push(square_names[i].to_string());
        }
        i +=1;
    }
    return squares;
}
