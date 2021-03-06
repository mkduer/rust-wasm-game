# rust-wasm-game

This project has two versions of tic-tac-toe in Rust as summarized below. The specifications can be found in the `specs_whitepaper.pdf`. The file structure is organized as follows:

```
        LICENSE
        README.md
        specs_whitepaper.pdf
	tic-tac-toe
	     |_ README.md
	     |_ Cargo.toml
	     |_ src
	            |_ main.rs (unit tests are in here)
	wasm-tic-tac-toe
   	     |_ README.md
	     |_ Cargo.toml
	     |_ src
	            |_ lib.rs
	            |_ utils.rs
             |_ dist
	            |_ bootstrap.js
	            |_ index.css
	            |_ index.html
	            |_ index.js
	            |_ package.json
	            |_ package-lock.json
	            |_ webpack.config.js
             |_ tests
	            |_ web.rs
```

### tic-tac-toe

This implementation creates a command-line version of the game allowing for manual and/or automatic play by two players. The instructions for building, running, and playing the game are detailed in the **tic-tac-toe** directory's [README.md](https://github.com/mkduer/rust-wasm-game/tree/master/tic-tac-toe)



### wasm-tic-tac-toe

This version takes a similar implementation as the previous in terms of game play. However, this Rust code is compiled to WebAssembly (WASM) using `wasm-bindgen`, `wasm-pack` and other tools in order to make the game playable through the browser. 

## <a href="https://youtu.be/8lt9jbfSNY0">
Click Image to Watch Youtube Demo:
</a>  


<a href="https://youtu.be/8lt9jbfSNY0">
<img src="extra/wasm-o-wins.png" 
alt="Rust-WASM autoplay demo" width="400" height="400"/></a>

WebAssembly is still a new-ish technology that was first announced in 2015. This assembly-like language allows for languages like Rust/C/C++ to be compiled with their memory-saving, efficiency, and possibly increased security feautres (in the case of Rust) that can be intertwined with javascript in the browser. 

Please use the listed versions for Rust dependencies and javascript packages, otherwise compiler warnings and failed builds will likely occur. More details about building this project and personal experience while working with Rust -> WASM can be found in the project directory's [README.md](https://github.com/mkduer/rust-wasm-game/tree/master/wasm-tic-tac-toe).

##### Copyright (c) 2019 Michelle Duer

   Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
   
       http://www.apache.org/licenses/LICENSE-2.0
       
   Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
