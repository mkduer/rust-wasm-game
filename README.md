# rust-wasm-game

The basic goal of this program is to create a tic-tac-toe game allowing for manual and/or automatic play by two players. The long-term goal if this program is compilingn the Rust code over to WASM and playable through the web browser.

## Instructions
* Clone or fork the repo:  

   `git clone git@github.com:mkduer/rust-wasm-game.git`

* Go into the **tic-tac-toe** directory and build with dependencies
  
    `cargo build`

* Run the game
  
   `cargo run`

   The current settings setup an automated Player 1 for a manual Player 2 to play against with the current settings: 

   `const P1_PLAY bool = false;`
   `const P2_PLAY bool = true;`

   To change these settings, the constant booleans `P1_PLAY` and `P2_PLAY` can be altered for full manual or full automatic play.

* Test the program by running the unit tests
  `cargo test`

  
## Next Steps
* Compile to WASM