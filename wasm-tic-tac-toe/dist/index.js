/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

const game = Game.new();

const name = document.getElementById("game-name");
const pre = document.getElementById("tic-tac-toe-canvas");
const player_status = document.getElementById("player-status");
const game_state = document.getElementById("game-state");

game.start(true, true)

name.textContent = game.title();
player_status.textContent = game.render_player_status();
game_state.textContent = game.render_game_state();

/*
const renderLoop = () => {
  pre.textContent = game... // TODO
  requestAnimationFrame(renderLoop);
}
*/
