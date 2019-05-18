use std::fmt;

const SIZE: usize = 3;

struct AutoPlay {
    player1: bool,  // True: random moves, False: manual moves
    player2: bool,  // True: random moves, False: manual moves
}

impl Default for AutoPlay {
    fn default() -> AutoPlay {
        // By default, both players are automated to play random, legal moves
        AutoPlay{player1: true, player2: true}
    }
}

struct Game {
    board: [[char; SIZE]; SIZE],    // tic tac toe board
    player: char,                   // current player
    auto_play: AutoPlay,            // type of play for each player
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
        }
    }

    fn start(&mut self, p1_auto: bool, p2_auto: bool) {
        // Set the automatic/manual play settings for each player in order to start the game
        self.auto_play.player1 = p1_auto;
        self.auto_play.player2 = p2_auto;

        match self.auto_play {
            AutoPlay {player1: false, player2: false} => 
                println!("player1: {}, player2: {}", self.auto_play.player1, self.auto_play.player2),
            AutoPlay {player1: true, player2: false} => 
                println!("player1: {}, player2: {}", self.auto_play.player1, self.auto_play.player2),
            AutoPlay {player1: false, player2: true} => 
                println!("player1: {}, player2: {}", self.auto_play.player1, self.auto_play.player2),
            AutoPlay {player1: _, player2: _} => 
                println!!("Default settings: automated player1 and player2 will make random, legal moves."),
        }
    }
}

#[allow(unused_must_use)]
impl fmt::Display for Game {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //  Display for board (can be used with println! and similar functions
        //  to print the tic tac toe board)
        let mut total_lines = &SIZE - 1;
        write!(formatter, "\n");
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
    println!("{}", game);
    game.start(true, true);
}



#[test]
fn test_board_display() {
    // note: decreased indent for raw-string
    let expect_board = r#"
    |   |  
 -----------
    |   |  
 -----------
    |   |  
"#;
    let init_board = Game::new();
    assert_eq!(expect_board, format!("{}", init_board));
}

#[test]
fn test_board_vals() {
    let board: [[char; 3]; 3];
    board = [[' ', ' ', ' '], 
               [' ', ' ', ' '], 
               [' ', ' ', ' ']];
    let init_board = Game::new();
    assert_eq!(board, init_board.board);
}
