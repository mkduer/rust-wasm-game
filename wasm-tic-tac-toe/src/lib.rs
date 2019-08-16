/*
*  reference for starting code:
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

mod utils;

use wasm_bindgen::prelude::*;
use rand::{thread_rng, Rng};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const P1: char = 'X';           // player 1's piece
const P2: char = 'O';           // player 2's piece
const SIZE: usize = 3;          // row/col sizes for board
const NO_WIN: usize = 9;        // default, invalid value to represent no winner
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

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Game {
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

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        // Allows for console.log debugging of Rust
        utils::set_panic_hook();

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

    pub fn start(&mut self, p1_auto: bool, p2_auto: bool) {
        // Set the automatic/manual play settings for each player in order to start the game
        // Create the coordinates per game size
        self.end_game = false;
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

    pub fn render_players(&self) -> String {
        // Render players and their playing type (e.g. automatic/manual)
        let status: String = format!("Player 1 :: {} ({} play)\nPlayer 2 :: {} ({} play)", 
                                     P1, &self.auto_play.play_type_str[0], 
                                     P2, &self.auto_play.play_type_str[1]);
        status
    }


    pub fn render_board(&self) -> String {
        // Render board as a string (for WASM)
        let mut board_state: String = "".to_string();
        let mut total_lines = &SIZE - 1;
        for row in &self.board {
            board_state += &format!("\n {} ║ {} ║ {}\n", row[0], row[1], row[2]);
            if total_lines > 0 {
                board_state += &format!("═══╬═══╬═══\n");
                total_lines -= 1;
            }
        }
        board_state += &format!("\n");
        board_state
    }

    pub fn render_indexed_board(&mut self) -> String {
        // Render indexed board as a string (for WASM) to allow for keystrokes
        let mut board_state: String = "".to_string();
        let mut total_lines = &SIZE - 1;
        for row in &BOARD_IDX {
            board_state += &format!("\n {}   {}   {}\n", row[0], row[1], row[2]);
            if total_lines > 0 {
                board_state += &format!("           \n");
                total_lines -= 1;
            }
        }
        board_state += &format!("\n");
        board_state
    }

    pub fn update(&mut self, manual_move: usize) -> usize { 
        // Have the current player choose a location for their move 
        let selected_move: usize = manual_move;
        let loc: usize = match &self.auto_play.play_type[self.curr_player] {
            true => self.auto_move(),
            false => self.manual_move(selected_move),
        };

        if loc == 9 {
            return loc;
        }

        // Update the board and coordinates
        let x = self.coordinates[loc].x;
        let y = self.coordinates[loc].y;
        self.board[x][y] = self.players[self.curr_player];
        self.coordinates[loc].legal = false;

        // Check for endgame and change players
        self.end_game = self.is_endgame();
        self.curr_player = self.switch_player();

        loc
    }

    pub fn get_end_game(&self) -> bool {
        // Returns end_game boolean
        self.end_game
    }

    pub fn get_winner(&self) -> usize {
        // Returns winner value
        self.winner
    }

    pub fn declare_draw(&self) -> String {
        // Drawn game message
        "DRAW: nobody wins".to_string()
    }

    pub fn declare_winner(&self) -> String {
        // Winner message
        let winner = format!("Player {} is the WINNER!", self.players[self.winner]);
        winner
    }

    pub fn reset(&mut self) {
        // Reset Game
        self.board = [[' ', ' ', ' '], 
                     [' ', ' ', ' '], 
                     [' ', ' ', ' ']];
        self.curr_player = 0;
        self.end_game = true;
        self.coordinates.clear();
        self.coordinates = coord_mapping();
        self.winner = NO_WIN;
    }
}

impl Game {

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

    fn manual_move(&mut self, selected_move: usize) -> usize {
        // Manual Move: Ask the user for the location where they want to place their piece

        // Make sure the move is valid
        match &selected_move {
            0 ... 8 => selected_move,
            _ => 9,
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
            if !is_drawn {
                return false;
            }
        }
        is_drawn
    }

    fn is_win(&mut self, row: &Vec<char>) -> bool {
        // Checks for win states and returns True if a win state is reached, False otherwise
        if row == &self.win_states.p1_win_state {
            self.winner = 0;
            return true;
        }
        if row == &self.win_states.p2_win_state {
            self.winner = 1;
            return true;
        }
        false
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
