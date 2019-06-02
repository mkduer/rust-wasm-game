/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

const game = Game.new();

const name = document.getElementById("game-name");
const pre = document.getElementById("tic-tac-toe-canvas");
const status = document.getElementById("player-status");

name.textContent = Game.title();
game.start(true, true)

status.textContent = game.render();

/*
const renderLoop = () => {
  pre.textContent = wasm.mutate(5, 2);
  requestAnimationFrame(renderLoop);
}
*/
