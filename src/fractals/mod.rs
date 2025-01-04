use crate::Complex;
use wasm_bindgen::prelude::*;

// Each fractal implementation is in its own file:
mod mandelbrot;
mod julia;
mod burning_ship;
mod sierpinski_carpet;

// Re-export the fractal implementations:
pub use mandelbrot::Mandelbrot;
pub use julia::{Julia, JuliaParams};
pub use burning_ship::BurningShip;
pub use sierpinski_carpet::SierpinskiCarpet;

/// Defines a generic fractal behavior.
pub trait Fractal: Sync {
    fn calculate(&self, c: Complex, max_iterations: u32, escape_radius: f64) -> u32;
}

/// Enum for which fractal to draw.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum FractalType {
    Mandelbrot,
    Julia,
    BurningShip,
    SierpinskiCarpet,
}

/// Helper function to pick the correct fractal based on FractalType.
pub fn create_fractal(fractal_type: FractalType) -> Box<dyn Fractal> {
    match fractal_type {
        FractalType::Mandelbrot => Box::new(Mandelbrot),
        FractalType::Julia => Box::new(Julia {
            params: JuliaParams {
                c_re: -0.7,
                c_im: 0.27015,
            },
        }),
        FractalType::BurningShip => Box::new(BurningShip),
        FractalType::SierpinskiCarpet => Box::new(SierpinskiCarpet),
    }
}