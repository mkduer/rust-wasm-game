/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";


const title = document.getElementById("title");

var pre = document.getElementById("tic-tac-toe-canvas");
var players = document.getElementById("players");
var game_state = document.getElementById("game-state");
var board = document.getElementById("board");

players.setAttribute('style', 'white-space: pre;');
game_state.setAttribute('style', 'white-space: pre;');
board.setAttribute('style', 'white-space: pre;');

var game = Game.new();
game.start(true, true)
title.textContent = game.title();
players.textContent = game.render_players();
board.textContent = game.render_board();
game_state.textContent = game.render_game_state();

const renderLoop = () => {
  pre.textContent = game.update();
  game_state.textContent = game.render_game_state();
  const end_game = game.end_game;

  if (!end_game) {
    requestAnimationFrame(renderLoop);
  } else {
    game.reset();
  }
};
