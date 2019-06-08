/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

const DRAW = 9;
const MS = 1000;
const AUTO_DELAY = 0.5 * MS;
const MANUAL_DELAY = 1.0 * MS;

document.getElementById("title").textContent = "Rusty Tic Tac Toe\nMagically Compiled to WASM";
listen();

function listen() {
  // listen for user selection of automatic/manual play
  var start_visible = document.getElementById("start-visible");
  var start_collapsed = document.getElementById("start-collapsed");
  var players = document.getElementById("players");
  var board = document.getElementById("board");
  var trans_board = document.getElementById("transparent-board");
  var winner = document.getElementById("winner");
  var reset_btn = document.getElementById("reset");
  var player1_type = true;
  var player2_type = true;
  var manual = false;
  var reset = false;

  var game = Game.new();
  winner.style.visibility = "collapse";

  // setup game play style by waiting for the user
  // to select automatic or manual play
  var auto_play = document.getElementById("auto");
  var manual_play = document.getElementById("manual");

  // settings for automatic play button
  auto_play.onclick = function() {
    player1_type = true;
    player2_type = true;
    manual = false;
    game.start(player1_type, player2_type)
    begin(game, reset, reset_btn, start_visible, start_collapsed, 
          players, board, trans_board, winner, manual)
  };

  // settings for manual play button
  manual_play.onclick = function() {
    player1_type = false;
    player2_type = true;
    manual = true;
    game.start(player1_type, player2_type)
    begin(game, reset, reset_btn, start_visible, start_collapsed, 
          players, board, trans_board, winner, manual)
  };
}

function begin(game, reset, reset_btn, start_visible, start_collapsed, 
               players, board, trans_board, winner, manual) {
  // begins the game by rendering it and playing the game ticks
  var manual_dialogue = document.getElementById("manual-dialogue");

  // display menu according to manual/automatic play
  if (manual) {
    winner.style.transform = 'translate(0%, -450%)';
    players.style.transform = 'translate(0%, -190%)';
    reset_btn.style.transform = 'translate(0%, -300%)';
  } else {
    winner.style.transform = 'translate(0%, 0%)';
    players.style.transform = 'translate(0%, 0%)';
    reset_btn.style.transform = 'translate(0%, 0%)';
  }

  // collapse visible start menu, display collapsed menu
  start_visible.style.visibility = "collapse";
  start_collapsed.style.visibility = "visible";

  // render game
  render(game, players, board, trans_board, manual);

  // initiate manual/automatic play
  if (manual) {
    manual_play(game, players, board, trans_board, winner, manual,
        start_visible, start_collapsed, reset, reset_btn, manual_dialogue);
  } else {
    auto_play(game, players, board, trans_board, winner, manual,
        start_visible, start_collapsed, reset, reset_btn, manual_dialogue);
  }
}

function render(game, players, board, trans_board, manual) {
  // render game content
  players.textContent = game.render_players();
  board.style.visibility = "visible";
  board.textContent = game.render_board();
  if (manual) {
    render_overlay(game, trans_board)
  }
}

function render_overlay(game, trans_board) {
  // render transparent overlay with indexed board
  trans_board.textContent = game.render_indexed_board();
  trans_board.style.visibility = "visible";
}

async function manual_play(game, players, board, trans_board, winner, manual,
                           start_visible, start_collapsed, reset, reset_btn, manual_dialogue) {
  // start the game and loop until end game is reached
  var end_game = false; 
  var local_reset = false;
  manual_dialogue.style.visibility = "visible";

  // listen for reset
  reset_btn.onclick = function() {
      local_reset = reset_all(game, start_visible, start_collapsed, players, board, 
                              trans_board, winner, reset, manual_dialogue);
  };

  do {
    document.getElementById("manual-dialogue").textContent = "Where do you want to place your piece?";
    await sleep(manual);
    
    // if reset button was not selected, continue game play
    if (!local_reset) {

      // listen for a valid keystroke by the user (or an `escape` equivalent)
      // 9 represents an invalid input value
      var success = manual_tick(game);
      end_game = game.get_end_game();

      // prompt for a valid keystroke
      while (success == 9 && !local_reset && !end_game) {
        document.getElementById("manual-dialogue").textContent = "Please select a valid cell number";
        success = manual_tick(game);
        end_game = game.get_end_game();
      }

      // user selected `escape` so reset game
      if (success == -1) {
        reset_all(game, start_visible, start_collapsed, players, 
                  board, trans_board, winner, reset, manual_dialogue);
        local_reset = true;
      // automated player
      } else {
        auto_tick(game, board);
        end_game = game.get_end_game();
      }
    }

    // TODO: update rendering of indexed board (in Rust) so that successful cell placement removes numeral
    board.textContent = game.render_board();

    // check for end game
  } while (!end_game && !local_reset);

  // select next function based on whether the game
  // stopped by reset or by reaching the end game
  if (local_reset) {
    return listen();
  } else {
    return game_over_msg(game, winner);
  }
}

