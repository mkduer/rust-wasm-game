##  Rust Compiled to WASM

This version of the Rust tic-tac-toe game will be compiled over to WebAssembly (WASM). 

Compiling Rust over to WASM was great fun once the compiliation was successful. I did run into obstacles due to versioning, changing compatible tools, and limited, working examples outside of the Rust manuals for wasm and wasm-bindgen for more modern versions. Most hurdles I ran into involved in-progress `wasm-bindgen` features, which were resolved by reverting back to the versions used in the [Rust-WASM tutorial](https://rustwasm.github.io/docs/book/introduction.html) for creating Conway's Game of Life. This is a fantastic tutorial and highly recommended. I suspect that the tools will become more stable and reliable over time.

## Build 

This project was built with Ubuntu 18.04 using a Firefox browser. Please use the listed versions for Rust dependencies (`Cargo.toml`) and javascript packages (`pkg/package.json`), otherwise compiler warnings and failed builds will likely occur. Following the provided build steps will help in getting the project built correctly. However, due to the changing tools and the newness of WASM, this program cannot be guaranteed to work with different environments.

Clone or fork the repo: `git clone git@github.com:mkduer/rust-wasm-game.git`

Go into the directory **wasm-tic-tac-toe**, compile Rust into WebAssembly:  
  `cd wasm-tic-tac-toe`
  `wasm-pack build`

Build dependencies and start the web server with the following sequence of commands:  
  `cd dist`  
  `npm install`  
  `npm start`  

## Current Work and Next Steps

- Implementing the components for manual play in the web browser.
- Alter design from the initial command-line implementation to a more intuitive browser design allowing for mouse-clicks
  
## Resources

The following resources were the most helpful in learning more about Rust + WASM:
*  [Tutorial: Conway's Game of Life](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)
*  [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
*  [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html)

##### Copyright (c) 2019 Michelle Duer

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

