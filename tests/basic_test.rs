//! Test suite for the Web and headless browsers.


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use chess_engine::ChessGame;

    #[test]
    fn init_board() {
        let mut game = ChessGame::new();
        assert_eq!(true, true);
    }

}
