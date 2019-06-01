/* 
*  reference: wasm-bindgen `Hello World`
*  https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html
*  importing from local package
*/

const rust = import('../pkg/wasm_tic_tac_toe');

rust
  .then(m => m.greet('👽'))
  .catch(console.error);