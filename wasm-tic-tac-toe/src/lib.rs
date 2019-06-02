/*
*  reference for starting code:
*  https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
*/

mod utils;

use wasm_bindgen::prelude::*;

use std::fmt;
use rand::{thread_rng, Rng};
use std::io::{stdin, stdout, Write};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO: remove this test code
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// TODO: remove this test code
#[wasm_bindgen]
pub fn greet() {
    alert("Testing 1, 2, 3");
}