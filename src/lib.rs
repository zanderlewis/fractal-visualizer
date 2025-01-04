use rayon::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

mod fractals;
use fractals::{Fractal, FractalType, create_fractal};

#[wasm_bindgen]
pub struct ViewState {
    width: u32,
    height: u32,
    zoom: f64,
    offset_x: f64,
    offset_y: f64,
    buffer: Vec<u8>,
    fractal: Box<dyn Fractal>,
}

#[wasm_bindgen]
impl ViewState {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, fractal_type: FractalType) -> Self {
        let fractal = create_fractal(fractal_type);
        Self {
            width,
            height,
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            buffer: vec![0; (width * height * 4) as usize],
            fractal,
        }
    }

    #[wasm_bindgen]
    pub fn zoom_at(&mut self, x: u32, y: u32, factor: f64) {
        let before = self.map_to_complex(x, y);
        self.zoom *= factor;
        let after = self.map_to_complex(x, y);
        self.offset_x += before.re - after.re;
        self.offset_y += before.im - after.im;
    }

    #[wasm_bindgen]
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.offset_x += dx * self.zoom;
        self.offset_y += dy * self.zoom;
    }

    fn map_to_complex(&self, x: u32, y: u32) -> Complex {
        Complex {
            re: (x as f64 - self.width as f64 / 2.0) / (self.width as f64 / 4.0) * self.zoom + self.offset_x,
            im: (y as f64 - self.height as f64 / 2.0) / (self.height as f64 / 4.0) * self.zoom + self.offset_y,
        }
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, max_iterations: u32, escape_radius: f64) -> Vec<u8> {
        let width = self.width;
        let height = self.height;
        let zoom = self.zoom;
        let offset_x = self.offset_x;
        let offset_y = self.offset_y;

        let buffer = &mut self.buffer;
        buffer.par_chunks_mut(width as usize * 4)
            .enumerate()
            .for_each(|(y, row)| {
                let y = y as u32;
                for x in 0..width {
                    let Complex { re, im } = Complex {
                        re: (x as f64 - width as f64 / 2.0) / (width as f64 / 4.0) * zoom + offset_x,
                        im: (y as f64 - height as f64 / 2.0) / (height as f64 / 4.0) * zoom + offset_y,
                    };
                    let iter = self.fractal.calculate(Complex { re, im }, max_iterations, escape_radius);

                    let color = if iter == max_iterations {
                        [0, 0, 0, 255]
                    } else {
                        let mag = (re * re + im * im).sqrt().max(1.0);
                        let smooth_iter = iter as f64 + 1.0 - (mag.ln() / std::f64::consts::LN_2);
                        let hue = (360.0 * smooth_iter / max_iterations as f64) % 360.0;
                        let s = 0.8;
                        let v = 0.9;
                        let rgb = hsv_to_rgb(hue, s, v);
                        [rgb.0, rgb.1, rgb.2, 255]
                    };

                    let index = (x as usize) * 4;
                    row[index..index + 4].copy_from_slice(&color);
                }
            });

        self.buffer.clone()
    }
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let hh = h / 60.0;
    let x = c * (1.0 - ((hh % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match hh as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}