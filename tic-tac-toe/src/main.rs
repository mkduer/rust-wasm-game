use std::fmt;

const SIZE: usize = 3;

#[derive(Debug, PartialEq)]
struct AutoPlay {
    player1: bool,  // True: automatic random moves, False: manual moves
    player2: bool,
    p1_type: String,  // "automatic" or "manual"
    p2_type: String,
}

impl Default for AutoPlay {
    fn default() -> AutoPlay {
        // By default, both players are automated to play random, legal moves
        AutoPlay{player1: true, player2: true, p1_type: "automatic".to_string(), p2_type: "automatic".to_string()}
    }
}

struct Game {
    board: [[char; SIZE]; SIZE],    // tic tac toe board
    player: char,                   // current player
    auto_play: AutoPlay,            // type of play for each player
    end_game: bool,                 // game status: False if in play, True if ended by win/draw
}

impl Game {
    fn new() -> Self {
        // Initializes the game board, first player piece and default autoplay for both players
        Self {
            board: [[' ', ' ', ' '], 
                    [' ', ' ', ' '], 
                    [' ', ' ', ' ']],
            player: 'X',
            auto_play: AutoPlay::default(), 
            end_game: false,
        }
    }

    fn start(&mut self, p1_auto: bool, p2_auto: bool) {
        // Set the automatic/manual play settings for each player in order to start the game
        self.auto_play.player1 = p1_auto;
        self.auto_play.player2 = p2_auto;

        match self.auto_play {
            AutoPlay {player1: false, player2: false, ..} => {
                self.auto_play.p1_type = "manual".to_string();
                self.auto_play.p2_type = "manual".to_string();
            },
            AutoPlay {player1: true, player2: false, ..} => {
                self.auto_play.p1_type = "automatic".to_string();
                self.auto_play.p2_type = "manual".to_string();
            },
            AutoPlay {player1: false, player2: true, ..} => {
                self.auto_play.p1_type = "manual".to_string();
                self.auto_play.p2_type = "automatic".to_string();
            },
            AutoPlay {player1: _, player2: _, ..} => {
                self.auto_play.p1_type = "automatic".to_string();
                self.auto_play.p2_type = "automatic".to_string();
            },
        }
    }
}

#[allow(unused_must_use)]
impl fmt::Display for Game {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //  Display for game state
        let mut game_status = format!("in play, {}'s turn", &self.player);
        if self.end_game {
            game_status = "ended".to_string();
        }

        let mut total_lines = &SIZE - 1;
        write!(formatter, "\nPlayer 1 ({})\nPlayer 2 ({})", &self.auto_play.p1_type, &self.auto_play.p2_type);
        write!(formatter, "\nGame {}:\n", &game_status);
        for row in &self.board {
            write!(formatter, "  {} | {} | {}\n", row[0], row[1], row[2]);
            if total_lines > 0 {
                write!(formatter, " -----------\n");
                total_lines -= 1;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    game.start(false, false);
    println!("{}", game);
}


/***********
 UNIT TESTS
************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_settings() {
        // test that players are set with correct autoplay booleans
        let mut game = Game::new();
        game.start(true, true);
        assert_eq!(AutoPlay {player1: true, player2: true, 
                             p1_type: "automatic".to_string(), p2_type: "automatic".to_string()}, 
                             game.auto_play);
        game.start(false, true);
        assert_eq!(AutoPlay {player1: false, player2: true, 
                             p1_type: "manual".to_string(), p2_type: "automatic".to_string()}, 
                             game.auto_play);
        game.start(true, false);
        assert_eq!(AutoPlay {player1: true, player2: false, 
                             p1_type: "automatic".to_string(), p2_type: "manual".to_string()}, 
                             game.auto_play);
        game.start(false, false);
        assert_eq!(AutoPlay {player1: false, player2: false, 
                             p1_type: "manual".to_string(), p2_type: "manual".to_string()}, 
                             game.auto_play);
    }

    #[test]
    fn test_board_display() {
        // test that init board displays correctly
        // note: decreased indent for raw-string
        let mut game = Game::new();
        game.start(true, true);

        let mut game_status = format!("Game in play, {}'s turn", game.player);
        if game.end_game {
            game_status = "Game ended".to_string();
        }
        let mut expect_board = format!("\nPlayer 1 ({})\nPlayer 2 ({})\n{}:", 
                                       game.auto_play.p1_type, 
                                       game.auto_play.p2_type, 
                                       game_status);
        expect_board += "\n    |   |  \n -----------\n    |   |  \n -----------\n    |   |  \n";
        assert_eq!(expect_board, format!("{}", game));
    }

    #[test]
    fn test_board_vals() {
        // test that init board contains correct values
        let board: [[char; SIZE]; SIZE];
        board = [[' ', ' ', ' '], 
                [' ', ' ', ' '], 
                [' ', ' ', ' ']];
        let game = Game::new();
        assert_eq!(board, game.board);
    }
}
