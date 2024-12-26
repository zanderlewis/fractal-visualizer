use rayon::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
struct Complex {
    re: f64,
    im: f64,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum FractalType {
    Mandelbrot,
    Julia,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct JuliaParams {
    c_re: f64,
    c_im: f64,
}

#[wasm_bindgen]
pub struct ViewState {
    width: u32,
    height: u32,
    zoom: f64,
    offset_x: f64,
    offset_y: f64,
    buffer: Vec<u8>,
    fractal_type: FractalType,
    julia_params: Option<JuliaParams>
}

#[wasm_bindgen]
impl ViewState {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, fractal_type: FractalType) -> Self {
        Self {
            width,
            height,
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            buffer: vec![0; (width * height * 4) as usize],
            fractal_type,
            julia_params: Some(JuliaParams { c_re: -0.7, c_im: 0.27015 })
        }
    }

    pub fn zoom_at(&mut self, x: u32, y: u32, factor: f64) {
        let before = self.map_to_complex(x, y);
        self.zoom *= factor;
        let after = self.map_to_complex(x, y);
        self.offset_x += before.re - after.re;
        self.offset_y += before.im - after.im;
    }

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

    pub fn draw(&mut self, max_iterations: u32, escape_radius: f64) -> Vec<u8> {
        let width = self.width;
        let height = self.height;
        let zoom = self.zoom;
        let offset_x = self.offset_x;
        let offset_y = self.offset_y;
        let fractal_type = self.fractal_type;
        let julia_params = self.julia_params.clone();
    
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
                    let (mut z_re, mut z_im, c_re, c_im) = match fractal_type {
                        FractalType::Mandelbrot => (0.0, 0.0, re, im),
                        FractalType::Julia => {
                            let params = julia_params.as_ref().unwrap();
                            (re, im, params.c_re, params.c_im)
                        },
                    };
    
                    let mut z_re2 = z_re * z_re;
                    let mut z_im2 = z_im * z_im;
                    let mut iter = 0;
    
                    while iter < max_iterations && z_re2 + z_im2 <= escape_radius {
                        z_im = 2.0 * z_re * z_im + c_im;
                        z_re = z_re2 - z_im2 + c_re;
                        z_re2 = z_re * z_re;
                        z_im2 = z_im * z_im;
                        iter += 1;
                    }
    
                    let color = if iter == max_iterations {
                        [0, 0, 0, 255]
                    } else {
                        let smooth_iter = iter as f64 + 1.0 - (z_re2 + z_im2).ln().ln() / 2.0_f64.ln();
                        let hue = (360.0 * smooth_iter / 50.0) % 360.0; // Normalize to a fixed base of 50 iterations
                        let s = 0.8;
                        let v = if iter < max_iterations { 1.0 } else { 0.0 };
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
    let h = h / 60.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as i32 {
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