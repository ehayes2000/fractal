use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
mod utils;

const MAX_ITERATIONS: u32 = 400;
const MAX_F: f64 = 4.0;
// const SCALING_FACTOR: f64 = f64::MAX / 4.0;

use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    xmin: f64,
    xsize: f64,
    ymin: f64,
    ysize: f64,
    scaling_factor: f64,
) -> Result<(), JsValue> {
    let mandeldata = get_mandelbrot(width, height, xmin, xsize, ymin, ysize, scaling_factor);
    let mandeldata =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mandeldata), width, height)?;
    ctx.put_image_data(&mandeldata, 0.0, 0.0)
}

fn get_mandelbrot(
    w: u32,
    h: u32,
    xmin: f64,
    xsize: f64,
    ymin: f64,
    ysize: f64,
    scaling_factor: f64,
) -> Vec<u8> {
    let mut div_dist: Vec<u8> = Vec::new();
    let mut avg_it: u128 = 0;
    for y in 0..h {
        for x in 0..w {
            let x0: f64 = (x as f64 / w as f64) * xsize + xmin;
            let y0: f64 = (y as f64 / h as f64) * ysize + ymin;
            let mut x: f64 = 0.0;
            let mut y: f64 = 0.0;
            let mut iterations: u32 = 0;
            while x * x + y * y <= scaling_factor * scaling_factor * 4.0
                && iterations < MAX_ITERATIONS
            {
                let temp = (x * x) / scaling_factor - (y * y) / scaling_factor + x0;
                y = 2.0 * (x * y) / scaling_factor + y0;
                x = temp;
                iterations += 1;
            }
            avg_it += iterations as u128;
            div_dist.push((iterations / 4) as u8); // r
            div_dist.push((iterations / 2) as u8); // g
            div_dist.push(iterations as u8); // b
            div_dist.push(255); // a
        }
    }
    log!("average iterations: {}", avg_it / (h as u128 * w as u128));
    div_dist
}

// #[wasm_bindgen]
// pub struct Mandelbrot {
//     window_width: u32,
//     window_height: u32,
//     viewport_xmin: f64,
//     viewport_xsize: f64,
//     viewport_ymin: f64,
//     viewport_ysize: f64,
//     divergence: Vec<u16>,
// }

// #[wasm_bindgen]
// impl Mandelbrot {
//     pub fn new(width: u32, height: u32) -> Mandelbrot {
//         let standard_x_min: f32 = -2.5;
//         let standard_x_size: f32 = 3.5;
//         let px_mc = width as f32 / standard_x_size as f32;
//         let ysize = height as f32 / px_mc as f32;
//         let ymin = -ysize / 2.0;
//         log!("ymin, ysize {} {}", ymin, ysize);
//         Mandelbrot {
//             viewport_xmin: standard_x_min as f64,
//             viewport_xsize: standard_x_size as f64,
//             viewport_ymin: ymin as f64,
//             viewport_ysize: ysize as f64,
//             window_width: width,
//             window_height: height,
//             divergence: vec![70; (width * height) as usize],
//         }
//     }

//     pub fn center(&mut self, x: u32, y: u32) {
//         let view_x =
//             (x as f64 / self.window_width as f64) * self.viewport_xsize + self.viewport_xmin;
//         let view_y =
//             (y as f64 / self.window_height as f64) * self.viewport_ysize + self.viewport_ymin;
//         self.viewport_xmin = view_x - self.viewport_xsize / 2.0;
//         self.viewport_ymin = view_y - self.viewport_ysize / 2.0;
//     }

//     pub fn zoom(&mut self, zoom_factor: f64) {
//         let new_x_size = self.viewport_xsize * zoom_factor;
//         let new_y_size = self.viewport_ysize * zoom_factor;
//         // x_min = center - 1/2 new size
//         // center = x_min + 1/2 old size
//         self.viewport_xmin = self.viewport_xmin + (self.viewport_xsize / 2.0) - (new_x_size / 2.0);
//         self.viewport_ymin = self.viewport_ymin + (self.viewport_ysize / 2.0) - (new_y_size / 2.0);
//         self.viewport_xsize = new_x_size;
//         self.viewport_ysize = new_y_size;
//     }

//     pub fn get_divergence(&self) -> *const u16 {
//         self.divergence.as_ptr()
//     }
// }
