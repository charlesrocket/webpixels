#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use webpixels::{pixelmosh, Options};

pub mod images;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn valid_image() {
    let options = Options::default();
    let result = pixelmosh(&images::VALID_IMAGE.to_vec(), &options);
    assert!(!result.is_err());
}

#[wasm_bindgen_test]
fn invalid_image() {
    let options = Options::default();
    let result = pixelmosh(&images::INVALID_IMAGE.to_vec(), &options);
    assert!(result.is_err());
}
