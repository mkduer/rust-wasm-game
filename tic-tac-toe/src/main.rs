use std::fmt;
use rand::{thread_rng, Rng};
use std::io::{stdin, stdout, Write};

const P1_PLAY: bool = false; // player 1 is set to `false` for automated play to start
const P2_PLAY: bool = true;  // player 2 is set to `true` for manual play to start
const P1: char = 'X';       // player 1's piece
const P2: char = 'O';       // player 2's piece
const SIZE: usize = 3;      // row/col sizes for board
const NO_WIN: usize = 9;    // default, invalid value to represent no winner
const BOARD_IDX: [[char; 3]; 3] = [['0', '1', '2'],     // indexed board for user's reference
                                   ['3', '4', '5'], 
                                   ['6', '7', '8']];


#[derive(Debug, PartialEq)]
struct AutoPlay {
    // Struct detailing whether players move automatically or manually
    play_type: [bool; 2],           // True: automatic random moves, False: manual moves
    play_type_str: [String; 2],     // "automatic" or "manual"
}

impl Default for AutoPlay {
    fn default() -> AutoPlay {
        // By default, both players are automated to play random, legal moves
        AutoPlay{ play_type: [true, true], 
                  play_type_str: ["automatic".to_string(), "automatic".to_string()]}
    }
}

#[derive(Debug, PartialEq)]
struct Coord {
    // Struct for mapping array indices to coordinates
    x: usize,       // x-coordinate
    y: usize,       // y-coordinate
    legal: bool,    // flag: True if it is legal to place a piece on the coordinate, False if coordinate is already full
}

#[derive(Debug, PartialEq)]
struct WinState {
    // Struct containing representations of win states
    p1_win_state: Vec<char>,    // represents player 1's win state
    p2_win_state: Vec<char>,    // represents player 2's win state
}

impl Default for WinState {
    fn default() -> WinState {
        // Generates winning state sized to default `SIZE`
        let mut win_state = WinState { p1_win_state: vec![], p2_win_state: vec![] };
        for _i in 0..SIZE {
            win_state.p1_win_state.push(P1);
            win_state.p2_win_state.push(P2);
        }
        win_state
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    // Struct with tic-tac-toe game settings and components
    board: [[char; SIZE]; SIZE],    // tic tac toe board
    curr_player: usize,             // current player 
    players: [char; 2],             // players represented by pieces
    auto_play: AutoPlay,            // type of play for each player
    end_game: bool,                 // game status: False if in play, True if ended by win/draw
    coordinates: Vec<Coord>,        // coordinates for moves
    win_states: WinState,           // win states for players
    winner: usize,                  // specifies winner if there is one
}

impl Game {
    fn new() -> Self {
        // Initializes the game board, first player piece and default autoplay for both players
        Self {
            board: [[' ', ' ', ' '], 
                    [' ', ' ', ' '], 
                    [' ', ' ', ' ']],
            curr_player: 0,
            players: [P1, P2],
            auto_play: AutoPlay::default(), 
            end_game: false,
            coordinates: coord_mapping(),
            win_states: WinState::default(),
            winner: NO_WIN,
        }
    }

    fn start(&mut self, p1_auto: bool, p2_auto: bool) {
        // Set the automatic/manual play settings for each player in order to start the game
        // Create the coordinates per game size
        self.auto_play.play_type = [p1_auto, p2_auto];

        match self.auto_play {
            AutoPlay { play_type: [false, false], .. } => {
                self.auto_play.play_type_str = ["manual".to_string(), "manual".to_string()];
                println!("\nPlayer 1 :: {} ({} play)\nPlayer 2 :: {} ({} play)\n", P1, &self.auto_play.play_type_str[0], 
                                                                                   P2, &self.auto_play.play_type_str[1]);
            },
            AutoPlay { play_type: [true, false], .. } => {
                self.auto_play.play_type_str = ["automatic".to_string(), "manual".to_string()];
                println!("\nPlayer 1 :: {} ({} play)\nPlayer 2 :: {} ({} play)\n", P1, &self.auto_play.play_type_str[0], 
                                                                                   P2, &self.auto_play.play_type_str[1]);
            },
            AutoPlay { play_type: [false, true], .. } => {
                self.auto_play.play_type_str = ["manual".to_string(), "automatic".to_string()];
                println!("\nPlayer 1 :: {} ({} play)\nPlayer 2 :: {} ({} play)\n", P1, &self.auto_play.play_type_str[0], 
                                                                                   P2, &self.auto_play.play_type_str[1]);
            },
            AutoPlay { .. } => {
                self.auto_play.play_type_str = ["automatic".to_string(), "automatic".to_string()];
                println!("\nPlayer 1 :: {} ({} play)\nPlayer 2 :: {} ({} play)\n", P1, &self.auto_play.play_type_str[0], 
                                                                                   P2, &self.auto_play.play_type_str[1]);
            },
        }
    }

