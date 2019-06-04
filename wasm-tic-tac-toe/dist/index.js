/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

const draw = 9;
const seconds = 1;
const ms = 1000;
const delay = seconds * ms;

var game = Game.new();

// create canvas of game title, board, etc
const title = document.getElementById("title");
var players = document.getElementById("players");
var board = document.getElementById("board");
var game_state = document.getElementById("game-state");
var winner = document.getElementById("winner");
var end_game = game.end_game();

// render element content
title.textContent = game.title();
players.textContent = game.render_players();
board.textContent = game.render_board();
game_state.textContent = game.render_game_state();

run_game(game);

async function run_game(game) {
  // start the game and loop until end game is reached
  game.start(true, true)

  console.log('before tick loop');
  do {
    await sleep(delay);
    tick();

    console.log('checking for end game');
    end_game = game.end_game();
  } while (end_game == false);

  console.log('game over');
  game_over_msg();
  console.log('resetting game');
  game.reset();
}

function tick() {
  // Run the game for one "tick" or move 
  console.log('updating game');
  game.update();
  board.textContent = game.render_board();
  game_state.textContent = game.render_game_state();
}

function game_over_msg() {
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
