use clap::Parser;
use mandelbrot_utils::calc::calculate_mandelbrot;
use mandelbrot_utils::image::write_png;
use std::time::SystemTime;

// mandelbrot caclulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// window size - how many samples to display
    #[arg(short, long, default_value_t = 1000)]
    pub xres: usize,

    /// how many samples to display in one line (max 1024)
    #[arg(short, long, default_value_t = 1000)]
    pub yres: usize,
}

fn main() {
    let args = Args::parse();
    let time_before = SystemTime::now();
    let mut my_mandelbrot: Vec<u8> = vec![0; args.xres * args.yres];
    calculate_mandelbrot(
        &mut my_mandelbrot,
        -1.5,
        0.5,
        -1.0,
        1.0,
        args.xres,
        args.yres,
    );
    let time_after = SystemTime::now();
    let time_difference = time_after.duration_since(time_before).unwrap().as_millis();
    print!("caclulation take {} ms", time_difference);
    write_png("test.png", &my_mandelbrot, (args.xres, args.yres)).unwrap();
}