    fn update(&mut self) { 
        // Have the current player choose a location for their move 
        let loc: usize = match &self.auto_play.play_type[self.curr_player] {
            true => self.auto_move(),
            false => self.manual_move(),
        };

        // Update the board and coordinates
        let x = self.coordinates[loc].x;
        let y = self.coordinates[loc].y;
        self.board[x][y] = self.players[self.curr_player];
        self.coordinates[loc].legal = false;

        // Check for endgame and change players
        self.end_game = self.is_endgame();
        self.curr_player = self.switch_player();
    }

    fn switch_player(&mut self) -> usize {
        // Switch current player
        if self.curr_player == 0 { 1 } else { 0 }
    }

    fn auto_move(&mut self) -> usize {
        // Automated Move: Return the location (index) for a random, legal move
        let max_rng = SIZE * SIZE;
        let mut rng = thread_rng();
        let mut loc = rng.gen_range(0, max_rng);
        let mut valid: bool = self.coordinates[loc].legal;

        // Make sure the move is valid
        while valid == false {
            loc = rng.gen_range(0, max_rng);
            valid = self.coordinates[loc].legal;
        }
        loc
    }

    fn manual_move(&mut self) -> usize {
        // Manual Move: Ask the user for the location where they want to place their piece
        println!("\nWhere do you want to place your piece? ");
        self.display_indexed_board();

        // Get user's choice for piece placement
        let mut loc = self.get_user_input();
        let mut valid: bool = self.coordinates[loc].legal;

        // Make sure the move is valid
        while valid == false {
            println!("\nA piece is already placed there. Please enter a valid location: ");
            loc = self.get_user_input();
            valid = self.coordinates[loc].legal;
        }
        loc
    }

    fn display_indexed_board(&mut self) {
        // Displays board with index repreesentations for placing pieces
        let mut total_lines = &SIZE - 1;
        for row in &BOARD_IDX {
            println!("  {} | {} | {}", row[0], row[1], row[2]);
            if total_lines > 0 {
                println!(" -----------");
                total_lines -= 1;
            }
        }
        println!("\n");
    }

    fn get_user_input(&mut self) -> usize {
        // Grabs the user's move from stdin and checks for validity
        let mut stdout = stdout();
        let stdin = stdin();
        let _clean = match stdout.flush() {
            Ok(result) => result,
            Err(error) => panic!("Unable to flush buffer, {}", error),
        };

        // Grab user's response
        let mut user_response = String::with_capacity(100);
        stdin.read_line(&mut user_response).unwrap();

        // Check that the input was a valid character
        let mut first_char = user_response.chars().next().unwrap();
        match &mut first_char {
            '0' ... '8' => {
                println!("You entered: {}", first_char);
                first_char.to_digit(10).unwrap() as usize
            },
            _ => { 
                println!("\nPlease enter a valid response: ");
                self.get_user_input()
            },
        }
    }

    fn is_endgame(&mut self) -> bool {
        // Checks for end game win/draw states returning True if an endgame is reached, False otherwise
        let total_states = SIZE + SIZE + 2;
        let slices = [[0, 1, 2], [3, 4, 5], [6, 7, 8],
                      [0, 3, 6], [1, 4, 7], [2, 5, 8],
                      [0, 4, 8], [2, 4, 6]];
        let mut board_slice: Vec<char> = vec![];

        // Pass list of arrays as rows and loop over them to check for win state
        for slice in 0..total_states {
            // e.g. [0, 1, 2]
            let state = &slices[slice];

            for index in 0..SIZE {
                // e.g. 0
                let loc = state[index];
                let x = self.coordinates[loc].x;
                let y = self.coordinates[loc].y;
                board_slice.push(self.board[x][y])
            }

            // Check the current slice of the board for a winning state
            if self.is_win(&board_slice) == true {
                board_slice.clear();
                return true;
            }
            board_slice.clear();
        }

        // if the board is full, check for drawn state
        self.is_draw()
    }

    fn is_draw(&mut self) -> bool {
        // Checks for drawn states and returns True if a drawn state is reached, False otherwise
        let mut is_drawn = true;
        for row in self.board.iter() {
            is_drawn = match row {
                [' ', _, _] | [_, ' ', _] | [_, _, ' '] => false,  // board is not full
                _ => true,                                         // board is full
            };
            if is_drawn == false {
                return false;
            }
        }
        is_drawn
    }

