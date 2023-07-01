#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use chess_engine::board::bitboard::Bitboard;
    use chess_engine::board::bitboard::parse_from_side;
    use chess_engine::board::bitboard::parse_from_piece_type;
    use chess_engine::board::bitboard::parse_all_pieces;
    use chess_engine::board::bitboard::parse_from_square;
    use chess_engine::board::bitboard::shift_left;
    use chess_engine::board::bitboard::shift_right;
    use chess_engine::board::bitboard::shift_up;
    use chess_engine::board::bitboard::shift_down;
    use chess_engine::board::bitboard::shift;
    use chess_engine::board::constants;
    use chess_engine::utils::chess_struct::Side;
    use chess_engine::utils::chess_struct::PieceType;

    #[test]
    fn init_board() {
        let bitboard: u64 = 0;
        let board = Bitboard{bitboard: bitboard};
        assert_eq!(true, true);
    }

    #[test]
    fn test_parse_from_side() {
        let bitboard: u64 = 13832559999223595008;
        let side = Side::Black;
        let fen = "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R";
        let board = parse_from_side(fen, side);
        assert_eq!(board.bitboard, bitboard);
    }

    #[test]
    fn test_parse_all_pieces() {
        let side = Side::Black;
        let fen = "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R";
        let black_pieces = parse_from_side(fen, side);
        let white_pieces = parse_from_side(fen, Side::White);
        let all_pieces = parse_all_pieces(fen);
        assert_eq!(all_pieces.bitboard, black_pieces.bitboard | white_pieces.bitboard);
        assert_eq!(all_pieces.bitboard ^ black_pieces.bitboard, white_pieces.bitboard);
    }

    #[test]
    fn test_parse_piece_type() {
        let fen = "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R";
        let pawns = parse_from_piece_type(fen, PieceType::Pawn);
        let knights = parse_from_piece_type(fen, PieceType::Knight);
        let bishops = parse_from_piece_type(fen, PieceType::Bishop);
        let rooks = parse_from_piece_type(fen, PieceType::Rook);
        let queens = parse_from_piece_type(fen, PieceType::Queen);
        let kings = parse_from_piece_type(fen, PieceType::King);
        let all_pieces = parse_all_pieces(fen);
        assert_eq!(all_pieces.bitboard, pawns.bitboard | knights.bitboard | bishops.bitboard | rooks.bitboard | queens.bitboard | kings.bitboard);
    }

    #[test]
    fn test_square_constants() {
        let fen = "r7/8/8/8/4n3/8/8/7p";
        let rooks = parse_from_piece_type(fen, PieceType::Rook);
        let pawns = parse_from_piece_type(fen, PieceType::Pawn);
        let knights = parse_from_piece_type(fen, PieceType::Knight);
        assert_eq!(rooks.bitboard, constants::A8);
        assert_eq!(pawns.bitboard, constants::H1);
        assert_eq!(knights.bitboard, constants::E4);
    }

    #[test]
    fn test_rank_file_constants() {
        let fen = "r7/r7/r7/r7/r7/r7/r7/RPPPPPPP";
        let rooks = parse_from_piece_type(fen, PieceType::Rook);
        let white_pieces = parse_from_side(fen, Side::White);

        assert_eq!(rooks.bitboard, constants::A_file);
        assert_eq!(white_pieces.bitboard, constants::first_rank);
    }

    #[test]
    fn test_shift_left() {
        let fen = "r7/r7/r7/r7/r7/r7/r7/RPPPPPPP";
        let rooks = parse_from_piece_type(fen, PieceType::Rook);
        let white_pieces = parse_from_side(fen, Side::White);

        assert_eq!(shift_left(rooks).bitboard, 0);
        assert_eq!(shift_left(white_pieces).bitboard, 254);
    }

    #[test]
    fn test_shift_right() {
        let fen = "7r/7r/7r/7r/7r/7r/7r/RPPPPPPP";
        let rooks = parse_from_piece_type(fen, PieceType::Rook);
        let white_pieces = parse_from_side(fen, Side::White);

        assert_eq!(shift_right(rooks).bitboard, 64);
        assert_eq!(shift_right(white_pieces).bitboard, 127);
    }
    
    #[test]
    fn test_shift_up() {
        let fen = "r7/8/8/8/8/8/8/RPPPPPPP";
        let rooks = parse_from_piece_type(fen, PieceType::Rook);
        let white_pieces = parse_from_side(fen, Side::White);

        assert_eq!(shift_up(rooks).bitboard, 32768);
        assert_eq!(shift_up(white_pieces).bitboard, 65280);
    }

    #[test]
    fn test_shift_down() {
        let fen = "r7/8/8/8/8/8/8/RPPPPPPP";
        let rooks = parse_from_piece_type(fen, PieceType::Rook);
        let white_pieces = parse_from_side(fen, Side::White);

        assert_eq!(shift_down(rooks).bitboard, constants::A7);
        assert_eq!(shift_down(white_pieces).bitboard, 0);
    }

    #[test]
    fn test_shift() {
        let fen = "8/8/8/8/3r4/8/8/8";
        let rooks = parse_from_piece_type(fen, PieceType::Rook);
        assert_eq!(shift(rooks, 3, 2).bitboard, constants::G6);
        assert_eq!(shift(rooks, -2, -1).bitboard, constants::B3);
    }

    #[test]
    fn test_parse_from_square() {
        let rank = 0;
        let file = 0;
        assert_eq!(parse_from_square(rank, file).bitboard, constants::A1); 
        assert_eq!(parse_from_square(5, 2).bitboard, constants::C6);
    }
}
