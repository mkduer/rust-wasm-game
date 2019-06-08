/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

const DRAW = 9;
const SECONDS = 0.5;
const MS = 1000;
const DELAY = SECONDS * MS;

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

  // setup game play style by waiting for the user
  // to select automatic or manual play
  var auto_play = document.getElementById("auto");
  var manual_play = document.getElementById("manual");

  // settings for automatic play button
  auto_play.onclick = function() {
    console.log('auto play selected')
    player1_type = true;
    player2_type = true;
    manual = false;
    game.start(player1_type, player2_type)
    begin(game, reset, reset_btn, start_visible, start_collapsed, 
          players, board, trans_board, winner, manual)
  };

  // settings for manual play button
  manual_play.onclick = function() {
    console.log('manual play selected')
    player1_type = true;
    player2_type = false;
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

  if (manual) {
    // TODO: Test that these transforms work correctly
    winner.style.transform = 'translate(0%, -450%)'
    players.style.transform = 'translate(0%, -190%)'
    reset_btn.style.transform = 'translate(0%, -300%)'
  }
  start_visible.style.visibility = "collapse";
  start_collapsed.style.visibility = "visible";

  render(game, players, board, trans_board, manual);

  if (manual) {
    manual_dialogue.style.visibility = "visible";
    document.getElementById("manual-dialogue").textContent = "Where do you want to place your piece?";
    /* TODO
    manual_play(game, players, board, trans_board, winner,
        start_visible, start_collapsed, reset, reset_btn, manual_dialogue);
        */
  } else {
    auto_play(game, players, board, trans_board, winner,
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

async function manual_play(game, players, board, trans_board, winner, 
                           start_visible, start_collapsed, reset, reset_btn, manual_dialogue) {
  // start the game and loop until end game is reached
  var end_game = false; 
  var local_reset = false;

  do {
    await sleep();

    // listen for reset
    reset_btn.onclick = function() {
        local_reset = reset_all(game, start_visible, start_collapsed, players, board, 
                                trans_board, winner, reset, manual_dialogue);
    };
    
    // if reset button was not selected, continue game play
    if (!local_reset) {
      tick(game, board);
    }

    // check for end game
    end_game = game.get_end_game();
  } while (!end_game && !local_reset);

  // select next function based on whether the game
  // stopped by reset or by reaching the end game
  if (local_reset) {
    return listen();
  } else {
    return game_over_msg(game, winner);
  }
}

async function auto_play(game, players, board, trans_board, winner, 
                         start_visible, start_collapsed, reset, reset_btn, manual_dialogue) {
  // start the game and loop until end game is reached
  var end_game = false; 
  var local_reset = false;

  do {
    await sleep();

    // listen for reset
    reset_btn.onclick = function() {
        local_reset = reset_all(game, start_visible, start_collapsed, players, 
                                board, trans_board, winner, reset, manual_dialogue);
    };
    
    // if reset button was selected
    if (!local_reset) {
      tick(game, board);
    }

    // check for end game
    end_game = game.get_end_game();
  } while (!end_game && !local_reset);

  // select next function based on whether the game
  // stopped by reset or by reaching the end game
  if (local_reset) {
    return listen();
  } else {
    return game_over_msg(game, winner);
  }
}

function tick(game, board) {
  // Run the game for one "tick" or move 
  game.update();
  board.textContent = game.render_board();
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
}

function sleep() {
  // sleep equivalent in javascript, source:
  // https://stackoverflow.com/questions/951021/what-is-the-javascript-version-of-sleep
  return new Promise(resolve => setTimeout(resolve, DELAY));
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

// TODO
function reset_menu_transform() {

}