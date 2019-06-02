/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import * as wasm from "../pkg/wasm_tic_tac_toe";

wasm.greet();

const name = document.getElementById("game-name");
const pre = document.getElementById("tic-tac-toe-canvas");

name.textContent = wasm.game_name();
pre.textContent = wasm.mutate(5, 2);




/*
import { Game } from "../pkg/wasm_tic_tac_toe";

const pre = document.getElementById("tic-tac-toe-canvas");
const game = Game.new();
game.start();
const renderLoop = () => {
  pre.textContent = wasm.mutate(5, 2);
  requestAnimationFrame(renderLoop);
}
*/
