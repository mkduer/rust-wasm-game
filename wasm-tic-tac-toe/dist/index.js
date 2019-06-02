/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

const pre = document.getElementById("tic-tac-toe-canvas");
/*
const game = Game.new();
game.start();
game.display_indexed_board();

const renderLoop = () => {
  pre.textContent = game.render();
  game.update();

  requestAnimationFrame(renderLoop);
};

game.reset();
*/