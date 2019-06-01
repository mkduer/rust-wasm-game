# tic-tac-toe

Ths program uses Rust to generate a command-line tic-tac-toe game allowing for manual and/or automatic play by two players. 

## Build 

Clone or fork the repo: `git clone git@github.com:mkduer/rust-wasm-game.git`

Go into the **tic-tac-toe** directory and build with dependencies: 
`cd tic-tac-toe`  
`cargo build`  

## Play

Run the game: `cargo run`

The current settings setup an automated Player 1 for a manual Player 2 to play against with the current settings: 

`const P1_PLAY bool = false;  // automatic`  
`const P2_PLAY bool = true;  // manual`

The player settings can be changed with different boolean values for the constants `P1_PLAY` and `P2_PLAY` on lines 5-6 of `main.rs`

## Test

Test the program by running the unit tests: `cargo test`
