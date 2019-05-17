use std::fmt;

const SIZE: usize = 3;

struct Board {
    squares: [[char; SIZE]; SIZE],
}

impl Board {
    fn new() -> Self {
        Self {
            squares: [[' ', ' ', ' '], 
                      [' ', ' ', ' '], 
                      [' ', ' ', ' ']]
        }
    }
}

#[allow(unused_must_use)]
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut total_lines = &SIZE - 1;
        write!(f, "\n");
        for row in &self.squares {
            write!(f, "  {} | {} | {}\n", row[0], row[1], row[2]);
            if total_lines > 0 {
                write!(f, " -----------\n");
                total_lines -= 1;
            }
        }
        Ok(())
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board);
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
    let init_board = Board::new();
    assert_eq!(expect_board, format!("{}", init_board));
}

#[test]
fn test_squares_vals() {
    let squares: [[char; 3]; 3];
    squares = [[' ', ' ', ' '], 
               [' ', ' ', ' '], 
               [' ', ' ', ' ']];
    let init_board = Board::new();
    assert_eq!(squares, init_board.squares);
}