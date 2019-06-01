###  Rust Compiled to WASM

This version of the Rust tic-tac-toe game will be compiled over to WebAssembly (WASM). 

### In-Progress Steps 

From the directory **wasm-tic-tac-toe**, compile Rust into WebAssembly with the following command:  
  `wasm-pack build`

Build dependencies and start the web server:  
  `cd dist`  
  `npm install`  
  `npm start`  





### Resources

The following resources were the most helpful in learning more about Rust + WASM:
*  [Tutorial: Conway's Game of Life](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)
*  [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
*  [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html)

Some of the main challenges were finding recent articles and examples specifically using the specific set of languages I wanted to learn: Rust, WASM and React.js. There were many issues trying to build with outdated tool versions, limited error handling in WASM, and much of this project demonstrates learning based on trial + test experimentation.