    fn is_win(&mut self, row: &Vec<char>) -> bool {
        // Checks for win states and returns True if a win state is reached, False otherwise
        if row == &self.win_states.p1_win_state {
            self.winner = 0;
            self.declare_winner();
            return true;
        }
        if row == &self.win_states.p2_win_state {
            self.winner = 1;
            self.declare_winner();
            return true;
        }
        false
    }

    fn declare_winner(&mut self) {
        // Declares a winner
        println!("\nWINNER: Player {} won the game!", self.players[self.winner]);
    }

    fn reset(&mut self) {
        // Reset Game
        self.board = [[' ', ' ', ' '], 
                     [' ', ' ', ' '], 
                     [' ', ' ', ' ']];
        self.curr_player = 0;
        self.end_game = false;
        self.coordinates.clear();
        self.coordinates = coord_mapping();
        self.winner = NO_WIN;
    }
}

fn coord_mapping() -> Vec<Coord> {
    // Generates a coordinate mapping of vector indices -> coordinates on the board
    // and `legal` represents whether a square is available for placing a piece (True)
    // or already has a piece placed on it (False)
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
        if self.end_game == true {
            game_status = "ended".to_string();
        }

        let mut total_lines = &SIZE - 1;
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
    // Play the game until an endgame state is reached
    let mut game = Game::new();
    game.start(P1_PLAY, P2_PLAY);
    println!("{}", game);

    while game.end_game == false {
        game.update();
        println!("{}", game);
    }
    game.reset();
}


/***********
 UNIT TESTS
************/
#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::{assert_ge};

    #[test]
    fn test_board_init() {
        // Test that init board contains correct values
        let board: [[char; SIZE]; SIZE];
        board = [[' ', ' ', ' '], 
                [' ', ' ', ' '], 
                [' ', ' ', ' ']];
        let game = Game::new();
        assert_eq!(board, game.board);
    }

