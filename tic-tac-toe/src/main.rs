use std::fmt;

const SIZE: usize = 3;

#[derive(Debug, PartialEq, Clone)]
struct AutoPlay {
    play_type: [bool; 2],           // True: automatic random moves, False: manual moves
    play_type_str: [String; 2],     // "automatic" or "manual"
}

impl Default for AutoPlay {
    fn default() -> AutoPlay {
        // By default, both players are automated to play random, legal moves
        AutoPlay{play_type: [true, true], 
                 play_type_str: ["automatic".to_string(), "automatic".to_string()]}
    }
}

struct Game {
    board: [[char; SIZE]; SIZE],    // tic tac toe board
    curr_player: usize,             // current player 
    players: [char; 2],             // players represented by pieces
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
            curr_player: 0,
            players: ['X', 'O'],
            auto_play: AutoPlay::default(), 
            end_game: false,
        }
    }

    fn start(&mut self, p1_auto: bool, p2_auto: bool) {
        // Set the automatic/manual play settings for each player in order to start the game
        self.auto_play.play_type = [p1_auto, p2_auto];

        match self.auto_play {
            AutoPlay { play_type: [false, false], .. } => {
                self.auto_play.play_type_str = ["manual".to_string(), "manual".to_string()];
            },
            AutoPlay { play_type: [true, false], .. } => {
                self.auto_play.play_type_str = ["automatic".to_string(), "manual".to_string()];
            },
            AutoPlay { play_type: [false, true], .. } => {
                self.auto_play.play_type_str = ["manual".to_string(), "automatic".to_string()];
            },
            AutoPlay { play_type: [_, _], .. } => {
                self.auto_play.play_type_str = ["automatic".to_string(), "automatic".to_string()];
            },
        }
    }

    fn update(&mut self) {
        // TODO: add update function: while game not ended, update_move
        println!("implement update()");
        if self.auto_play.play_type[self.curr_player] {
            self.auto_move();
        } else {
            self.manual_move();
        }
    }

    fn auto_move(&mut self) {
        // TODO: update_auto (randomized, legal moves)
        println!("make an automated move");
    }

    fn manual_move(&mut self) {
        // TODO: update_manual
        println!("make a manual move");
    }

    fn is_endgame(&mut self) {
        // TODO: check for win/draw state (draw state only needs to be checked if the board is full)
        println!("is the game won/drawn?");
        self.end_game = true;
    }

    fn reset(&mut self) {
        // TODO: reset game
        println!("reset game");
        Self {
            board: [[' ', ' ', ' '], 
                    [' ', ' ', ' '], 
                    [' ', ' ', ' ']],
            curr_player: 0,
            players: ['X', 'O'],
            auto_play: self.auto_play.to_owned(),
            end_game: false,
        };
    }
}

#[allow(unused_must_use)]
impl fmt::Display for Game {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //  Display for game state
        let mut game_status = format!("in play, {}'s turn", &self.players[self.curr_player]);
        if self.end_game {
            game_status = "ended".to_string();
        }

        let mut total_lines = &SIZE - 1;
        write!(formatter, "\nPlayer 1 ({})\nPlayer 2 ({})", &self.auto_play.play_type[0], &self.auto_play.play_type[1]);
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
    game.start(true, true);
    println!("{}", game);

    while game.end_game == false {
        game.update();
    }
    game.reset();
}


/***********
 UNIT TESTS
************/

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test reset()
    // TODO: test update()

    #[test]
    fn test_player_init_settings() {
        // test that players are set with correct autoplay booleans
        let mut game = Game::new();
        game.start(true, true);
        assert_eq!(AutoPlay { play_type: [true, true], play_type_str: ["automatic".to_string(), "automatic".to_string()] }, game.auto_play);
        game.start(false, true);
        assert_eq!(AutoPlay { play_type: [false, true], play_type_str: ["manual".to_string(), "automatic".to_string()] }, game.auto_play);
        game.start(true, false);
        assert_eq!(AutoPlay { play_type: [true, false], play_type_str: ["automatic".to_string(), "manual".to_string()] }, game.auto_play);
        game.start(false, false);
        assert_eq!(AutoPlay { play_type: [false, false], play_type_str: ["manual".to_string(), "manual".to_string()] }, game.auto_play);
    }

    #[test]
    fn test_board_init_display() {
        // test that init board displays correctly
        let mut game = Game::new();
        game.start(true, true);

        let mut game_status = format!("Game in play, {}'s turn", &game.players[game.curr_player]);
        if game.end_game {
            game_status = "Game ended".to_string();
        }
        let mut expect_board = format!("\nPlayer 1 ({})\nPlayer 2 ({})\n{}:", 
                                       &game.auto_play.play_type[0], 
                                       &game.auto_play.play_type[1], 
                                       &game_status);
        expect_board += "\n    |   |  \n -----------\n    |   |  \n -----------\n    |   |  \n";
        assert_eq!(expect_board, format!("{}", game));
    }

    #[test]
    fn test_board_init_vals() {
        // test that init board contains correct values
        let board: [[char; SIZE]; SIZE];
        board = [[' ', ' ', ' '], 
                [' ', ' ', ' '], 
                [' ', ' ', ' ']];
        let game = Game::new();
        assert_eq!(board, game.board);
    }
}
