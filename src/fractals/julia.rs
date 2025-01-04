use crate::Complex;
use super::Fractal;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct JuliaParams {
    pub c_re: f64,
    pub c_im: f64,
}

#[wasm_bindgen]
pub struct Julia {
    pub params: JuliaParams,
}

impl Fractal for Julia {
    fn calculate(&self, mut z: Complex, max_iterations: u32, escape_radius: f64) -> u32 {
        let mut z_re2 = z.re * z.re;
        let mut z_im2 = z.im * z.im;
        let mut iter = 0;

        while iter < max_iterations && z_re2 + z_im2 <= escape_radius {
            z.im = 2.0 * z.re * z.im + self.params.c_im;
            z.re = z_re2 - z_im2 + self.params.c_re;
            z_re2 = z.re * z.re;
            z_im2 = z.im * z.im;
            iter += 1;
        }

        iter
    }
}