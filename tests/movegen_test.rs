#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use chess_engine::utils::chess_util;
    use chess_engine::board::bitboard::Bitboard;

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
        assert_eq!(expected_knight_moves.sort(), knight_moves.sort());
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
