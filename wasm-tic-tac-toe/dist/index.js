/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

var game = Game.new();

// set variables
const draw = 9;
const title = document.getElementById("title");
const seconds = 5;
const ms = 1000;

// start game
game.start(true, true)

// create canvas of game title, board, etc
title.textContent = game.title();
var players = document.getElementById("players");
var board = document.getElementById("board");
var game_state = document.getElementById("game-state");
var winner = document.getElementById("winner");
var end_game = false;

// render element content
players.textContent = game.render_players();
board.textContent = game.render_board();
game_state.textContent = game.render_game_state();

console.log('before loop');

// loop game until it reaches an end state
do {
  tick();

  // TODO: trying to set delay right now
  // setTimeout(() => { delay_message() }, delay);
  setTimeout(delay_message(), seconds * ms);
  end_game = game.end_game();
} while (!end_game);

game_over_msg();
console.log('resetting game');
game.reset();


function tick() {
  // Run the game for one "tick" or move 
  console.log('updating game');
  game.update();

  console.log('rendering board');
  board.textContent = game.render_board();
  console.log('rendering game state');
  game_state.textContent = game.render_game_state();
}

function delay_message() {
  console.log("delay for time ticks");
}

function game_over_msg() {
  const game_over = game.game_over();
  if (game_over == draw) {
    winner.textContent = game.declare_draw();  
  } else {
    winner.textContent = game.declare_winner();  
  }
}