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

// 
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
