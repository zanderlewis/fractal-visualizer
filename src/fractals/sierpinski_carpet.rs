use crate::Complex;
use super::Fractal;

/// Struct representing the Sierpinski Carpet fractal.
pub struct SierpinskiCarpet;

/// Implementation of the `Fractal` trait for SierpinskiCarpet.
impl Fractal for SierpinskiCarpet {
    /// Calculates the iteration count for a given point `c`.
    ///
    /// Determines if the point `c` is part of the Sierpinski Carpet.
    /// A point is not part of the fractal if, in any base-3 digit,
    /// both the x and y coordinates have a digit of 1.
    fn calculate(&self, c: Complex, max_iterations: u32, _escape_radius: f64) -> u32 {
        let mut x = c.re;
        let mut y = c.im;
        let mut iter = 0;

        while iter < max_iterations {
            // Convert to base 3 by multiplying by 3 and truncating
            x *= 3.0;
            y *= 3.0;

            // Extract the fractional part
            let digit_x = x.trunc() as u32 % 3;
            let digit_y = y.trunc() as u32 % 3;

            // Check if the point is in the central square
            if digit_x == 1 && digit_y == 1 {
                return iter;
            }

            iter += 1;
        }

        iter
    }
}