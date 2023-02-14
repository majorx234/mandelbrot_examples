use num::complex::Complex;

pub fn complex_square_add_loop(c: Complex<f64>, limit: usize) -> Option<u8> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i as u8);
        }
        z = z * z + c
    }
    None
}

pub fn calculate_mandelbrot(
    buffer: &mut [u8],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    resolution_x: usize,
    resolution_y: usize,
) {
    for py in 0..resolution_y {
        let y = (py as f64 / ((resolution_y - 1) as f64)) * (y_max - y_min) + y_min;
        for px in 0..resolution_x {
            let x = (px as f64 / (resolution_x - 1) as f64) * (x_max - x_min) + x_min;
            let xy_escape = complex_square_add_loop(Complex::new(x, y), 255);
            match xy_escape {
                Some(xy_escape) => buffer[py * resolution_x + px] = xy_escape,
                None => buffer[py * resolution_x + px] = 0,
            }
        }
    }
}
