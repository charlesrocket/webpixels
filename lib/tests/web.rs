#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

pub mod files;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn valid_image() {
    let options = webpixels::Options::default();
    webpixels::pixelmosh(&files::IMAGE.to_vec(), &options).unwrap();
}
