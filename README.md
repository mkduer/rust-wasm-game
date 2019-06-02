# rust-wasm-game

This project has two versions of tic-tac-toe in Rust.

### tic-tac-toe

This implementation creates a command-line version of the game allowing for manual and/or automatic play by two players. The instructions for building, running, and playing the game are detailed in the **tic-tac-toe** directory's [README.md](https://github.com/mkduer/rust-wasm-game/tree/master/tic-tac-toe)



### wasm-tic-tac-toe

This version takes a similar implementation as the previous in terms of game play. However, this Rust code is compiled to WebAssembly (WASM) using `wasm-bindgen`, `wasm-pack` and other tools in order to make the game playable through the browser. 

WebAssembly is still a new-ish technology that was first announced in 2015. This assembly-like language allows for languages like Rust/C/C++ to be compiled with their memory-saving, efficiency, and possibly increased security (in the case of Rust) features in a language that can be intertwined with javascript in the browser. While there is a lot of anticipation for the benefits of WASM, there were a lot of unexpected bumps while implementing this project. 



Note: This version is still in-progress as of June 2, 2019.

This project was built with Ubuntu 18.04 using a Firefox browser. Due to in-progress `wasm-bindgen` features (such as imports for javascript), it is highly encouraged to use the listed versions for Rust dependencies and javascript packages, otherwise compiler warnings and failed builds can occur. The steps for doing so can be found in the project directory's [README.md](https://github.com/mkduer/rust-wasm-game/tree/master/wasm-tic-tac-toe).




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
