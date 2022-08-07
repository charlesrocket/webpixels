mod utils;

use wasm_bindgen::prelude::*;

use libmosh::{mosh, Options as MoshOptions};

use crate::utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Options(MoshOptions);

#[wasm_bindgen]
impl Options {
    pub fn default() -> Options {
        Options(MoshOptions::default())
    }

    pub fn min_rate(&self) -> u16 {
        self.0.min_rate
    }

    pub fn max_rate(&self) -> u16 {
        self.0.max_rate
    }

    pub fn pixelation(&self) -> u8 {
        self.0.pixelation
    }

    pub fn line_shift_rng(&self) -> f64 {
        self.0.line_shift_rng
    }

    pub fn reverse_rng(&self) -> f64 {
        self.0.reverse_rng
    }

    pub fn flip_rng(&self) -> f64 {
        self.0.flip_rng
    }

    pub fn channel_swap_rng(&self) -> f64 {
        self.0.channel_swap_rng
    }

    pub fn channel_shift_rng(&self) -> f64 {
        self.0.channel_shift_rng
    }

    pub fn seed(&self) -> u64 {
        self.0.seed
    }

    pub fn set_min_rate(&mut self, value: u16) -> () {
        self.0.min_rate = value;
    }

    pub fn set_max_rate(&mut self, value: u16) -> () {
        self.0.max_rate = value;
    }

    pub fn set_pixelation(&mut self, value: u8) -> () {
        self.0.pixelation = value;
    }

    pub fn set_line_shift_rng(&mut self, value: f64) -> () {
        self.0.line_shift_rng = value;
    }

    pub fn set_reverse_rng(&mut self, value: f64) -> () {
        self.0.reverse_rng = value;
    }

    pub fn set_flip_rng(&mut self, value: f64) -> () {
        self.0.flip_rng = value;
    }

    pub fn set_channel_swap_rng(&mut self, value: f64) -> () {
        self.0.channel_swap_rng = value;
    }

    pub fn set_channel_shift_rng(&mut self, value: f64) -> () {
        self.0.channel_shift_rng = value;
    }

    pub fn set_seed(&mut self, value: u64) -> () {
        self.0.seed = value;
    }
}
