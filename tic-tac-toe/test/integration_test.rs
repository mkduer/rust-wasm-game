extern crate Game;

mod integration_tests {

    use Game;
    use more_asserts::{assert_ge};

    #[test]
    fn test_auto_move_reaches_endgame() {
        // Should result in an endgame within `Game::const SIZE` moves, 
        // otherwise, there is a halting error
        let mut max_moves = Game::SIZE as isize;
        let mut game = Game::new();

        game.start(true, true);

        while (game.end_game == false) && (max_moves > -2) {
            game.update();
            max_moves -= 1;
        }
        assert_ge!(max_moves, 0)
    }
}