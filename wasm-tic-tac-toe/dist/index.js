/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

const draw = 9;
const seconds = 1;
const ms = 1000;
const delay = seconds * ms;

var player1_type = true;
var player2_type = true;
var start = false;

document.getElementById("title").textContent = "Rusty Tic Tac Toe\nMagically Compiled to WASM";

// setup game play style by waiting for the user
// to select automatic or manual play
var auto_play = document.getElementById("auto");
var manual_play = document.getElementById("manual");

// settings for automatic play
auto_play.onclick = function() {
  console.log('auto play selected')
  player1_type = true;
  player2_type = true;
  start = true
  main(player1_type, player2_type, start)
};

// settings for manual play
manual_play.onclick = function() {
  console.log('manual play selected')
  player1_type = true;
  player2_type = false;
  start = true
  main(player1_type, player2_type, start)
};

function main(player1_type, player2_type, start) {
  console.log('main() function\n');
  var start_visible = document.getElementById("start-visible");
  var start_collapsed = document.getElementById("start-collapsed");
  var players = document.getElementById("players");
  var game_state = document.getElementById("game-state");
  var winner = document.getElementById("winner");

  var game = Game.new();

  console.log('trying to hide visible buttons')
  start_visible.style.visibility = "collapse";
  console.log('trying to show hidden player type details')
  start_collapsed.style.visibility = "visible";

  console.log('start = ' + start);
  if (start) {
    console.log('player 1: ' + player1_type + 
                ' player 2: ' + player2_type + 
                ' start: ' + start);



    render(game, players, board, game_state);
    play(game, board, game_state, winner);

    console.log('game done');
  }
}

function render(game, players, board, game_state) {
  console.log('render() function\n');
  players.textContent = game.render_players();
  board.textContent = game.render_board();
  game_state.textContent = game.render_game_state();
}

async function play(game, board, game_state, winner) {
  console.log('play() function\n');
  // start the game and loop until end game is reached
  var end_game = false; 

  do {
    await sleep(delay);
    tick(game, board, game_state);
    end_game = game.end_game();
  } while (end_game == false);
  game_over_msg(game, winner);
}

function tick(game, board, game_state) {
  console.log('tick() function');
  // Run the game for one "tick" or move 
  game.update();
  board.textContent = game.render_board();
  game_state.textContent = game.render_game_state();
}

function game_over_msg(game, winner) {
  console.log('game_over_msg() function\n');
  // displays whether the game ended in a draw or a win,
  // as well as the winner in the latter case
  const game_over = game.game_over();
  if (game_over == draw) {
    winner.textContent = game.declare_draw();  
  } else {
    winner.textContent = game.declare_winner();  
  }
}

function sleep(ms) {
  // sleep equivalent in javascript, source:
  // https://stackoverflow.com/questions/951021/what-is-the-javascript-version-of-sleep
  return new Promise(resolve => setTimeout(resolve, ms));
}

function reset(game, start_visible, start_collapsed, start) {
  console.log('reset() function\n');
  game.reset();
  start_visible.style.visibility = "visible";
  start_collapsed.style.visibility = "collapse";
  start = false;
}