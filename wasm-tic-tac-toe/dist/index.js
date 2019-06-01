const wasm = import("./node_modules/@mkduer/wasm-tic-tac-toe/");

wasm.then(js => {
  wasm.greet();
});