function manual_tick(game) {
  // Run the game for one "tick" or move 
  var success = 9;
  var key = -2;
  
  // following code source: https://medium.com/@uistephen/keyboardevent-key-for-cross-browser-key-press-check-61dbad0a067a
  // a good article on dealing with deprecated `keyCode`, which is still ubiquitous in browsers AND allowing
  // for the spec suggested `key` as well as why to use `keyup`
  window.addEventListener('keyup', (e) => {
    if (e.defaultPrevented) {
      return;
    }
    key = e.key || e.keyCode;

    switch (key) {
      // handle escape key presses
      case "Escape": 
      case "esc": 
      case "27": 
        return -1;
      case "0":
      case "1":
      case "2":
      case "3":
      case "4":
      case "5":
      case "6":
      case "7":
      case "8":
        success = game.update(key);
        if (success == 9) {
          break;
        }
        return success;
      case "Enter":
      default: 
        success = 9;
    }
  });
}

async function auto_play(game, players, board, trans_board, winner, manual,
                         start_visible, start_collapsed, reset, reset_btn, manual_dialogue) {
  // start the game and loop until end game is reached
  var end_game = false; 
  var local_reset = false;

  // listen for reset
  reset_btn.onclick = function() {
      local_reset = reset_all(game, start_visible, start_collapsed, players, 
                              board, trans_board, winner, reset, manual_dialogue);
  };
    
  do {
    await sleep(manual);

    // if reset button was selected
    if (!local_reset) {
      end_game = auto_tick(game, board);
    }
  } while (!end_game && !local_reset);

  // select next function based on whether the game
  // stopped by reset or by reaching the end game
  if (local_reset) {
    return listen();
  } else {
    return game_over_msg(game, winner);
  }
}

function auto_tick(game, board) {
  // Run the game for one "tick" or move 
  var success = game.update();
  if (success > -1 || success < 9) {
    board.textContent = game.render_board();
  } else {
    throw "auto_tick function failed to update board";
  }
  return game.get_end_game();
}

function game_over_msg(game, winner) {
  // displays whether the game ended in a draw or a win,
  // as well as the winner in the latter case

  const game_over = game.get_winner();
  if (game_over === DRAW) {
    winner.textContent = game.declare_draw();  
  } else {
    winner.textContent = game.declare_winner();  
  }
  winner.style.visibility = "visible";
}

function sleep(manual) {
  // sleep equivalent in javascript, source:
  // https://stackoverflow.com/questions/951021/what-is-the-javascript-version-of-sleep
  var delay = AUTO_DELAY;

  if (manual) {
    delay = MANUAL_DELAY;
  }
  return new Promise(resolve => setTimeout(resolve, delay));
}

function reset_all(game, start_visible, start_collapsed, players, board, 
                   trans_board, winner, reset, manual_dialogue) {
  // resets the game settings
  reset = true;
  game.reset();
  start_visible.style.visibility = "visible";
  start_collapsed.style.visibility = "collapse";
  manual_dialogue.style.visibility = "collapse";
  board.style.visibility = "collapse";
  trans_board.style.visibility = "collapse";

  players.textContent = "";
  board.textContent = "";
  winner.textContent = "";
  manual_dialogue.textContent = "";

  return reset;
}