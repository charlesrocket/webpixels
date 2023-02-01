#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use webpixels::Core;

pub mod images;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn valid_image() {
    let mut core = Core::default();
    let result = core.pixelmosh(&images::VALID_IMAGE.to_vec());
    assert!(!result.is_err());
}

#[wasm_bindgen_test]
fn invalid_image() {
    let mut core = Core::default();
    let result = core.pixelmosh(&images::INVALID_IMAGE.to_vec());
    assert!(result.is_err());
}
