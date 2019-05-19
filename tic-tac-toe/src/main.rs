use std::fmt;
use rand::{thread_rng, Rng};

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

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
    legal: bool,
}

struct Game {
    board: [[char; SIZE]; SIZE],    // tic tac toe board
    curr_player: usize,             // current player 
    players: [char; 2],             // players represented by pieces
    auto_play: AutoPlay,            // type of play for each player
    end_game: bool,                 // game status: False if in play, True if ended by win/draw
    coordinates: Vec<Coord>,        // coordinates for moves
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
            coordinates: coord_mapping(),
        }
    }

    fn start(&mut self, p1_auto: bool, p2_auto: bool) {
        // Set the automatic/manual play settings for each player in order to start the game
        // Create the coordinates per game size
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
            AutoPlay { .. } => {
                self.auto_play.play_type_str = ["automatic".to_string(), "automatic".to_string()];
            },
        }
    }

    fn update(&mut self) { 
        // Have the current player choose a location for their move 
        println!("implement update()"); 
        let loc: usize;
        if self.auto_play.play_type[self.curr_player] {
            loc = self.auto_move();
        } else {
            loc = self.manual_move();
        }

        // Update the board and coordinates
        let x = self.coordinates[loc].x;
        let y = self.coordinates[loc].y;
        self.board[x][y] = self.players[self.curr_player];
        self.coordinates[loc].legal = false;

        println!("loc: {}, coordinates: {:?}", loc, self.coordinates[loc]);

        self.end_game = self.is_endgame();
        self.curr_player = self.switch_player();
    }

    fn switch_player(&mut self) -> usize {
        // Switch current player
        if self.curr_player == 0 { 1 } else { 0 }
    }

    fn auto_move(&mut self) -> usize {
        // Return the location (index) for a random, legal move
        let max_rng = SIZE * SIZE;
        let mut rng = thread_rng();
        let mut loc = rng.gen_range(0, max_rng);
        let mut valid: bool = self.coordinates[loc].legal;

        while valid == false {
            loc = rng.gen_range(0, max_rng);
            println!("loc: {}", loc);
            valid = self.coordinates[loc].legal;
        }
        loc
    }

    fn manual_move(&mut self) -> usize {
        println!("make a manual move");
        let max_rng: usize = SIZE * SIZE;
        // TODO: ask player for legal move
        // TODO: check if it's legal    
        // TODO: repeat until valid move
        // TODO: return valid Coord
        /*
        let mut loc: usize;
        loc
        */
        0
    }

    fn is_endgame(&mut self) -> bool {
        // TODO: check for win/draw state (draw state only needs to be checked if the board is full)
        // TODO: print out the results and board if it is the end of the game, return true if end game
        true
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
            coordinates: self.coordinates.to_owned(),
        };
    }
}

fn coord_mapping() -> Vec<Coord> {
    let mut coordinates: Vec<Coord> = vec![];
    for i in 0..SIZE {
        for j in 0..SIZE {
            let coord = Coord { x: i, y: j, legal: true };
            coordinates.push(coord);
        }
    }
    coordinates
}

#[allow(unused_must_use)]
impl fmt::Display for Game {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //  Display game state (allows display with macros like println!)
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
        println!("{}", game);
    }
    game.reset();
}



/*****************
 INTEGRATION TESTS
******************/

#[cfg(test)]
mod integration_tests {
    use super::*;
    use more_asserts::{assert_ge};

    #[test]
    fn test_auto_move_reaches_endgame() {
        // Should result in an endgame within `Game::const SIZE` moves, 
        // otherwise, there is a halting error
        let mut max_moves = SIZE as isize;
        let mut game = Game::new();

        game.start(true, true);

        while (game.end_game == false) && (max_moves > -2) {
            game.update();
            max_moves -= 1;
        }
        assert_ge!(max_moves, 0)
    }
}


/***********
 UNIT TESTS
************/

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test reset()
    // TODO: test auto_move
    // TODO: test manual_move
    // TODO: test switch_player
    // TODO: test update()


    #[test]
    fn test_player_init_settings() {
        // Test that players are set with correct autoplay booleans
        let mut game = Game::new();
        game.start(true, true);
        assert_eq!(AutoPlay { play_type: [true, true], 
                              play_type_str: ["automatic".to_string(), "automatic".to_string()] }, 
                              game.auto_play);
        game.start(false, true);
        assert_eq!(AutoPlay { play_type: [false, true], 
                              play_type_str: ["manual".to_string(), "automatic".to_string()] }, 
                              game.auto_play);
        game.start(true, false);
        assert_eq!(AutoPlay { play_type: [true, false], 
                              play_type_str: ["automatic".to_string(), "manual".to_string()] }, 
                              game.auto_play);
        game.start(false, false);
        assert_eq!(AutoPlay { play_type: [false, false], 
                              play_type_str: ["manual".to_string(), "manual".to_string()] }, 
                              game.auto_play);
    }

    #[test]
    fn test_board_init_display() {
        // Test that init board displays correctly
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
        // Test that init board contains correct values
        let board: [[char; SIZE]; SIZE];
        board = [[' ', ' ', ' '], 
                [' ', ' ', ' '], 
                [' ', ' ', ' ']];
        let game = Game::new();
        assert_eq!(board, game.board);
    }
}
