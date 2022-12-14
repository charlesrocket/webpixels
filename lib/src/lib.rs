use wasm_bindgen::prelude::*;

use libmosh::{mosh, MoshOptions};

pub mod utils;

#[wasm_bindgen]
pub struct Options(MoshOptions);

#[wasm_bindgen]
impl Options {
    pub fn default() -> Self {
        Self(MoshOptions::default())
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

    pub fn line_shift(&self) -> f64 {
        self.0.line_shift
    }

    pub fn reverse(&self) -> f64 {
        self.0.reverse
    }

    pub fn flip(&self) -> f64 {
        self.0.flip
    }

    pub fn channel_swap(&self) -> f64 {
        self.0.channel_swap
    }

    pub fn channel_shift(&self) -> f64 {
        self.0.channel_shift
    }

    pub fn seed(&self) -> u64 {
        self.0.seed
    }

    pub fn set_min_rate(&mut self, value: u16) {
        self.0.min_rate = value;
    }

    pub fn set_max_rate(&mut self, value: u16) {
        self.0.max_rate = value;
    }

    pub fn set_pixelation(&mut self, value: u8) {
        self.0.pixelation = value;
    }

    pub fn set_line_shift(&mut self, value: f64) {
        self.0.line_shift = value;
    }

    pub fn set_reverse(&mut self, value: f64) {
        self.0.reverse = value;
    }

    pub fn set_flip(&mut self, value: f64) {
        self.0.flip = value;
    }

    pub fn set_channel_swap(&mut self, value: f64) {
        self.0.channel_swap = value;
    }

    pub fn set_channel_shift(&mut self, value: f64) {
        self.0.channel_shift = value;
    }

    pub fn set_seed(&mut self, value: u64) {
        self.0.seed = value;
    }
}

/// Processes provided image data
///
/// # Errors
/// TODO
#[wasm_bindgen]
pub fn pixelmosh(image: &[u8], options: &Options) -> Result<Vec<u8>, JsValue> {
    let decoder = png::Decoder::new(image);
    let mut output: Vec<u8> = Vec::new();
    let mut reader = decoder
        .read_info()
        .map_err(|error| JsValue::from(error.to_string()))?;

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .map_err(|error| JsValue::from(error.to_string()))?;

    mosh(&info, &mut buf, &options.0).map_err(|error| JsValue::from(error.to_string()))?;

    {
        let mut encoder = png::Encoder::new(&mut output, info.width, info.height);

        encoder.set_color(info.color_type);
        encoder.set_depth(info.bit_depth);

        let mut writer = encoder
            .write_header()
            .map_err(|error| JsValue::from(error.to_string()))?;

        writer
            .write_image_data(&buf)
            .map_err(|error| JsValue::from(error.to_string()))?;
    }

    Ok(output)
}
