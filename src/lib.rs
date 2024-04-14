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
