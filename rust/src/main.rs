use num::complex::Complex;

fn complex_square_add_loop(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
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
) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = Vec::new();
    for py in 0..resolution_y {
        let y = (py as f64 / ((resolution_y - 1) as f64)) * (y_max - y_min) + y_min;
        let mut column: Vec<usize> = Vec::new();
        for px in 0..resolution_x {
            let x = (px as f64 / (resolution_x - 1) as f64) * (x_max - x_min) + x_min;
            let xy_escape = complex_square_add_loop(Complex::new(x, y), 100);
            match xy_escape {
                Some(xy_escape) => column.push(xy_escape),
                None => column.push(0),
            }
        }
        rows.push(column);
    }
    return rows;
}

fn main() {
    let my_mandelbrot = calculate_mandelbrot(-2.0, 1.0, -1.0, 1.0, 10, 10);
    println!("{:?}", my_mandelbrot);
}
