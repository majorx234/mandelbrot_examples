use image::save_buffer_with_format;
use image::ColorType;
use image::ImageError;
use num::complex::Complex;
use std::fs::File;

fn complex_square_add_loop(c: Complex<f64>, limit: usize) -> Option<u8> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i as u8);
        }
        z = z * z + c
    }
    None
}

fn calculate_mandelbrot(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    resolution_x: usize,
    resolution_y: usize,
) -> Vec<u8> {
    let mut rows: Vec<u8> = vec![0; resolution_y * resolution_x];
    for py in 0..resolution_y {
        let y = (py as f64 / ((resolution_y - 1) as f64)) * (y_max - y_min) + y_min;
        for px in 0..resolution_x {
            let x = (px as f64 / (resolution_x - 1) as f64) * (x_max - x_min) + x_min;
            let xy_escape = complex_square_add_loop(Complex::new(x, y), 255);
            match xy_escape {
                Some(xy_escape) => rows[py * resolution_x + px] = xy_escape,
                None => rows[py * resolution_x + px] = 0,
            }
        }
    }
    return rows;
}

fn write_png(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), ImageError> {
    image::save_buffer_with_format(
        filename,
        pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::L8,
        //image::ColorType::Rgb8
        image::ImageFormat::Png,
    )?;
    Ok(())
}

fn main() {
    let my_mandelbrot = calculate_mandelbrot(-1.5, 0.5, -1.0, 1.0, 1000, 1000);
    write_png("test.png", &my_mandelbrot, (1000, 1000));
}
