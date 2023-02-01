use wasm_bindgen::prelude::*;

use libmosh::MoshCore;

pub mod utils;

#[wasm_bindgen]
#[derive(Default)]
pub struct Core(MoshCore);

#[wasm_bindgen]
impl Core {
    pub fn min_rate(&self) -> u16 {
        self.0.options.min_rate
    }

    pub fn max_rate(&self) -> u16 {
        self.0.options.max_rate
    }

    pub fn pixelation(&self) -> u8 {
        self.0.options.pixelation
    }

    pub fn line_shift(&self) -> f64 {
        self.0.options.line_shift
    }

    pub fn reverse(&self) -> f64 {
        self.0.options.reverse
    }

    pub fn flip(&self) -> f64 {
        self.0.options.flip
    }

    pub fn channel_swap(&self) -> f64 {
        self.0.options.channel_swap
    }

    pub fn channel_shift(&self) -> f64 {
        self.0.options.channel_shift
    }

    pub fn seed(&self) -> u64 {
        self.0.options.seed
    }

    pub fn set_min_rate(&mut self, value: u16) {
        self.0.options.min_rate = value;
    }

    pub fn set_max_rate(&mut self, value: u16) {
        self.0.options.max_rate = value;
    }

    pub fn set_pixelation(&mut self, value: u8) {
        self.0.options.pixelation = value;
    }

    pub fn set_line_shift(&mut self, value: f64) {
        self.0.options.line_shift = value;
    }

    pub fn set_reverse(&mut self, value: f64) {
        self.0.options.reverse = value;
    }

    pub fn set_flip(&mut self, value: f64) {
        self.0.options.flip = value;
    }

    pub fn set_channel_swap(&mut self, value: f64) {
        self.0.options.channel_swap = value;
    }

    pub fn set_channel_shift(&mut self, value: f64) {
        self.0.options.channel_shift = value;
    }

    pub fn new_seed(&mut self) {
        self.0.options.new_seed();
    }

    /// Processes provided image data
    ///
    /// # Errors
    /// TODO
    #[wasm_bindgen]
    pub fn pixelmosh(&mut self, image: &[u8]) -> Result<Vec<u8>, JsValue> {
        let mut output: Vec<u8> = Vec::new();

        self.0
            .read_image(image)
            .map_err(|error| JsValue::from(error.to_string()))?;

        self.0
            .mosh()
            .map_err(|error| JsValue::from(error.to_string()))?;

        {
            let mut encoder = png::Encoder::new(&mut output, self.0.data.width, self.0.data.height);

            encoder.set_color(self.0.data.color_type);
            encoder.set_depth(self.0.data.bit_depth);

            let mut writer = encoder
                .write_header()
                .map_err(|error| JsValue::from(error.to_string()))?;

            writer
                .write_image_data(&self.0.data.buf)
                .map_err(|error| JsValue::from(error.to_string()))?;
        }

        Ok(output)
    }
}
