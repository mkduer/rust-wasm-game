##  Rust Compiled to WASM

This version of the Rust tic-tac-toe game will be compiled over to WebAssembly (WASM). 

## Build 

Clone or fork the repo: `git clone git@github.com:mkduer/rust-wasm-game.git`

Go into the directory **wasm-tic-tac-toe**, compile Rust into WebAssembly:  
  `wasm-pack build`

Build dependencies and start the web server with the following sequence of commands:  
  `cd dist`  
  `npm install`  
  `npm start`  

## Next Steps...
- Compile to WASM (wip) with a cli-like appearance for proof-of-concept browser game
- Try another implementation with `serde`
- Try to get WASM and React.js working together
  
## Resources

The following resources were the most helpful in learning more about Rust + WASM:
*  [Tutorial: Conway's Game of Life](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)
*  [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
*  [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html)

Some of the main challenges were finding recent articles and examples specifically using the specific set of languages I wanted to learn: Rust, WASM and React.js. There were many issues trying to build with outdated tool versions, limited error handling in WASM, and much of this project demonstrates learning based on trial + test experimentation.


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