    #[test]
    fn test_start() {
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
    fn test_game_display() {
        // Test that init board displays correctly
        let mut game = Game::new();
        game.start(true, true);

        let mut game_status = format!("Game in play, {}'s turn", &game.players[game.curr_player]);
        if game.end_game {
            game_status = "Game ended".to_string();
        }
        let mut expect_board = format!("\n{}:", &game_status);
        expect_board += "\n    |   |  \n -----------\n    |   |  \n -----------\n    |   |  \n";
        assert_eq!(expect_board, format!("{}", game));
    }

    #[test]
    fn test_auto_move_reaches_endgame() {
        // Should result in an endgame within Game const `SIZE * SIZE` moves, 
        // otherwise, there is a halting error, which is caught by the while loop's
        // `max_moves > -2` stopping condition. If this stopping condition is used
        // the assert will fail.
        let mut max_moves = (SIZE * SIZE) as isize;
        let mut game = Game::new();

        game.start(true, true);

        while (game.end_game == false) && (max_moves > -2) {
            println!("max_moves: {:?}, end_game: {:?}", max_moves, game.end_game);
            game.update();
            max_moves -= 1;
        }
        println!("exited while loop");
        println!("max_moves: {:?}, end_game: {:?}", max_moves, game.end_game);
        assert_ge!(max_moves, 0)
    }

    #[test]
    fn test_is_win_false() {
        // Tests that the function returns a `false` to signify a win has not occurred
        // for various scenarios
        let mut game = Game::new();

        let mut test_row: Vec<char> = vec![P1, P2, P2];
        assert_ne!(game.is_win(&test_row), true);
        test_row.clear();

        test_row = vec![P1, P1, P2];
        assert_ne!(game.is_win(&test_row), true);
        test_row.clear();

        test_row = vec![P2, P1, P2];
        assert_ne!(game.is_win(&test_row), true);
        test_row.clear();

        test_row = vec![' ', P1, P1];
        assert_ne!(game.is_win(&test_row), true);
        test_row.clear();

        test_row = vec![P1, ' ', P1];
        assert_ne!(game.is_win(&test_row), true);
        test_row.clear();

        test_row = vec![P2, P2, ' '];
        assert_ne!(game.is_win(&test_row), true);
    }

    #[test]
    fn test_is_win_true() {
        // Tests that the function returns a `true` to signify a win has occurred
        let mut game = Game::new();

        let mut test_row: Vec<char> = vec![P1, P1, P1];
        assert_eq!(game.is_win(&test_row), true);
        test_row.clear();

        test_row = vec![P2, P2, P2];
        assert_eq!(game.is_win(&test_row), true);
        test_row.clear();
    }

    #[test]
    fn test_is_win_none() {
        // Test for correct default value when there is no winner
        let mut game = Game::new();
        let test_row: Vec<char> = vec![' ', P1, ' '];
        let _game_won = game.is_win(&test_row);
        assert_eq!(game.winner, NO_WIN);
    }

    #[test]
    fn test_p2_win_state() {
        // Test for correct winner when P2 wins
        let mut game = Game::new();
        let test_row: Vec<char> = vec![P2, P2, P2];
        let _game_won = game.is_win(&test_row);
        assert_eq!(game.players[game.winner], P2);
    }

    #[test]
    fn test_p1_win_state() {
        // Test for correct winner when P1 wins
        let mut game = Game::new();
        let test_row: Vec<char> = vec![P1, P1, P1];
        let _game_won = game.is_win(&test_row);
        assert_eq!(game.players[game.winner], P1);
    }


    #[test]
    fn test_declare_winner_p1() {
        // Test that the correct winner was declared 
        // by testing the printed variable's value (left-hand assert_eq param)
        // with the expected value (right-hand assert_eq param)
        let mut game = Game::new();
        let test_row: Vec<char> = vec![P1, P1, P1];
        let _game_won = game.is_win(&test_row);
        game.declare_winner();
        assert_eq!(game.players[game.winner], P1);
    }

    #[test]
    fn test_declare_winner_p2() {
        // Test that the correct winner was declared 
        // by testing the printed variable's value (left-hand assert_eq param)
        // with the expected value (right-hand assert_eq param)
        let mut game = Game::new();
        let test_row: Vec<char> = vec![P2, P2, P2];
        let _game_won = game.is_win(&test_row);
        game.declare_winner();
        assert_eq!(game.players[game.winner], P2);
    }

    #[test]
    fn test_reset() {
        // Tests if the game resets correctly to its original values after being played
        // by comparing with another unplayed game instantiated with the same initial values
        let mut original_game = Game::new();
        original_game.start(true, true);

        while original_game.end_game == false {
            original_game.update();
            println!("{}", original_game);
        }
        original_game.reset();

        let mut comparison_game = Game::new();
        comparison_game.start(true, true);
        assert_eq!(original_game, comparison_game);
    }

    #[test]
    fn test_is_draw_true() {
        // Tests that the function returns a `true` to signify a draw has occurred
        let mut game = Game::new();

        game.board = [['O', 'X', 'O'],
                      ['O', 'X', 'X'],
                      ['X', 'O', 'X']];
        assert_eq!(game.is_draw(), true);

        game.board = [['X', 'X', 'O'],
                      ['O', 'O', 'X'],
                      ['X', 'X', 'O']];
        assert_eq!(game.is_draw(), true);
    }

    #[test]
    fn test_is_draw_false() {
        // Tests that the function returns a `true` to signify a draw has occurred
        let mut game = Game::new();

        game.board = [['X', 'O', 'O'],
                      ['O', 'X', ' '],
                      ['X', 'X', ' ']];
        assert_ne!(game.is_draw(), true);

        game.board = [[' ', ' ', ' '],
                      [' ', ' ', ' '],
                      [' ', ' ', ' ']];
        assert_ne!(game.is_draw(), true);

        game.board = [['O', 'X', 'X'],
                      ['O', 'X', 'O'],
                      [' ', ' ', 'X']];
        assert_ne!(game.is_draw(), true);
    }

    #[test]
    fn test_switch_player() {
        // Tests that players are switched correctly
        let mut game = Game::new();
        game.curr_player = 0;
        assert_eq!(game.switch_player(), 1);

        game.curr_player = 1;
        assert_eq!(game.switch_player(), 0);
    }

    #[test]
    fn test_end_game_true() {
        // Tests that is_endgame returns True if a win/drawn state is reached
        let mut game = Game::new();

        // drawn end game
        game.board = [['X', 'O', 'O'],
                      ['O', 'X', 'X'],
                      ['X', 'O', 'X']];
        assert_eq!(game.is_endgame(), true);

        // won end game
        game.board = [['X', 'O', 'O'],
                      ['O', 'X', 'O'],
                      ['X', 'X', 'X']];
        assert_eq!(game.is_endgame(), true);
    }

    #[test]
    fn test_end_game_false() {
        // Tests that is_endgame returns False if the board is incomplete with no win
        let mut game = Game::new();

        game.board = [['O', ' ', 'O'],
                      ['O', 'X', 'X'],
                      ['X', 'O', 'X']];
        assert_eq!(game.is_endgame(), false);

        game.board = [['O', 'X', 'X'],
                      [' ', ' ', 'X'],
                      [' ', ' ', 'O']];
        assert_eq!(game.is_endgame(), false);

        game.board = [['X', 'O', 'O'],
                      ['X', 'O', 'X'],
                      [' ', 'X', ' ']];
        assert_eq!(game.is_endgame(), false);
    }
}
