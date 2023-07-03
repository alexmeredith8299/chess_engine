#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use chess_engine::utils::chess_util;
    use chess_engine::board::bitboard::Bitboard;
    use chess_engine::board::constants;
    use chess_engine::utils::chess_struct::Side;

    #[test]
    fn test_get_all_moves_knight() {
        //G1
        let rank = 0;
        let file = 6;
        let enemies = Bitboard{bitboard:0};
        //E2 is occupied with an ally (as in starting position)
        let allies = Bitboard{bitboard:2048};
        let mut knight_moves = chess_util::get_all_moves_knight(rank, file, enemies, allies);
        let mut expected_knight_moves = Vec::from(["h3".to_string(), "f3".to_string()]);
        knight_moves.sort();
        expected_knight_moves.sort();
        assert_eq!(expected_knight_moves, knight_moves);
    }

   #[test]
   fn test_en_passant_square() {
        let old_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let fen = "rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR";
        let en_passant_square = chess_util::get_en_passant_square(old_fen, fen).unwrap();
        let expected_en_passant_square = constants::F3;
        assert_eq!(expected_en_passant_square, en_passant_square);
   }

   #[test]
   fn test_en_passant() {
       //G5
       let rank = 4;
       let file = 6;
       let enemies = Bitboard{bitboard:constants::F5};
       let allies = Bitboard{bitboard:0};
       let en_passant_square = Some(constants::F6);
       let mut pawn_moves = chess_util::get_all_moves_pawn(rank, file, Side::White, enemies, allies, en_passant_square);
       let mut expected_pawn_moves = Vec::from(["g6".to_string(), "f6".to_string()]);
       pawn_moves.sort();
       expected_pawn_moves.sort();
       assert_eq!(expected_pawn_moves, pawn_moves);
   }

   #[test]
   fn test_add_piece_to_fen() {
        let fen = "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R";
        let target = "a5";
        let piece = "wP";
        let new_fen = chess_util::add_piece_to_fen(fen, target, piece);
        let expected_fen = "r1bqkbnr/pppp1ppp/2n5/PB2p3/4P3/5N2/PPPP1PPP/RNBQK2R";
        assert_eq!(expected_fen, new_fen);
   }

   #[test]
   fn test_remove_piece_from_fen() {
        let fen = "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R";
        let target = "h1";
        let piece = "wP";
        let new_fen = chess_util::remove_piece_from_fen(fen, target, piece);
        let expected_fen = "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK3";
        assert_eq!(expected_fen, new_fen);
   }

}
