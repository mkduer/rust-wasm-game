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

#[wasm_bindgen]
pub fn game_name() -> String {
    "Tic Tac Toe".to_string()
}

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

#[wasm_bindgen]
pub fn mutate(a: u32, b: u32) -> u32 {
    let x = a + 3;
    let y = b + 1;
    add(x, y)
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty = 0,
    P1 = 1,
    P2 = 1,
}