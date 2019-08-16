/*
*  reference for starting point: 
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

import { Game } from "../pkg/wasm_tic_tac_toe";

const DRAW = 9;
const MS = 1000;
const AUTO_DELAY = 0.5 * MS;
const MANUAL_DELAY = 1.0 * MS;

document.getElementById("title").textContent = "Rusty Tic Tac Toe\nMagically Compiled to WASM";
listen();

function listen() {
  let settings = {
    start_visible: document.getElementById("start-visible"),
    start_collapsed: document.getElementById("start-collapsed"),
    players: document.getElementById("players"),
    board: document.getElementById("board"),
    trans_board: document.getElementById("transparent-board"),
    winner: document.getElementById("winner"),
    reset_btn: document.getElementById("reset"),
    player1_type: true,
    player2_type: true,
    manual: false,
    reset: false
  }

  // listen for user selection of automatic/manual play
  let game = Game.new();

  // setup game play style by waiting for the user
  // to select automatic or manual play
  let auto_play = document.getElementById("auto");
  let manual_play = document.getElementById("manual");

  // settings for automatic play button
  auto_play.onclick = (e) => {
    e.preventDefault()
    settings.player1_type = true;
    settings.player2_type = true;
    settings.manual = false;
    game.start(settings.player1_type, settings.player2_type)
    begin(game, settings)
  };

  // settings for manual play button
  manual_play.onclick = () => {
    settings.player1_type = false;
    settings.player2_type = true;
    settings.manual = true;
    game.start(settings.player1_type, settings.player2_type)
    begin(game, settings)
  };
}

function begin(game, settings) {
  // begins the game by rendering it and playing the game ticks
  let manual_dialogue = document.getElementById("manual-dialogue");

  // display menu according to manual/automatic play
  if (settings.manual) {
    settings.winner.style.transform = 'translate(0%, -450%)';
    settings.players.style.transform = 'translate(0%, -190%)';
    settings.reset_btn.style.transform = 'translate(0%, -300%)';
  } else {
    settings.winner.style.transform = 'translate(0%, 0%)';
    settings.players.style.transform = 'translate(0%, 0%)';
    settings.reset_btn.style.transform = 'translate(0%, 0%)';
  }

  // collapse visible start menu, display collapsed menu
  settings.start_visible.style.visibility = "collapse";
  settings.start_collapsed.style.visibility = "visible";

  // render game
  render(game, settings);

  // initiate manual/automatic play
  if (settings.manual) {
    manual_play(game, manual_dialogue, settings);
  } else {
    auto_play(game, manual_dialogue, settings);
  }
}

function render(game, settings) {
  // render game content
  settings.players.textContent = game.render_players();
  settings.board.style.visibility = "visible";
  settings.board.textContent = game.render_board();

  if (settings.manual) {
    render_overlay(game, settings.trans_board)
  }
}

function render_overlay(game, trans_board) {
  // render transparent overlay with indexed board
  trans_board.textContent = game.render_indexed_board();
  trans_board.style.visibility = "visible";
}

async function manual_play(game, manual_dialogue, settings) {

    console.log('settings in manual_play()')
    console.log(settings)
  // start the game and loop until end game is reached
  let end_game = false; 
  let local_reset = false;
  manual_dialogue.style.visibility = "visible";

  // listen for reset
  settings.reset_btn.onclick = function() {
      local_reset = reset_all(game, manual_dialogue, settings);
  };
  document.getElementById("manual-dialogue").textContent = "Where do you want to place your piece?";

  // if reset button was not selected, continue game play
  while (!local_reset && !end_game) {
    await sleep(settings.manual);

    // listen for a valid keystroke by the user (or an `escape` equivalent)
    // 9 represents an invalid input value
    let success = manual_tick(game);
    end_game = game.get_end_game();

    // user selected `escape` so reset game
    if (success == -1) {
      reset_all(game, manual_dialogue, settings);
      local_reset = true;
    // automated player
    } else {
      auto_tick(game, settings.board);
      end_game = game.get_end_game();
    }

    // TODO: update rendering of indexed board (in Rust) so that successful cell placement removes numeral
    settings.board.textContent = game.render_board();
  }

  // select next function based on whether the game
  // stopped by reset or by reaching the end game
  if (local_reset) {
    return listen();
  } else {
    return game_over_msg(game, settings.winner);
  }
}

function manual_tick(game) {
  // Run the game for one "tick" or move 
  let success = 9;
  let key = -2;
  
  // following code source: https://medium.com/@uistephen/keyboardevent-key-for-cross-browser-key-press-check-61dbad0a067a
  // a good article on dealing with deprecated `keyCode`, which is still ubiquitous in browsers AND allowing
  // for the spec suggested `key` as well as why to use `keyup`
  window.addEventListener('keyup', (e) => {
    if (e.defaultPrevented) {
      return;
    }
    key = e.key || e.keyCode;

    if (key >= "0" && key <= "8") {
      console.log('valid key = ' + key)
      return game.update(key);
    } else if (key == "Escape" || key == "esc" || key == "27") {
      console.log('escape key pressed, key = ' + key)
      return -1;
    }
    return success;
  });
}

async function auto_play(game, manual_dialogue, settings) {
  // start the game and loop until end game is reached

  let end_game = false; 
  let local_reset = false;

  // listen for reset
  settings.reset_btn.onclick = function() {
      local_reset = reset_all(game, manual_dialogue, settings);
  };
    
  do {
    await sleep(settings.manual);

    // if reset button was selected
    if (!local_reset) {
      auto_tick(game, settings.board);
    }

    // check for end game
    end_game = game.get_end_game();
  } while (!end_game && !local_reset);

  // select next function based on whether the game
  // stopped by reset or by reaching the end game
  if (local_reset) {
    return listen();
  } else {
    return game_over_msg(game, settings.winner);
  }
}

function auto_tick(game, board) {
  // Run the game for one "tick" or move 
  let success = game.update();
  if (success > -1 || success < 9) {
    board.textContent = game.render_board();
  } else {
    throw "auto_tick function failed to update board";
  }
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

function sleep(manual) {
  // sleep equivalent in javascript, source:
  // https://stackoverflow.com/questions/951021/what-is-the-javascript-version-of-sleep
  let delay = AUTO_DELAY;

  if (manual) {
    delay = MANUAL_DELAY;
  }
  return new Promise(resolve => setTimeout(resolve, delay));
}

function reset_all(game, manual_dialogue, settings) {
  // resets the game settings
  reset = true;
  game.reset();
  settings.start_visible.style.visibility = "visible";
  settings.start_collapsed.style.visibility = "collapse";
  manual_dialogue.style.visibility = "collapse";
  settings.board.style.visibility = "collapse";
  settings.winner.style.visibility = "collapse";
  settings.trans_board.style.visibility = "collapse";

  settings.players.textContent = "";
  settings.board.textContent = "";
  settings.winner.textContent = "";
  manual_dialogue.textContent = "";

  return reset;
}