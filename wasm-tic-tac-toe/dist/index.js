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

  var player1_type = true;
  var player2_type = true;
  var reset = false;

  // setup game play style by waiting for the user
  // to select automatic or manual play
  var auto_play = document.getElementById("auto");
  var manual_play = document.getElementById("manual");

  // settings for automatic play button
  auto_play.onclick = function() {
    console.log('auto play selected')
    player1_type = true;
    player2_type = true;
    main(player1_type, player2_type, reset)
  };

  // settings for manual play button
  manual_play.onclick = function() {
    console.log('manual play selected')
    player1_type = true;
    player2_type = false;
    main(player1_type, player2_type, reset)
  };
}

function main(player1_type, player2_type, reset) {
  var start_visible = document.getElementById("start-visible");
  var start_collapsed = document.getElementById("start-collapsed");
  var players = document.getElementById("players");
  var game_state = document.getElementById("game-state");
  var winner = document.getElementById("winner");

  var game = Game.new();

  start_visible.style.visibility = "collapse";
  start_collapsed.style.visibility = "visible";

  render(game, players, board, game_state);
  play(game, players, board, game_state, winner,
       start_visible, start_collapsed, reset);

  console.log('game done');
}

function render(game, players, board, game_state) {
  players.textContent = game.render_players();
  board.textContent = game.render_board();
  game_state.textContent = game.render_game_state();
}

async function play(game, players, board, game_state, winner, start_visible, start_collapsed, reset) {
  // start the game and loop until end game is reached
  var reset_btn = document.getElementById("reset");
  var end_game = false; 
  var local_reset = false;

  do {
    await sleep();
    // if the reset button is selected
    reset_btn.onclick = function() {
        local_reset = reset_all(game, start_visible, start_collapsed, players, board, game_state, winner, reset);
    };

    if (local_reset === false) {
      tick(game, board, game_state);
    }
    end_game = game.get_end_game();
  } while (!end_game);

  if (local_reset) {
    console.log('GAME was RESET');
    return listen();
  } else {
    return game_over_msg(game, winner);
  }
}

function tick(game, board, game_state) {
  // Run the game for one "tick" or move 
  game.update();
  board.textContent = game.render_board();
  game_state.textContent = game.render_game_state();
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

function reset_all(game, start_visible, start_collapsed, players, board, game_state, winner, reset) {
  reset = true;
  game.reset();
  start_visible.style.visibility = "visible";
  start_collapsed.style.visibility = "collapse";
  players.textContent = "";
  board.textContent = "";
  game_state.textContent = "";
  winner.textContent = "";
  return reset;
}