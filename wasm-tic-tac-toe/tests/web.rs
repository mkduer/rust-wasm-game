//! Test suite for the Web and headless browsers.

// source: sample test grabbed from https://rustwasm.github.io/wasm-pack/book/print.html
// as reference. These are to be replaced by custom tests relating to this specific project 

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
