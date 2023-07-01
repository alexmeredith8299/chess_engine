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

    
}
