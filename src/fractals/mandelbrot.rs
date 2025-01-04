use crate::Complex;
use super::Fractal;

pub struct Mandelbrot;

impl Fractal for Mandelbrot {
    fn calculate(&self, c: Complex, max_iterations: u32, escape_radius: f64) -> u32 {
        let mut z = Complex { re: 0.0, im: 0.0 };
        let mut z_re2 = z.re * z.re;
        let mut z_im2 = z.im * z.im;
        let mut iter = 0;

        while iter < max_iterations && z_re2 + z_im2 <= escape_radius {
            z.im = 2.0 * z.re * z.im + c.im;
            z.re = z_re2 - z_im2 + c.re;
            z_re2 = z.re * z.re;
            z_im2 = z.im * z.im;
            iter += 1;
        }

        iter
    }
}