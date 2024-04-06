mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Canvas {
    width: u32,
    height: u32,
    r: Vec<u8>,
    g: Vec<u8>,
    b: Vec<u8>,
}

#[wasm_bindgen]
pub struct OffsetsRGB {
    pub r: *const u8,
    pub g: *const u8,
    pub b: *const u8,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new() -> Canvas {
        let w: u32 = 1024;
        let h: u32 = 720;
        let linear_size: usize = (w * h) as usize;
        Canvas {
            width: w,
            height: h,
            r: vec![255; linear_size],
            g: vec![244; linear_size],
            b: vec![0; linear_size],
        }
    }

    pub fn pixels(&self) -> OffsetsRGB {
        OffsetsRGB {
            r: self.r.as_ptr(),
            g: self.g.as_ptr(),
            b: self.b.as_ptr(),
        }
    }
}
