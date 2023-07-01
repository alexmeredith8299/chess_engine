#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use chess_engine::utils::chess_util;

    #[test]
    fn test_get_all_moves_knight() {
        //D4
        let rank = 3;
        let file = 3;
        let knight_moves = chess_util::get_all_moves_knight(rank, file);
        //assert_eq!(true, false);
    }

    
}
